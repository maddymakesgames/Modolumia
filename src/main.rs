#[macro_use]
extern crate log;

pub mod api;
pub mod database;
pub mod util;

use actix_files::{Files};

use actix_web::{App, HttpServer};


use crate::database::{DBManager};
use crate::util::logging::init_logging;


fn main() {
	init_logging();
	let manager = DBManager::new();
	run(manager).unwrap();
}

#[actix_web::main]
async fn run(manager: DBManager) -> std::io::Result<()> {
	let mut server = HttpServer::new(move || App::new()
			.data(manager.clone())
			.service(api::palettes::search)
			.service(Files::new("/resources", "resources"))
			.service(Files::new("/", "html"))
		);

	#[cfg(debug_assertions)]
	{
		use listenfd::ListenFd;
		let mut listenfd = ListenFd::from_env();
		server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
			server.listen(l)?
		} else {
			server.bind("127.0.0.1:3000")?
		};
	}

	server.run().await
}