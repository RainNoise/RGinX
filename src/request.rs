use std::net::TcpStream;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

pub struct RequestReader;

pub struct Request {
	pub method: String,
	pub uri: String,
	pub version: String,
	pub headers: HashMap<String, String>,
}

impl RequestReader {
	pub fn fetch_request_options(buf_reader: &mut BufReader<&TcpStream>) -> Result<Request, ()> {
		let (str, _) = Self::read_line(buf_reader);
		let ar_top: Vec<_> = str.split(" ").collect();

		match ar_top.len() {
			3 => {
				let method = match ar_top[0].to_uppercase().as_str() {
					m @ ("GET" | "POST") => String::from(m),
					_ => String::from("GET")
				};
				let uri = ar_top[1].replace("..", "");
				let version = String::from(ar_top[2]);

				/* 
				let ar_headers_str: Vec<_> = buf_reader
					.lines()
					.map(
						|line| 
							match line {
								Ok(res) => res,
								Err(_) => String::new()
							}
					)
					.take_while(|line| !line.is_empty())
					.collect();
				*/
				
				Ok(Request {
					method,
					uri,
					version,
					headers: HashMap::new()
				})
			},
			_ => Err(())
		}
	}

	fn read_line(buf_reader: &mut BufReader<&TcpStream>) -> (String, usize) {
		let mut buf_string = String::new();

		let length = buf_reader.read_line(&mut buf_string).unwrap();

		(buf_string, length)
	}
}
