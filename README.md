# A gRPC example in Rust

This is a custom implementation of the gRPC Example for LetsGetRusty Link: https://github.com/letsgetrusty/grpc_example

This is a demo with features as below:
- Server support Payments and Membership.
- 2 clients; one for payments and one for membership.


Run gRPC server
```
cargo run --bin server
```

Run gRPC client for payments
```
cargo run --bin payments-client
```

Run gRPC client for members
```
cargo run --bin members-client
```