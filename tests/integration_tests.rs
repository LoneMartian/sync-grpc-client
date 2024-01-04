use std::{
    thread::{sleep, JoinHandle},
    time::Duration,
};

use common::greeter_server::Greeter;
use sync_grpc_client::*;
use tokio_util::sync::CancellationToken;

use crate::common::{
    greeter_client::GreeterClient, greeter_server::GreeterServer, GoodbyeReply, GoodbyeRequest,
    HelloReply, HelloRequest,
};
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

implement_sync_grpc_client!(
    GreeterClient,
    (Unary, say_hello, HelloRequest, HelloReply),
    (Unary, say_goodbye, GoodbyeRequest, GoodbyeReply)
);

#[test]
fn test_happy_path() {
    // console_subscriber::init();
    let token: CancellationToken = CancellationToken::new();
    let address = String::from("127.0.0.1:50051");
    let test = String::from("http://127.0.0.1:50051");
    let handle = start_server(address.clone(), token.child_token()).expect("Server");
    let client = SyncGreeterClient::new(test).expect("Client");
    sleep(Duration::from_millis(500));
    let response = client
        .say_hello(HelloRequest { name: "foo".into() })
        .expect("Response");
    assert_eq!(
        response.into_inner(),
        HelloReply {
            message: "Hello foo!".into()
        }
    );
    let response = client
        .say_goodbye(GoodbyeRequest { name: "foo".into() })
        .expect("Response");
    assert_eq!(
        response.into_inner(),
        GoodbyeReply {
            message: "Goodbye foo!".into()
        }
    );
    sleep(Duration::from_secs(1));
    token.cancel();
    drop(client);
    println!("Joining thread");
    handle.join().expect("Thread");
}
