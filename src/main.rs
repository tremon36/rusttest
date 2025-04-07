use axum::{
    Router,
    response::Html,
    routing::get,
};
use db::Db;

mod db;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/helloworld", get(helloworld_handler));


    // Now connect to db

    let dbv: Db = Db::new().await;
    let data_q1 = dbv.query1().await;
    for e in data_q1 {
        println!("{e} ");
    }

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server running");
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> Html<String> {
    return Html::from(String::from("Hello world! html"));
}

async fn helloworld_handler() -> Html<String> {
    return Html::from(String::from("This is a generic response"));
}
