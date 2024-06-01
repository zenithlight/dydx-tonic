# dydx-tonic

`dydx-tonic` is a Rust library to interface with full nodes on dYdX Chain using gRPC. It uses `tonic` to build the dYdX protos and generate the client code.

## Installation

To generate the crate ensure [protoc](https://grpc.io/docs/protoc-installation/) is installed. Then `cargo run` and the client crate will be present in `/gen`.

To use `dydx-tonic` in your Rust project, add the path to the generated crate, along with its peer dependencies `tokio` and `tonic` to your `Cargo.toml` file:

```toml
[dependencies]
dydx-tonic = { path = "../dydx-tonic/gen" }
tokio = { version = "1.37.0", features = ["full"] }
tonic = "0.11.0"
```

## Usage

Here's a simple example of how to use dydx-tonic to interact with the dYdX protocol:

```rust
use dydx_tonic::dydxprotocol;
use tonic::transport::Endpoint;

const ENDPOINT: &str = "http://dydx-dao-grpc-1.polkachu.com:23890";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the dYdX node
    let channel = Endpoint::from_static(ENDPOINT).connect().await?;

    // Create gRPC client instances
    let mut asset_client = dydxprotocol::assets::query_client::QueryClient::new(channel.clone());
    let mut perpetual_client = dydxprotocol::perpetuals::query_client::QueryClient::new(channel);

    // Prepare the request
    let request = tonic::Request::new(dydxprotocol::perpetuals::QueryAllPerpetualsRequest {
        pagination: None,
    });

    // Make the gRPC call to retrieve all perpetuals
    let response = perpetual_client.all_perpetuals(request).await?;

    // Print the response
    println!("Response: {:?}", response);

    Ok(())
}
```