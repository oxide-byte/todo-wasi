use hyper::{Body, Request, Response};
use tokio_postgres::Client;

pub async fn delete_todo(client: &Client, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("Delete Todo...");

    let id = req.uri().path().split("/")
        .last().expect("missing ID to delete")
        .parse::<i32>().expect("error parsing id to int");

    let _ = client
        .execute("delete from todo.todo where id=$1", &[&id]).await;

    Ok(Response::new(Body::empty()))
}