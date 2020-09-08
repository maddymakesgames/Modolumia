use actix_web::client::{Client, ClientResponse};
use actix_web::http::HeaderMap;
use std::collections::HashMap;

#[derive(Clone)]
pub struct HTTPClient {
	headers: Option<HashMap<String, String>>
}

impl HTTPClient {

	pub fn new() -> Self {
		HTTPClient {
			headers: None
		}
	}

	pub fn with_headers(headers: HashMap<String, String>) -> Self {
		HTTPClient {
			headers: Some(headers)
		}
	}

	pub async fn request(&self, info: RequestInfo) -> Result<HTTPResponse, String> {
		match info.r#type {
			RequestType::GET => self.get_request(info).await,
			RequestType::POST => self.post_request(info).await,
			RequestType::PUT => self.put_request(info).await
		}
	}

	async fn post_request(&self, info: RequestInfo) -> Result<HTTPResponse, String> {
		let mut req = Client::new().post(info.url);
		
		if info.content_type.is_some() {
			req = req.content_type(info.content_type.unwrap());
		}
		
		match &self.headers {
			Some(map) => {
				for (key, val) in map {
					req = req.header(key, val.to_owned());
				}
			}
			None => {}
		}

		match info.data {
			Some(str) => {
				match req.send_body(&str).await {
					Err(e) => Err(format!("Error making post request: {}", e)),
					Ok(response) => Ok(HTTPResponse::from(response).await)
				}
			}
			None => Err("A post request needs data to send".to_owned())
		}
	}

	async fn put_request(&self, info: RequestInfo) -> Result<HTTPResponse, String> {
		let mut req = Client::new().put(info.url);
		
		if info.content_type.is_some() {
			req = req.content_type(info.content_type.unwrap());
		}
		
		match &self.headers {
			Some(map) => {
				for (key, val) in map {
					req = req.header(key, val.to_owned());
				}
			}
			None => {}
		}

		match info.data {
			Some(str) => {
				match req.send_body(&str).await {
					Err(e) => Err(format!("Error making post request: {}", e)),
					Ok(response) => Ok(HTTPResponse::from(response).await)
				}
			}
			None => Err("A post request needs data to send".to_owned())
		}
	}

	async fn get_request(&self, info: RequestInfo) -> Result<HTTPResponse, String> {
		let mut req = Client::new().get(&info.url);
		
		if info.content_type.is_some() {
			req = req.content_type(info.content_type.unwrap());
		}
		
		match &self.headers {
			Some(map) => {
				for (key, val) in map {
					req = req.header(key, val.to_owned());
				}
			}
			None => {}
		}

		let response = req.send().await.unwrap();
		Ok(HTTPResponse::from(response).await)
	}
}

pub struct RequestInfo {
	url: String,
	content_type: Option<String>,
	r#type: RequestType,
	data: Option<String>
}

impl RequestInfo {
	pub fn get(path: String) -> Self {
		RequestInfo {
			url: path,
			r#type: RequestType::GET,
			data: None,
			content_type: None
		}
	}
	
	pub fn post(path: String, data: String) -> Self {
		RequestInfo {
			url: path,
			data: Some(data),
			content_type: None,
			r#type: RequestType::POST
		}
	}
	
	pub fn put(path: String, data: String) -> Self {
		RequestInfo {
			url: path,
			data: Some(data),
			content_type: None,
			r#type: RequestType::PUT
		}
	}
	
	pub fn content_type(mut self, content_type: String) -> Self {
		self.content_type = Some(content_type);
		self
	}
}

pub enum RequestType {
	GET,
	POST,
	PUT
}

pub struct HTTPResponse {
	pub body: String,
	pub headers: HeaderMap,
	pub status_code: u16
}

// I hate ClientResponse....
use std::pin::Pin;
use std::boxed::Box;
use tokio::stream::Stream;
use actix_web::web::Bytes;
use actix_http::encoding::Decoder;
use actix_http::error::PayloadError;
use actix_http::Payload;

impl HTTPResponse {
	pub async fn from(mut response: ClientResponse<Decoder<Payload<Pin<Box<dyn Stream<Item = Result<Bytes, PayloadError>>>>>>>) -> Self {
		let body: actix_web::web::Bytes = response.body().await.unwrap();
		let headers = response.headers();
		HTTPResponse {
			headers: headers.to_owned(),
			body: String::from_utf8(body.to_vec()).unwrap(),
			status_code: response.status().as_u16()
		}
	}
}