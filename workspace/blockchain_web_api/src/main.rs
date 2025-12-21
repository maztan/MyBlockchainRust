use std::{error::Error, fmt::Display, convert::Infallible};
use tokio;

use tower::{Service, Layer, ServiceBuilder};

/// Just "fun" example of middleware implementation using tower crate
#[tokio::main]
async fn main() {
    let service_stack = ServiceBuilder::new()
        .layer(Middleware1Layer)
        .service(MyService  { value: "default value".to_string() });

    // Services in the stack take a "&mut self", so if we want to execute stack
    // concurrently, we need to clone it (not required here)
    // Every part of stack (services, layers) have to implement clone for this to work
    let request_data = SomeData { value: "my request data1".to_string()};
    match service_stack.clone().call(request_data).await {
        Ok(response) => println!("{response}")
        // no error as it's Infallible
    }
}

#[derive(Debug, Clone)]
struct SomeData{
    value: String,
}

// MIDDLEWARE 1
#[derive(Clone)]
struct Middleware1<S>{
    inner: S,
    config_value1: String, // can be set via layer
}

impl<S> Service<SomeData> for Middleware1<S> 
where S: Service<SomeData>,{
    type Error = S::Error;
    type Response = S::Response;
    type Future = S::Future;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: SomeData) -> Self::Future {
        println!("Middleware1 received: {:?}", req);
        println!("Middleware1 config_value1 (can set via layer): {}\n", self.config_value1);
        self.inner.call(req)
    }
}

// SERVICE
#[derive(Clone)]
struct MyService{
    value: String,
}

impl Service<SomeData> for MyService {
    type Error = Infallible;
    type Response = String;
    type Future = std::future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: SomeData) -> Self::Future {
        let response = format!("MyService processed: {} with value: {}", req.value, self.value);
        std::future::ready(Ok(response))
    }
}

// LAYER
#[derive(Clone)]
struct Middleware1Layer;

impl<S> Layer<S> for Middleware1Layer {
    type Service = Middleware1<S>;

    fn layer(&self, inner: S) -> Self::Service {
        // Here you can pass configuration to the middleware
        Middleware1 {
            inner,
            config_value1: "Hey from MyServiceLayer!".to_string(),
        }
    }
}
