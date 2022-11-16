use std::{fs::File, io::BufReader};

use actix_web::{web, middleware::Logger, App, HttpResponse, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, rsa_private_keys};
use env_logger::Env;

use crate::config::Config;
use crate::htpasswd::Htpasswd;
use crate::{api, app, assets};
use crate::jwt::{bearer_jwt_validator, login};

pub async fn start(conf: Config, passwd: Htpasswd) -> std::io::Result<()> {

    let addr = conf.server_addr.clone();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    println!("starting HTTPS server at https://{addr}");

    let tls_config = load_rustls_config(&conf);

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(bearer_jwt_validator);

        App::new()
            .wrap(Logger::default())
            //.wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(conf.clone()))
            .app_data(web::Data::new(passwd.clone()))
            .route("/", web::get().to(app::index))
            .route("/favicon.ico", web::get().to(app::favicon))
            .route("/login", web::post().to(login))
            .service(
                web::scope("/api/v1")
                    .configure(api::api_factory)
                    .wrap(auth)
            )
    })
        .bind_rustls(&addr, tls_config)?
        .run()
        .await
}

async fn index() -> HttpResponse {
    if let Some(asset) = assets::get("static/index.html") {
        let msg = String::from_utf8(asset).unwrap();
        HttpResponse::Ok()
            .body(msg)
    } else {
        HttpResponse::NotFound()
            .body("Not Found!")
    }
}

async fn favicon() -> HttpResponse {
    if let Some(asset) = assets::get("static/favicon.ico") {
        HttpResponse::Ok()
            .content_type("image/x-icon")
            .body(asset)
    } else {
        HttpResponse::NotFound()
            .body("Not Found!")
    }
}

fn load_rustls_config(conf: &Config) -> rustls::ServerConfig {
    // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open(&conf.cert).unwrap());
    let key_file = &mut BufReader::new(File::open(&conf.key).unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = rsa_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate RSA private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}