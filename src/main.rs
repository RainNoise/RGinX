mod request;
mod response;
mod config;


use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io::BufReader;
use response::ResponseFabric;
use rginx::ThreadPool;
use crate::request::RequestReader;
use config::Config;


fn main() {
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	let pool = ThreadPool::new(4);
	
	for stream in listener.incoming() {
		let stream = stream.unwrap();
		pool.execute(|| handle_connection(stream));
	}
}

fn handle_connection(mut stream: TcpStream) {
	let mut buf_reader = BufReader::new(&stream);
	let request_result = RequestReader::fetch_request_options(&mut buf_reader);
	let response_constructor = ResponseFabric::new(Config::new(String::new()));

	match request_result {
		Ok(request) => stream.write_all(
				&response_constructor.construct_response(&request).to_bytes()
			).unwrap(),
		Err(_) => stream.write_all(
				&response_constructor.bad_request().to_bytes()
			).unwrap()
	};

	
	stream.flush().unwrap();
}

