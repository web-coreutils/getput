use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server, Method};
use hyper::service::{make_service_fn, service_fn};

use anyhow::Result;


use clap::Parser;

async fn hello_world(req: Request<Body>) -> Result<Response<Body>> {

    println!("{:?}", req);

    let key = req.uri().path(); // TODO maybe remove "/"

    let res = match req.method() {
        &Method::GET => {
            String::from("GET worked")
        },
        &Method::PUT => {
            let val = hyper::body::to_bytes(req.into_body()).await?;
            println!("based? {:?}", val);
            String::from("PUT worked")
        }
        _ => String::from("bad"),
    };
    Ok(Response::new(res.into()))
}

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

/*
fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
}
*/

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
