use crate::{request::Request, config::Config};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct ResponseFabric {
	config: Config,
}

enum MIME {
	Text(String),
	Image(String),
	Application(String)
}

impl MIME {
	pub fn to_string(self) -> String{
		match self {
			MIME::Text(c) => String::from("text/") + &c,
			MIME::Image(c) => String::from("image/") + &c,
			MIME::Application(c) => String::from("application/") + &c,
		}
	}
}

impl ResponseFabric {
	pub fn new(config: Config) -> ResponseFabric {
		ResponseFabric { config }
	}

	pub fn construct_response (&self, request: &Request) -> Response{
		let request_path = PathBuf::from(request.uri.trim_start_matches("/").to_string());
		let root = Path::new(&self.config.root).to_path_buf();
		let path = root.join(request_path);

		if path.exists() {
			let req_file = if path.is_dir() {
				path.join("index.html")
			}
			else {
				path
			};

			match fs::read(req_file.clone()) {
				Ok(content) => self.ok(&content, self.get_mime(&req_file).to_string()),
				Err(_) => self.forbidden()
			}
		}
		else {
			self.not_found()
		}
	}

	pub fn ok(&self, content: &Vec<u8>, mime: String) -> Response {
		let mut headers = HashMap::new();

		headers.insert(
			String::from("Content-type"), mime
		);
		headers.insert(
			String::from("Content-length"), content.len().to_string()
		);

		Response {
			status: String::from("200 Ok"),
			body: content.clone(), 
			headers
		}
	}

	pub fn bad_request(&self) -> Response {
		let root = Path::new(&self.config.root).to_path_buf();
		let req_file = root.join("402.html");
		let mut headers = HashMap::new();

		headers.insert(
			String::from("Content-type"), String::from("text/html")
		);
		
		let content = match fs::read(req_file.clone()) {
			Ok(content) => content,
			Err(_) => b"402 Bad Request".to_vec()
		};
			
		headers.insert(
			String::from("Content-length"), content.len().to_string()
		);
		
		Response {
			status: String::from("402 Bad Request"),
			body: content, 
			headers
		}
	}
	
	pub fn forbidden(&self) -> Response {
		let root = Path::new(&self.config.root).to_path_buf();
		let req_file = root.join("403.html");
		let mut headers = HashMap::new();

		headers.insert(
			String::from("Content-type"), String::from("text/html")
		);
		
		let content = match fs::read(req_file.clone()) {
			Ok(content) => content,
			Err(_) => b"403 Forbidden".to_vec()
		};
			
		headers.insert(
			String::from("Content-length"), content.len().to_string()
		);
		
		Response {
			status: String::from("403 Forbidden"),
			body: content, 
			headers
		}
	}

	pub fn not_found(&self) -> Response {
		let root = Path::new(&self.config.root).to_path_buf();
		let req_file = root.join("404.html");
		let mut headers = HashMap::new();

		headers.insert(
			String::from("Content-type"), String::from("text/html")
		);
		
		let content = match fs::read(req_file.clone()) {
			Ok(content) => content,
			Err(_) => b"404 Not Found".to_vec()
		};
			
		headers.insert(
			String::from("Content-length"), content.len().to_string()
		);
		
		Response {
			status: String::from("404 Not Found"),
			body: content, 
			headers
		}
	}
	fn get_mime(&self, file: &PathBuf) -> MIME{
		let ext = match file.extension() {
			Some(x) => x.to_str().expect("Error getting extension"),
			None => ""
		};
		
		match ext.to_ascii_lowercase().as_str() {
			"html" | "htm" => MIME::Text(String::from("html")),
			"txt" => MIME::Text(String::from("plain")),
			"css" => MIME::Text(String::from("css")),
			"js" => MIME::Application(String::from("javascript")),
			"png" => MIME::Image(String::from("png")),
			"jpg" | "jpeg" => MIME::Image(String::from("jpeg")),
			&_ => MIME::Application(String::from("binary"))
		}
	}

	
}


pub struct Response {
	pub status: String,
	pub headers: HashMap<String, String>,
	pub body: Vec<u8>
}

impl Response {
	pub fn to_bytes(self) -> Vec<u8> {
		let status = format!("HTTP/1.1 {}\r\n", self.status);

		let mut headers = String::new();
		
		for(key, value) in self.headers.iter() {
			headers = headers + &format!("{}: {}\r\n", &key, &value);
		};

		[
			status.as_bytes(),
			 headers.as_bytes(),
			b"\r\n",
			&self.body
		].concat()
	}
}