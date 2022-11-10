use std::env;
use std::fs;
use std::path::Path;

use actix_web::{web, App, HttpServer};

mod api;
mod app;
mod cli;
mod config;
mod htpasswd;

// load binary data
#[macro_use]
extern crate bindata;
#[macro_use]
extern crate bindata_impl;

pub mod assets {
    bindata!("static");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let work_dir = env::current_dir()?;
    let work_dir = work_dir.as_path();
    #[cfg(debug_assertions)]
    println!("{:?}", work_dir);

    // config
    let conf_path = work_dir.join(Path::new("videodir.conf"));
    let conf_src = fs::read_to_string(conf_path)?;
    let conf = config::Config::load(&conf_src);

    #[cfg(debug_assertions)]
    println!("{:?}", &conf);

    // htpasswd
    let passwd_path = work_dir.join(Path::new("htpasswd"));
    let mut passwd = htpasswd::Htpasswd::load(&fs::read_to_string(&passwd_path)?);
    // println!("{:?}",  &passwd);

    // cli
    let matches = cli::cli().get_matches();

    match matches.subcommand() {
        Some(("list", _sub_matches)) => {
            passwd.list();
            return Ok(());
        }
        Some(("add", sub_matches)) => {
            let username = sub_matches
                .get_one::<String>("username")
                .map(|s| s.as_str())
                .unwrap();
            let password = sub_matches
                .get_one::<String>("password")
                .map(|s| s.as_str())
                .unwrap();
            passwd.add(username, password);
            passwd.write_to_path(&passwd_path)?;
            return Ok(());
        }
        Some(("remove", sub_matches)) => {
            let username = sub_matches
                .get_one::<String>("username")
                .map(|s| s.as_str())
                .unwrap();
            passwd.remove(username);
            passwd.write_to_path(&passwd_path)?;
            return Ok(());
        }
        _ => {
            // do nothing -> run http service
        }
    }
    let my_data : app::MyData = app::MyData {
        config: conf.clone(),
        htpasswd: passwd.clone()
    };
    HttpServer::new(move || {
         App::new()
            .app_data(my_data.clone())
            .route("/", web::get().to(app::index))
            .route("/favicon.ico", web::get().to(app::favicon))
            .service(
                web::scope("/api/v1")
                    .configure(api::api_factory)
            )
    })
        .bind(&conf.server_addr)?
        .run()
        .await

}
