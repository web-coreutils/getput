use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, Method, Error};
use hyper::service::{make_service_fn, service_fn};

use anyhow::Result;

use std::collections::HashMap;
use std::sync::{Arc, RwLock, atomic::{Ordering, AtomicUsize}};


use clap::Parser;


const NAME: &str = "getput";
const AUTHOR: &str = "github.com/{lquenti,meipp}";
const VERSION: &str = "0.1";
const ABOUT: &str = "putget";

#[derive(Parser, Debug)]
#[clap(name = NAME, author = AUTHOR, version = VERSION, about = ABOUT, long_about = None)]
struct Cli {
    /// Where to store the database
    #[clap(short = 'f', long, default_value_t = format!("./{}.db", NAME))]
    database_file: String,
    /// The maximum amount of characters allowed for a key
    #[clap(short = 'k', long, default_value_t = 1024)]
    max_key_length: u64,
    /// The maximum amount of characters allowed for a value
    #[clap(short = 'v', long, default_value_t = 1024*1024*1024)]
    max_value_length: u64,
    /// On which port to listen
    #[clap(short, long, default_value_t = 6379)]
    port: u64,
}

async fn handle_req(req: Request<&str>, counter: Arc<AtomicUsize>) {
    println!("{:?}", req.method());
}

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let counter = Arc::new(AtomicUsize::new(0));

    let make_service = make_service_fn(move |_| {
        let counter = counter.clone();

        async move {
            Ok::<_, Error>(service_fn(move |req| {
                handle_req(req, counter.clone())
                /*
                let key: &str = req.uri().path();
                match req.method() {
                    &Method::GET => {
                        println!("GET {:?}", &key);
                    }
                    &Method::PUT => {
                        println!("PUT {:?}", &key);
                    }
                    _ => {
                        println!("anything else");
                    },
                }
                */
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_service);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
