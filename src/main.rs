#[macro_use]
extern crate log;

#[macro_use]
extern crate serde;

mod config;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use failure::{Error, format_err};
use lazy_static::lazy_static;

use std::{collections::HashMap, env, fs::File, io::Read};

use config::*;

lazy_static! {
    pub static ref CONFIG: Config = {
        let f_name = env::var("UPGRENADE_SERVER_CONFIG").unwrap_or("upgrenade.toml".to_owned());
        let mut f = File::open(&f_name).unwrap_or_else(|e| {
            error!("Could not open config file {}: {}", f_name, e);
            panic!();
        });
        let mut s = String::new();

        f.read_to_string(&mut s).unwrap_or_else(|e| {
            error!("Could not read config file into string: {}", e);
            panic!();
        });

        //dbg!(s.parse::<toml::Value>().unwrap());

        toml::from_str(&s).unwrap_or_else(|e| {
            error!("Could not parse the config: {}", e);
            panic!();
        })
    };
}

#[get("/versions")]
async fn versions() -> Result<impl Responder, Error> {
    Ok(HttpResponse::Ok().json(
            CONFIG
            .version_map()?
            .iter()
            .map(|(k, v)| (format!("{}", k), v))
            .collect::<HashMap<_, _>>(),
            ))
}

#[get("/versions/latest")]
async fn versions_latest() -> Result<impl Responder, Error> {
    Ok(HttpResponse::Ok().json(
            CONFIG
            .version_map()?
            .iter()
            .last()
            .map(|(v, l)| (v.to_string(), l))
            .ok_or_else(|| format_err!("No versions to show"))?,
            ))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().unwrap_or("".into());
    env_logger::init();

    // Trigger Config parsing errors (if any) before running the server
    info!("Config:\n{}", toml::to_string(&*CONFIG).unwrap());

    HttpServer::new(|| App::new().service(versions).service(versions_latest))
        .bind("127.0.0.1:5000")?
        .run()
        .await
}
