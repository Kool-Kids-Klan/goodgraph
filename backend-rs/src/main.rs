use std::collections::HashMap;
use std::io::{self, BufRead};
use std::sync::Mutex;

use actix_cors::Cors;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::http::header::ContentType;
use actix_web::web::Data;
use actix_web::{get, post, App, Error, HttpResponse, HttpServer, Responder};
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
struct Graph {
    graph_type: String,
    x: Vec<i32>,
    y: Vec<i32>,
}

#[derive(Serialize)]
struct Plot {
    graphs: Vec<Graph>,
}

#[post("/files")]
async fn save_files(
    loaded_files: Data<Mutex<HashMap<String, Vec<String>>>>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, Error> {
    let mut file_buffer;
    let file = form.files.first().unwrap();
    file_buffer = io::BufReader::new(&file.file).lines();

    let file_name;
    if let Some(name) = &file.file_name {
        file_name = name.clone();
    } else {
        file_name = "None".to_string();
    }
    let mut loaded_files = loaded_files.lock().unwrap();
    // instead of 'all' should be user id (I am thinking that with DB it will be useless)
    loaded_files
        .entry("all".to_string())
        .or_insert(vec![])
        .push(file_name);

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

    let bar_graph = Graph {
        graph_type: "bar".to_string(),
        x,
        y,
    };
    let response = serde_json::to_string(&Plot {
        graphs: vec![bar_graph],
    })
    .unwrap();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response))
}

#[derive(Debug, Serialize)]
struct SavedFilesResponse {
    files: Vec<String>,
}

#[get("/files")]
async fn get_loaded_files(
    loaded_files: Data<Mutex<HashMap<String, Vec<String>>>>,
) -> Result<impl Responder, Error> {
    let loaded_files = loaded_files.lock().unwrap();
    let response = serde_json::to_string(&SavedFilesResponse {
        files: loaded_files.get("all").unwrap().to_vec(),
    })
    .unwrap();

    return Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // very temporary design as server needs to distinguish between clients
    // and has to be stateless
    let loaded_files: Data<Mutex<HashMap<String, Vec<String>>>> =
        Data::new(Mutex::new(HashMap::new()));

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(Data::clone(&loaded_files))
            .service(echo)
            .service(save_files)
            .service(get_loaded_files)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
