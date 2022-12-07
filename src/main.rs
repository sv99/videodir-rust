mod api;
mod app;
mod cli;
mod config;
mod htpasswd;
mod jwt;

use std::env;
use std::fs;
use std::path::Path;

// use env_logger::Env;
use simple_log::LogConfigBuilder;
#[macro_use]
extern crate simple_log;

use crate::config::Config;
use crate::htpasswd::Htpasswd;

// load binary data
#[macro_use]
extern crate bindata;
#[macro_use]
extern crate bindata_impl;

pub mod assets {
    bindata!("static");
}

#[actix_web::main]
async fn main() -> Result<(), String> {
    let work_dir = env::current_dir().unwrap();
    let work_dir = work_dir.as_path();

    // config
    let conf_path = work_dir.join(Path::new("videodir.conf"));
    let conf_src = fs::read_to_string(conf_path).unwrap();
    let conf = Config::load(&conf_src);

    // logger
    //env_logger::init_from_env(Env::default().default_filter_or("info"));
    let config = LogConfigBuilder::builder()
        .path(work_dir.join(Path::new("videodir.log")).to_str().unwrap())
        // .size(1 * 100)
        // .roll_count(10)
        .time_format("%Y-%m-%d %H:%M:%S.%f") //E.g:%H:%M:%S.%f
        .level(&conf.log_level)
        .output_file()
        .output_console()
        .build();

    simple_log::new(config)?;
    debug!("{:?}", work_dir);
    debug!("{:?}", &conf);

    // htpasswd
    let passwd_path = work_dir.join(Path::new("htpasswd"));
    let mut passwd= Htpasswd::load(&fs::read_to_string(&passwd_path).unwrap());
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
            passwd.write_to_path(&passwd_path).unwrap();
            return Ok(());
        }
        Some(("remove", sub_matches)) => {
            let username = sub_matches
                .get_one::<String>("username")
                .map(|s| s.as_str())
                .unwrap();
            passwd.remove(username);
            passwd.write_to_path(&passwd_path).unwrap();
            return Ok(());
        }
        _ => {
            // do nothing -> run http service
        }
    }

    let srv = app::start(conf, passwd);

    srv.await.map_err(|err| err.to_string())
}
