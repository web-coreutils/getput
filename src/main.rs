use std::convert::Infallible;
use hyper::{Body, Request, Response, Server, Method};
use hyper::service::{make_service_fn, service_fn};

use anyhow::Result;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};


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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:3000".parse()?;
    let hm: HashMap<String, String> = HashMap::new();
    let storage: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(hm));

    let make_service = make_service_fn(move |_conn| {
        let storage = storage.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let storage = storage.clone();
                async move {
                    Ok::<_, Infallible>(handle(storage.clone(), req))
                }
            }))
        }
    });

    Server::bind(&addr).serve(make_service).await?;
    Ok(())
}

fn split_on(s: String, c: char) -> Option<(String, String)> {
    let mut iter = s.splitn(2, c);
    Some((iter.next()?.into(), iter.next()?.into()))
}

fn handle(storage: Arc<Mutex<HashMap<String, String>>>, req: Request<Body>) -> Response<Body> {
    let key: String = req.uri().path().into();
    println!("{:?} {:?}", req.method(), req.uri().path());

    match req.method() {
        &Method::GET => Response::new(Body::from(format!("GET {:?}", storage.lock().unwrap().get(&key)))),
        &Method::PUT => {
            let (k, v) = split_on(key, '=').expect("PUT URI must have form key=value");
            storage.lock().unwrap().insert(k, v);

            Response::new(Body::from(format!("PUT {:?}", storage.lock().unwrap())))
        },
        _ => Response::new(Body::from(format!("bad")))
    }
}
