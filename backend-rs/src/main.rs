use std::io::Read;

use actix_cors::Cors;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{post, App, Error, HttpResponse, HttpServer, Responder};

#[post("/health")]
async fn echo() -> impl Responder {
    HttpResponse::Ok().body("healthy")
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[post("/files")]
async fn save_files(
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, Error> {
    println!("{:?}", form);
    for mut f in form.files {
        let mut file_content = vec![];
        f.file.read_to_end(&mut file_content).unwrap();
        println!("{:?}", std::str::from_utf8(&file_content).unwrap())
    }

    Ok(HttpResponse::Ok())
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
