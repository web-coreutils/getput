use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use std::convert::Infallible;

use anyhow::Result;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use clap::Parser;

use std::fs::File;
use std::io::{BufReader, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::path::Path;

const NAME: &str = "getput";
const AUTHOR: &str = "github.com/{lquenti,meipp}";
const VERSION: &str = "0.1";
const ABOUT: &str = "putget";

#[derive(Parser, Debug, Clone)]
#[clap(name = NAME, author = AUTHOR, version = VERSION, about = ABOUT, long_about = None)]
struct Cli {
    /// Where to store the database
    #[clap(short = 'f', long, default_value_t = String::from("./db.json"))]
    database_file: String,
    /// The maximum amount of characters allowed for a key
    #[clap(short = 'k', long, default_value_t = 1024)]
    max_key_length: usize,
    /// The maximum amount of characters allowed for a value
    #[clap(short = 'v', long, default_value_t = 1024*1024*1024)]
    max_value_length: usize,
    /// On which port to listen
    #[clap(short, long, default_value_t = 6379)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let cli2 = cli.clone();

    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), cli.port));
    let hm: HashMap<String, String> = hashmap_from_file(&cli.database_file)?;
    let storage: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(hm));
    let storage2 = storage.clone();

    let make_service = make_service_fn(move |_conn| {
        let cli = cli.clone();
        let storage = storage.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let cli = cli.clone();
                let storage = storage.clone();
                async move { Ok::<_, Infallible>(handle(&cli, storage.clone(), req)) }
            }))
        }
    });

    Server::bind(&addr)
        .serve(make_service)
        .with_graceful_shutdown(shutdown(storage2, &cli2.database_file))
        .await?;
    Ok(())
}

fn split_on(s: String, c: char) -> Option<(String, String)> {
    let mut iter = s.splitn(2, c);
    Some((iter.next()?.into(), iter.next()?.into()))
}

fn response(status: u16, body: &str) -> Response<Body> {
    Response::builder()
        .status(status)
        .body(Body::from(String::from(body)))
        .unwrap()
}

fn handle(
    cli: &Cli,
    storage: Arc<Mutex<HashMap<String, String>>>,
    req: Request<Body>,
) -> Response<Body> {
    let key: String = req.uri().path().into();
    if key.len() > cli.max_key_length {
        return response(414, "URI Too Long");
    }
    println!("{:?} {:?}", req.method(), req.uri().path());

    match req.method() {
        &Method::GET => match storage.lock().unwrap().get(&key) {
            None => response(404, "Not Found"),
            Some(value) => response(200, value),
        },
        &Method::PUT => {
            let (k, v) = split_on(key, '=').expect("PUT URI must have form key=value");
            if v.len() > cli.max_value_length {
                return response(413, "Payload Too Large");
            }

            let inserted = storage.lock().unwrap().insert(k, v);

            match inserted {
                None => response(201, "Created"),
                Some(_) => response(200, "OK"),
            }
        }
        _ => response(405, "Method Not Allowed"),
    }
}

async fn shutdown(storage: Arc<Mutex<HashMap<String, String>>>, database_file: &str) {
    tokio::signal::ctrl_c()
        .await
        .expect("Could not set interrupt handler");
    println!("\nShutting down server");
    println!("storage: {:?}", storage.lock().unwrap());

    let s = serde_json::to_string(&*storage.lock().unwrap()).unwrap();

    let mut output = File::create(database_file).unwrap();
    write!(output, "{}", s).unwrap();
}

pub fn hashmap_from_file(file_path: &str) -> Result<HashMap<String, String>> {
    if !Path::new(file_path).exists() {
        return Ok(HashMap::new());
    }

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let res = serde_json::from_reader(reader)?;
    Ok(res)
}
