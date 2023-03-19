mod web;

use axum::{*, response::Html};
use web::{builtin, point};

#[tokio::main]
async fn main(){
    let app = axum::Router::new()
        .route("/", routing::get(index))
        .route("/get", routing::get(point::get))
        .route("/up", routing::post(point::up))
        .route("/down", routing::post(point::down))
    ;
    
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await.unwrap()
    ;
}

async fn index() -> Html<String>{    
    let point = tokio::fs::read_to_string("point.txt").await.unwrap();
    let html = builtin::read_html("index").await
        .replace("{{a}}", &point)
    ;
    Html(html)
}