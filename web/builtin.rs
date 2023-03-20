use std::time::SystemTime;

pub fn log_sys(name: &str){
    println!("[log|{}]({:?})",
    name.replace(" ", "_") , SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap());
}

pub async fn read_html(name: &str) -> String{
    let header = tokio::fs::read_to_string(format!("layout/header.html")).await.unwrap_or(String::new());
    let html = tokio::fs::read_to_string(format!("front/{name}.html")).await.unwrap_or(String::new());
    let css = tokio::fs::read_to_string(format!("front/{name}.css")).await.unwrap_or(String::new());
    let js = tokio::fs::read_to_string(format!("front/{name}.js")).await.unwrap_or(String::new());
    let all_css = tokio::fs::read_to_string(format!("front/all.css")).await.unwrap_or(String::new());

    format!("{header}{html}<style>{css}\n{all_css}</style><script>{js}</script>")
}