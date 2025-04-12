use tokio_postgres::{Client, Error, NoTls};

pub async fn connect() -> Result<Client, Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=127.0.0.1 user=rust_user password=rust_password dbname=rust_database",
        NoTls,
    )
    .await?;

    // Spawn the connection handler in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Postgres connection error: {}", e);
        }
    });

    // Test query (optional — you can remove it if you just want the client)
    let rows = client.query("SELECT $1::TEXT", &[&"hello world"]).await?;
    let value: &str = rows[0].get(0);
    assert_eq!(value, "hello world");

    Ok(client)
}
