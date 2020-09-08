use serde::Deserialize;
use actix_web::{post, get, web, Responder};
use crate::database::{SearchTerm, SearchBuilder, Databases};
use crate::database::Palette;
use crate::database::{DBManager};

#[derive(Deserialize, Debug)]
pub struct Search {
	search_val: String
}

#[post("/api/palettes/search")]
pub async fn search(manager: web::Data<DBManager>, web::Json(search_term_str): web::Json<Search>) -> impl Responder {
	debug!("{:?}", search_term_str);
	let search_term = SearchTerm::or()
		.child(SearchTerm::string("name")
			.child(SearchTerm::regex().child(SearchTerm::string(&format!("{}", search_term_str.search_val)))))
		.child(SearchTerm::string("author")
			.child(SearchTerm::regex().child(SearchTerm::string(&format!("{}", search_term_str.search_val)))));
	let search = SearchBuilder::new().filter(search_term).build();
	let search_res = manager.search_db::<Palette>(Databases::Palettes, search).await;
	match search_res {
		Ok(val) => {
			match val.docs {
				Some(docs) => {
					let mut output = Vec::new();
					for doc in docs {
						output.push(doc.fields);
					}
					serde_json::to_string(&output).unwrap()
				},
				None => format!("No palettes matching {} found", search_term_str.search_val)
			}
		},
		Err(e) => format!("Error searching: {}", e)
	}
}

// #[get("/palettes/{id}")]
// pub async fn get_palette(manager: web::Data<DBManager>, web::Url(search_term_str))