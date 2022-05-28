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

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
}
