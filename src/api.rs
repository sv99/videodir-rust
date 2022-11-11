use std::path;
use std::io::{Error, ErrorKind};

use actix_web::{web, HttpResponse, Responder};
use actix_web::http::header::ContentType;
use serde::{Deserialize, Serialize};

use crate::config::Config;

pub fn api_factory(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
        web::resource("/version")
            .route(web::get().to(get_version)))
        .service(
            web::resource("/volumes")
                .route(web::get().to(get_volumes)))
        .service(
            web::resource("/list")
                .route(web::post().to(post_list)))
        .service(
            web::resource("/file")
                .route(web::post().to(post_file)))
        .service(
            web::resource("/filesize")
                .route(web::post().to(post_filesize)))
        .service(
            web::resource("/remove")
                .route(web::post().to(post_remove)));
}

#[derive(Serialize)]
struct Version {
    version: String,
}

#[derive(Serialize, Deserialize)]
pub struct Path {
    path: Vec<String>,
}

impl Path {

    fn join(&self) -> String {
        self.path.join(std::path::MAIN_SEPARATOR.to_string().as_str())
    }

    fn get_path(&self, volume: &String) -> String {
        format!("{}{}{}", volume, std::path::MAIN_SEPARATOR.to_string().as_str(), &self.join())
    }

}

async fn get_version() -> impl Responder {
    web::Json(Version { version: env!("CARGO_PKG_VERSION").to_string() })
}

async fn get_volumes(conf: web::Data<Config>) -> impl Responder {
    let res = serde_json::to_string(&conf.video_dirs).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(res)
}

async fn post_list(path: web::Json<Path>, conf: web::Data<Config>) -> impl Responder {
    let volumes = &conf.video_dirs;
    let mut res: Vec<String> = Vec::new();
    for v in volumes {
        let p_str = path.get_path(v);
        let p = path::Path::new(&p_str);
        if p.exists() {
            if p.is_dir() {
                let items = std::fs::read_dir(p);
                for item in items.unwrap() {
                    let item_path = item.unwrap().path();
                    let name = format!("{}", item_path.file_name().unwrap().to_str().unwrap());
                    res.push(name);
                }
            }
        }
    }
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&res).unwrap())
}

async fn post_file(path: web::Json<Path>, conf: web::Data<Config>) -> impl Responder {
    let volumes = &conf.video_dirs;
    // let path: &web::Json<Path> = req.app_data().unwrap();
    for v in volumes {
        let p_str = path.get_path(v);
        let p = path::Path::new(&p_str);
        if p.exists() {
            if p.is_file() {
                // return file
                return actix_files::NamedFile::open_async(p).await;
            }
        }
    }
    return Err(Error::new(ErrorKind::Other, "Not found"));
}

#[derive(Serialize)]
struct Filesize {
    size: u64,
}

async fn post_filesize(path: web::Json<Path>, conf: web::Data<Config>) -> impl Responder {
    let volumes = &conf.video_dirs;
    for v in volumes {
        let p_str = path.get_path(v);
        let p = path::Path::new(&p_str);
        if p.exists() {
            if p.is_file() {
                let s = Filesize { size: p.metadata().unwrap().len() };
                return HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .body(serde_json::to_string(&s).unwrap())
            }
        }
    }
    return HttpResponse::NotFound()
        .body("Not found");
}

#[derive(Serialize)]
struct Result {
    result: String,
}

async fn post_remove(path: web::Json<Path>, conf: web::Data<Config>) -> impl Responder {
    let volumes = &conf.video_dirs;
    let mut res_str = "Not found".to_string();
    for v in volumes {
        let p_str = path.get_path(v);
        let p = path::Path::new(&p_str);
        if p.exists() {
            if p.is_file() {
                match std::fs::remove_file(p) {
                    Ok(_) => res_str = "Ok".to_string(),
                    Err(e) => res_str = e.to_string()
                };
            }
            if p.is_dir() {
                match std::fs::remove_dir(p) {
                    Ok(_) => res_str = "Ok".to_string(),
                    Err(e) => res_str = e.to_string()
                };
            }
        }
    }
    let res = Result { result: res_str };
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&res).unwrap())
}
