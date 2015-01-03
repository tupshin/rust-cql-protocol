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
            debug!("startup response says: {}", startup(&mut stream));

            //the second one should fail because the stream is already initialized
            debug!("startup response says: {}", startup(&mut stream));
           // debug!("query response says: {}", query(&mut stream, "select * from foo.bar".to_string()));
        }
    }
}

fn startup(stream:&mut CqlStream) {
    let outbound_body_bytes = &mut Vec::new();
    let response_bytes = &mut Vec::new();
    let frame = Frame::build_startup(outbound_body_bytes);
    debug!("startup frame {}",frame);
    match stream.write_frame(frame) {
        Err(err) => panic!("response: {}", err),
        Ok(_) => match stream.get_next_frame(response_bytes) {
            Err(err) => {panic!(err)},
            Ok(frame) => {
                debug!("response frame: {}",frame);
                debug!("response frame size: {}",frame.len());
            }
        }
    }
}
