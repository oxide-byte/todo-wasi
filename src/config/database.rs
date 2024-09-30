use std::env;
use tokio_postgres::{Client, Config, Connection, NoTls, Socket};
use tokio_postgres::tls::NoTlsStream;

pub async fn connect() -> (Client, Connection<Socket, NoTlsStream>) {

    println!("initialize DB client/connection");

    let host = env::var("DB_HOST").unwrap_or("localhost".to_string());
    println!("Host definition: {:?}", host);

    let (client, connection) =
        Config::new()
            .host(&host)
            .port(5432)
            .dbname("todo")
            .keepalives(true)
            .user("todo")
            .password("password")
            .connect(NoTls)
            .await
            .expect("Unable to connect to DB");

    println!("DB client/connection created");

    (client, connection)
}