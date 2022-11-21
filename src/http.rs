use std::{env, net::SocketAddr};

use axum::{extract::Path, routing::get, Extension, Router};
use tokio::sync::mpsc::Sender;

pub async fn main_web(queue: Sender<String>) {
    let app = Router::new()
        .route("/notify/:message", get(notify))
        .layer(Extension(queue));

    let port = str::parse::<u16>(&env::var("WEB_PORT").expect("WEB_PORT undefined"))
        .expect("WEB_PORT not a u16");

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn notify(
    Extension(queue): Extension<Sender<String>>,
    Path(message): Path<String>,
) -> String {
    match queue.send(message.to_owned()).await {
        Ok(()) => "true".to_owned(),
        Err(_e) => "false".to_owned(),
    }
}
