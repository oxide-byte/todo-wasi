use hyper::{Body, Request, Response};
use tokio_postgres::Client;
use crate::dao::todo_dao::Todo;

pub async fn update_todo(client: &Client, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("Update Todo...");

    let payload = hyper::body::to_bytes(req.into_body()).await?;
    let payload_string = String::from_utf8(payload.to_vec()).unwrap();

    let todo:Todo = serde_json::from_str(&*payload_string).unwrap();

    let result = client
        .query("update todo.todo \
         set owner=$2, name=$3, description=$4, status=$5, modified=(now() at time zone 'utc') \
         where id =$1 \
         RETURNING id, owner, name, description, status, created, modified",
               &[
                   &todo.id.unwrap(),
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