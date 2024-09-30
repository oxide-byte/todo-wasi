# Todo WASI

## Introduction

This Repository is a little POC on experimenting with WebAssembly/WASI on Backend side. 

Due some great experiences with Leptos (WebAssembly/Frontend), this POC contains a Rest API Server based on minimal dependencies, storing data in a Postgres DB. The code itself does not contain a lot off error checks, validations or security gates.

The Docker image of the Web Server is based on *wasmedge/slim:0.13.5* with a total size of 64.26 MB.

The WebAssembly Server size build on release is 2 MB.

## Execution

The application can build and run inside Docker as Docker composition with:

````shell
docker compose up
````

In case you have a Rust development environment with cargo configured for build a target WASI:

````shell
cargo build --target wasm32-wasip1
````

or for the release mode:

````shell
cargo build --target wasm32-wasip1 --profile release
````

The Server itself can be started when you have installed wasmedge with:

````shell
chmod +x target/wasm32-wasip1/debug/todo-wasi.wasm
wasmedge target/wasm32-wasip1/debug/todo-wasi.wasm
````

In case you have IntelliJ, you can execute the HTTP requests found in the folder /http
like 

````http request
GET http://127.0.0.1:8080/api/todo
Accept: application/json
````