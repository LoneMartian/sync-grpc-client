use sync_grpc_client::{
    common::{
        greeter_client::GreeterClient, GoodbyeReply, GoodbyeRequest, HelloReply, HelloRequest,
    },
    implement_sync_grpc_client,
};

implement_sync_grpc_client!(
    GreeterClient,
    (Unary, say_hello, HelloRequest, HelloReply),
    (Unary, say_goodbye, GoodbyeRequest, GoodbyeReply)
);

fn main() {
    let test = String::from("http://127.0.0.1:50051");
    let client = SyncGreeterClient::new(test).expect("Client");
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
}
