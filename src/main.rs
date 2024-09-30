mod services;
mod dao;
mod config;

use std::net::SocketAddr;
use std::sync::{OnceLock};
use chrono::Utc;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response};
use tokio::net::TcpListener;
use tokio_postgres::{Client};
use crate::services::{get_todos, insert_todo, update_todo, delete_todo};
use crate::config::database::connect;

static CLIENT: OnceLock<Client> = OnceLock::new();

async fn route(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let db = CLIENT.get().expect("Get Client");
    match (req.method(), req.uri().path()) {

        (&Method::GET, "/api/todo") => get_todos(db).await,
        (&Method::POST, "/api/todo") => insert_todo(db, req).await,
        (&Method::PUT, "/api/todo") => update_todo(db, req).await,
        (&Method::DELETE, x) if x.starts_with("/api/todo/")=> delete_todo(db, req).await,

        _ => {
            Ok(Response::builder()
                .status(404)
                .body(Body::from("no resource found"))
                .unwrap()
            )
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    println!("VERSION: {:?}", Utc::now());
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let (client, connection) = connect().await;

    println!("open connection");
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    println!("share client");
    CLIENT.get_or_init(|| client);

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = Http::new()
                .serve_connection(stream, service_fn(route))
                .await {
                    println!("Error serving connection: {:?}", err);
                }
        });
    }
}