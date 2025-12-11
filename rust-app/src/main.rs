async fn docs() -> impl Responder {
    let path = "/workspaces/aitour26-WRK541-real-world-code-migration-with-github-copilot-agent-mode/src/python-app/webapp/static/openapi.json";
    match std::fs::read_to_string(path) {
        Ok(content) => {
            match serde_json::from_str::<serde_json::Value>(&content) {
                Ok(json) => HttpResponse::Ok().json(json),
                Err(_) => HttpResponse::InternalServerError().body("Invalid OpenAPI JSON format")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("OpenAPI file not found")
    }
}
use actix_web::{get, web};
#[get("/countries/{country}")]
async fn country_cities(path: web::Path<String>) -> impl Responder {
    let data = fs::read_to_string("/workspaces/aitour26-WRK541-real-world-code-migration-with-github-copilot-agent-mode/src/python-app/webapp/weather.json").expect("Unable to read weather.json");
    let json: Value = serde_json::from_str(&data).expect("Invalid JSON");
    let country = path.into_inner();
    if let Some(cities) = json.get(&country).and_then(|c| c.as_object()) {
        let city_names: Vec<String> = cities.keys().cloned().collect();
        HttpResponse::Ok().json(city_names)
    } else {
        HttpResponse::NotFound().body("Country not found")
    }
}
// src/main.rs
use std::fs;
use actix_web::{App, HttpServer, Responder, HttpResponse};
use serde_json::Value;



async fn root() -> impl Responder {
    HttpResponse::Found()
        .append_header(("Location", "/docs"))
        .finish()
}

async fn countries() -> impl Responder {
    let data = fs::read_to_string("/workspaces/aitour26-WRK541-real-world-code-migration-with-github-copilot-agent-mode/src/python-app/webapp/weather.json").expect("Unable to read weather.json");
    let json: Value = serde_json::from_str(&data).expect("Invalid JSON");
    let countries: Vec<String> = json.as_object()
        .map(|obj| obj.keys().cloned().collect())
        .unwrap_or_default();
    HttpResponse::Ok().json(countries)
}

async fn health() -> impl Responder {
    "OK"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(root))
            .route("/health", web::get().to(health))
            .route("/countries", web::get().to(countries))
            .service(country_cities)
            .route("/docs", web::get().to(docs))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
