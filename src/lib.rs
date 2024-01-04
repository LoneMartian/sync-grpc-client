pub use paste;
pub use std::error::Error;
pub mod common;

#[macro_export]
macro_rules! implement_service_method {
    (Unary, $client:ident, $function:ident, $req:ty, $res:ty) => {
        pub fn $function(
            &self,
            request: $req,
        ) -> Result<tonic::Response<$res>, Box<dyn $crate::Error>> {
            self.runtime.block_on(async {
                $client::connect(self.endpoint.clone())
                    .await?
                    .$function(request)
                    .await
                    .map_err(|status| status.into())
            })
        }
    };
    ($other:ident, $client:ident, $function:ident, $req:ty, $res:ty) => {
        compile_error!("This macro only supports clients with `Unary`-type methods");
    };
}

#[macro_export]
macro_rules! implement_sync_grpc_client {
    ($client:ident, $(($type:tt, $fn:ident, $req:ty, $res:ty)),*) => {
        paste::paste! {
            pub struct [<Sync$client>] {
                runtime: tokio::runtime::Runtime,
                endpoint: String
            }

            impl [<Sync$client>] {
                pub fn new(endpoint: String) -> Result<Self,Box<dyn $crate::Error>> {
                    let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_io()
                    .build()
                        ?;
                    Ok(Self {
                        runtime,
                        endpoint
                    })
                }

                $(
                    $crate::implement_service_method!{ $type, $client, $fn, $req, $res}
                )*
            }

            impl Drop for [<Sync$client>] {
                fn drop(&mut self) {
                    println!("Dropping the client");
                    self.runtime.block_on(async {});
                }
            }
        }
    };
}
