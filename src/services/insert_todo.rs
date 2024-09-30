use hyper::{Body, Request, Response};
use tokio_postgres::Client;
use crate::dao::todo_dao::Todo;

pub async fn insert_todo(client: &Client, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("Insert Todo...");

    let payload = hyper::body::to_bytes(req.into_body()).await?;
    let payload_string = String::from_utf8(payload.to_vec()).unwrap();

    let todo:Todo = serde_json::from_str(&*payload_string).unwrap();

    let result = client
        .query("insert into todo.todo(owner, name, description, status) \
        values($1,$2,$3,$4) \
        RETURNING id, owner, name, description, status, created, modified",
      &[
          &todo.owner.unwrap(),
          &todo.name.unwrap(),
          &todo.description.unwrap(),
          &todo.status.unwrap()
      ]
    ).await;

    let response = Todo::from_record(result.unwrap().iter().next().unwrap());

    Ok(Response::builder()
        .status(200)
        .header("Content-Type","application/json")
        .body(Body::from(serde_json::to_string(&response).unwrap()))
        .unwrap())
}