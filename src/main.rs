use std::env;
use std::net::TcpStream;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: poke [host] [port]");
        std::process::exit(1);
    }

    let addr: String = args[1..].join(":");
    match TcpStream::connect(&addr) {
        Ok(_) => println!("Connection to {} [tcp] succeeded.", addr),
        Err(e) => eprintln!("Connection to {} [tcp] failed: {}.", addr, e),
    }       
}
