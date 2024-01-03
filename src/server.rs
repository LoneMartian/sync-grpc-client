use std::{
    error::Error,
    thread::{sleep, JoinHandle},
    time::Duration,
};

use sync_grpc_client::common::{
    greeter_server::{Greeter, GreeterServer},
    GoodbyeReply, GoodbyeRequest, HelloReply, HelloRequest,
};
use tokio_util::sync::CancellationToken;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
pub struct DummyGreeter {}

#[tonic::async_trait]
impl Greeter for DummyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Received Hello: {:#?}", request);
        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }

    async fn say_goodbye(
        &self,
        request: Request<GoodbyeRequest>,
    ) -> Result<Response<GoodbyeReply>, Status> {
        let reply = GoodbyeReply {
            message: format!("Goodbye {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

fn start_server(
    addr: String,
    cloned_token: CancellationToken,
) -> Result<JoinHandle<()>, Box<dyn Error>> {
    let thread = std::thread::spawn(move || {
        let greeter = DummyGreeter::default();
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Runtime");
        let server_future = Server::builder()
            .add_service(GreeterServer::new(greeter))
            .serve_with_shutdown(addr.parse().unwrap(), cloned_token.cancelled());
        runtime
            .block_on(server_future)
            .expect("failed to successfully run the future on RunTime");
    });
    Ok(thread)
}

fn main() {
    let token: CancellationToken = CancellationToken::new();
    let address = String::from("127.0.0.1:50051");
    let handle = start_server(address, token.child_token());
    sleep(Duration::from_secs(15));
    token.cancel();
    if let Ok(handle) = handle {
        let _ = handle.join();
    }
}
