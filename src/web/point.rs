use super::builtin;
use axum::{*, response::Html};

pub async fn up() -> Html<&'static str>{
    builtin::log_sys("point up");
    let point = tokio::fs::read_to_string("point.txt").await.unwrap().parse::<isize>().unwrap();
    tokio::fs::write("point.txt", (point + 1).to_string()).await.unwrap();
    Html(r#"{"status": 200}"#)
}

pub async fn down() -> Html<&'static str>{
    builtin::log_sys("point down");
    let point = tokio::fs::read_to_string("point.txt").await.unwrap().parse::<isize>().unwrap();
    tokio::fs::write("point.txt", (point - 1).to_string()).await.unwrap();
    Html(r#"{"status": 200}"#)
}

pub async fn get() -> Html<String>{    
    let point = tokio::fs::read_to_string("point.txt").await.unwrap();
    Html(format!("{{\"status\": 200, \"point\":{point}}}"))
}