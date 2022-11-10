use actix_web::HttpResponse;
use mime;

use crate::config::Config;
use crate::htpasswd::Htpasswd;
use crate::assets;

#[derive(Debug, Clone)]
pub struct MyData {
    pub config: Config,
    pub htpasswd: Htpasswd,
}

pub async fn index() -> HttpResponse {
    if let Some(asset) = assets::get("static/index.html") {
        let msg = String::from_utf8(asset).unwrap();
        HttpResponse::Ok()
            .content_type(mime::TEXT_PLAIN)
            .body(msg)
    } else {
        HttpResponse::NotFound()
            .content_type(mime::TEXT_PLAIN)
            .body("Not Found!")
    }
}

pub async fn favicon() -> HttpResponse {
    let image_icon: mime::Mime = "image/x-icon".parse().unwrap();
    if let Some(asset) = assets::get("static/favicon.ico") {
        HttpResponse::Ok()
            .content_type(image_icon)
            .body(asset)
    } else {
        HttpResponse::NotFound()
            .content_type(image_icon)
            .body(vec![])
    }
}
