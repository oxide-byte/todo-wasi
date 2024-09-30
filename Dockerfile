# BUILD BLOCK
#------------
FROM rustlang/rust:nightly-bullseye-slim as build

EXPOSE 8080

RUN cargo install cargo-wasi

ADD src ./src
ADD Cargo.toml .
ADD Cargo.toml .

RUN cargo wasi build --release

# RUN BLOCK
#----------
FROM wasmedge/slim:0.13.5
COPY --from=build target/wasm32-wasi/release/todo-wasi.wasm /todo-wasi.wasm
CMD ["wasmedge", "--env", "DB_HOST=postgres", "/todo-wasi.wasm"]