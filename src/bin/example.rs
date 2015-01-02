#![feature(phase)]
#[phase(plugin, link)]
extern crate log;
extern crate cql;

use std::io::TcpStream;

use cql::CqlStream;
use cql::Frame;


pub fn main() {
    match TcpStream::connect("127.0.0.1:9042") {
        Err(_) => debug!("failed to connect"),
        Ok(mut stream) => {
            startup(&mut stream);
            debug!("startup response says: {}", startup(&mut stream));
           // debug!("query response says: {}", query(&mut stream, "select * from foo.bar".to_string()));
        }
    }
}

fn startup(stream:&mut CqlStream) {
    let mut bytes:Vec<u8> = Vec::new();
    let frame = Frame::build_startup(&bytes);
    match stream.write_frame(frame) {
        Err(err) => panic!("response: {}", err),
        Ok(_) => match stream.get_next_frame() {
            Err(_) => {panic!()},
            Ok(frame) => {debug!("response: {}",frame.get_header())}
        }
    }
}
