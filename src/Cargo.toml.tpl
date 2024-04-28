[package]
name = "dydx-tonic"
version = "0.1.0"
edition = "2021"

[lib]
name = "dydx_tonic"
path = "src/lib.rs"

[dependencies]
prost = "0.12.4"
prost-types = "0.12.4"
tonic = "0.11.0"