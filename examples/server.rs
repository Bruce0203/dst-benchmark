use fast_collections::{typenum::U3, Vec};
use httparse::{Request, EMPTY_HEADER};
use std::{
    io::Read,
    net::{SocketAddr, TcpListener},
};

const GET: Vec<u8, U3> = Vec::from_array(*b"GET");

fn main() {
    const PORT: u16 = 25525;
    let addr: SocketAddr = format!("0.0.0.0:{PORT}").parse().unwrap();
    let listener = TcpListener::bind(addr).unwrap();
    let (mut stream, _addr) = listener.accept().unwrap();
    let mut buf = [0; 4096];
    let read = stream.read(&mut buf).unwrap();
    let read = &mut buf[..read];
    let mut headers = [EMPTY_HEADER; 25];
    let mut request = Request::new(&mut headers);
    let res = request.parse(read);
    println!("{:?}", res);
    let value = res.unwrap();
    println!("{:?}", headers[0]);
    let value = unsafe { std::str::from_utf8_unchecked(read) };
    println!("{}", value);
}
