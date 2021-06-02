mod utils;
mod config_parser;

use crate::utils::JwtHelper;
use crate::config_parser::ConfigSerde;
use hyper::server::conn::AddrStream;
use hyper::{Body, Request, Response, Server};
use hyper::service::{service_fn, make_service_fn};
use futures::future::{self, Future};
use std::env;
use log::{info, warn};
use serde_json::{Map, Value};
use std::str::FromStr;

fn main() {
    env_logger::init();
    env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY Symmetric is not set");
    env::var("CONFIG_PATH").expect("CONFIG_PATH path is not set in the environment");
    env::var("PORT").expect("CONFIG_PATH path is not set in the environment");
    let port = env::var("PORT").unwrap();
    let port: u16 = u16::from_str(&port).unwrap_or(0);
    let address = ([127, 0, 0, 1], port).into();
    let make_svc = make_service_fn(move |socket: &AddrStream| {
        let remote_address = socket.remote_addr();
        let secret_key = env::var("JWT_SECRET_KEY").unwrap();
        let config_path= env::var("CONFIG_PATH").unwrap();
        let obj = ConfigSerde::read_config(&config_path);
        let config_object: &Map<String, Value>;
        match obj {
            Ok(v) => {
                config_object = &v;

            },
            Err(_e) => println!("Error when parsing the config file")
        }

        service_fn(move |req: Request<Body>| {
            let helper = JwtHelper {};
            if helper.validate_request(&req, &secret_key) {
                println!("Request path {}", req.uri().path());
                let obj = ConfigSerde::read_config(&*config_path);
                let mut k: Map<String, Value> = Map::new();
                match obj {
                    Ok(v) => {
                        k = v;
                    }
                    Err(_e) => println!("Error")
                }
                let url = ConfigSerde::detect_url(k,String::from(req.uri().path() )).replace("\"","");
                if url == ""{
                    let body_str = format!("{:?}", "No route found");
                    let response = Response::new(Body::from(body_str));
                    Box::new(future::ok(response))
                }else{
                    return hyper_reverse_proxy::call(remote_address.ip(), &url, req);
                }
            } else {
                let body_str = format!("{:?}", "Unauthorized");
                let response = Response::new(Body::from(body_str));
                Box::new(future::ok(response))
            }
        })
    });

    let server = Server::bind(&address)
        .serve(make_svc)
        .map_err(|e| eprintln!("server error: {}", e));
    println!("Running server on {:?}", address);
    hyper::rt::run(server);
}