use rust_bert::pipelines::ner::NERModel;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static; 

struct NERModelWrapper {
    model: Arc<Mutex<NERModel>>,
}

lazy_static! {
    static ref NER_MODEL_WRAPPER: NERModelWrapper = NERModelWrapper {
    model: Arc::new(Mutex::new(NERModel::new(Default::default()).unwrap())),
};
}

async fn index() -> impl Responder {
    let html = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Redactr</title>
</head>
<body>
    <h1>Redactr</h1>
    <p>Redactr is a microservice that redacts personal identifiable information (PII) from text.</p>
    <p>It is a HTTP API that accepts a JSON payload with a text field and returns a JSON payload with a redacted_text field.</p>
    <p>It is built with Rust and Actix Web.</p>
    <p>Endpoints available:</p>
    <ul>
        <li>POST <a href="/redact">/redact</a></li>
        <li>GET <a href="/health">/health</a></li>
    </ul>
</body>
</html>"#;
    HttpResponse::Ok().body(html)
}

async fn detect(input_text: web::Json<String>) -> impl Responder {
    //    Set-up model
    let model_guard = NER_MODEL_WRAPPER.model.lock().unwrap();

    //let ner_model = NERModel::new(Default::default()).unwrap();

    //    Define input
    let input = [
        "My name is Amélie. I live in Москва.",
        "Chongqing is a city in China.",
        "Asked John Smith about Acme Corp",
    ];

    //    Run model
    let output = model_guard.predict_full_entities(&input);
    for entity in output {
        println!("{entity:?}");
    }


    let redacted_text = String::new();
    // Return the redacted text
    HttpResponse::Ok().body(redacted_text)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(move || {
        App::new()
        .service(web::resource("/").route(web::get().to(index)))
        .service(web::resource("/detect").route(web::post().to(detect)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

