use std::net::TcpStream;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    host: String,

    port: String,
}

fn main() {
    let args = Args::parse();

    let addr: String = args.host + ":" + &args.port;
    match TcpStream::connect(&addr) {
        Ok(_) => println!("Connection to {} [tcp] succeeded.", addr),
        Err(e) => eprintln!("Connection to {} [tcp] failed: {}.", addr, e),
    }       
}
