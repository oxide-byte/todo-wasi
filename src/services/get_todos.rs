use hyper::{Body, Response};
use tokio_postgres::{Client};
use crate::dao::todo_dao::Todo;

pub async fn get_todos(client: &Client) -> Result<Response<Body>, hyper::Error> {
    println!("Get Todos...");

    let rows = client
        .query("select id, owner, name, description, status, created, modified from todo.todo", &[])
        .await;

    if rows.is_ok() {
        let mut todos:Vec<Todo> = Vec::new();
        for row in rows.unwrap().iter() {
            todos.push(Todo::from_record(row));
        }

        Ok(Response::builder()
               .status(200)
               .header("Content-Type","application/json")
               .body(Body::from(serde_json::to_string(&todos).unwrap()))
               .unwrap())
    } else {
        Ok(Response::builder()
               .status(404)
               .header("Content-Type","application/json")
               .body(Body::empty())
               .unwrap())
    }
}