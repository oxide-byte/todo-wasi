# BUILD BLOCK
#------------
FROM rustlang/rust:nightly-bullseye-slim AS build

EXPOSE 8080

RUN rustup target add wasm32-wasip1

ADD src ./src
ADD Cargo.toml .
ADD Cargo.lock .

RUN cargo build --target wasm32-wasip1 --profile release

# RUN BLOCK
#----------
FROM wasmedge/slim-runtime:0.13.5 AS run
COPY --from=build target/wasm32-wasip1/release/todo-wasi.wasm /todo-wasi.wasm
CMD ["wasmedge", "--env", "DB_HOST=postgres", "/todo-wasi.wasm"]