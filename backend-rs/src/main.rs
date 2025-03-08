use std::io::{self, BufRead};

use actix_cors::Cors;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::http::header::ContentType;
use actix_web::{post, App, Error, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[post("/health")]
async fn echo() -> impl Responder {
    HttpResponse::Ok().body("healthy")
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[derive(Serialize)]
struct BedgraphDataPoints {
    x: Vec<i32>,
    y: Vec<i32>,
}

#[post("/files")]
async fn save_files(
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, Error> {
    let mut file_buffer;
    let file = form.files.first().unwrap();
    file_buffer = io::BufReader::new(&file.file).lines();

    // skip frist 2 lines
    file_buffer.next();
    file_buffer.next();

    let mut x = vec![];
    let mut y = vec![];
    for line in file_buffer.map_while(Result::ok) {
        let mut parts = line.split(" ");
        x.push(parts.nth(1).unwrap().parse::<i32>().unwrap());
        y.push(parts.last().unwrap().parse::<i32>().unwrap());
    }

    let response = serde_json::to_string(&BedgraphDataPoints { x, y }).unwrap();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new().wrap(cors).service(echo).service(save_files)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
