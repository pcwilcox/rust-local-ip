extern crate local_ip;

use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();

    let mut x : Option<&str> = None;

    if args.len() >= 2 {
        x = Some(&args[1]);
    }

    println!("{}", local_ip::get(x).expect("Could not find local IP address.").to_string());
}
