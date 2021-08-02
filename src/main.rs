use std::io::{self, BufRead, Read, Write};
use std::net::TcpStream;
use std::thread::{self};
use std::{str};
use std::env;

fn main()  {
    let args: Vec<String> = env::args().collect();
    let host = match args.get(1){
        Some(val) => val,
        None => {println!("A hostname is required"); std::process::exit(1)}
    };
    let port = match args.get(2){
        Some(val) => val,
        None => {println!("A port is required"); std::process::exit(1)}
    };
    let address = format!("{}:{}", host, port);
    let mut stream = TcpStream::connect(address).unwrap();
    let mut y = TcpStream::try_clone(&stream).unwrap();
    thread::spawn(move || read_line(&mut y));
    loop{
        let mut client_buffer = [0u8; 100];
        stream.read(&mut client_buffer).unwrap();
        stream.flush().unwrap();
        println!("{}", str::from_utf8(&client_buffer).unwrap());
    }
}

fn read_line(stream: &mut TcpStream){
    let stdin = io::stdin();
    for line in stdin.lock().lines(){
        let l = line.unwrap().clone();
        stream.write(l.as_bytes()).unwrap();
    }
}
