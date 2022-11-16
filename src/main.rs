mod api;
mod app;
mod cli;
mod config;
mod htpasswd;
mod jwt;

use std::env;
use std::fs;
use std::path::Path;

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
async fn main() -> std::io::Result<()> {
    let work_dir = env::current_dir()?;
    let work_dir = work_dir.as_path();
    #[cfg(debug_assertions)]
    println!("{:?}", work_dir);

    // config
    let conf_path = work_dir.join(Path::new("videodir.conf"));
    let conf_src = fs::read_to_string(conf_path)?;
    let conf = Config::load(&conf_src);

    #[cfg(debug_assertions)]
    println!("{:?}", &conf);

    // htpasswd
    let passwd_path = work_dir.join(Path::new("htpasswd"));
    let mut passwd= Htpasswd::load(&fs::read_to_string(&passwd_path)?);
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

    app::start(conf, passwd).await

}
