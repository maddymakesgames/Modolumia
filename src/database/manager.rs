use dotenv::dotenv;
use std::env;
use std::collections::HashMap;
use serde::{Serialize, Deserialize, de::DeserializeOwned};

use crate::database::search::{SearchInfo, SearchResult};
use crate::util::http_client::{HTTPClient, RequestInfo};
use crate::database::DocumentType;

#[derive(Serialize, Deserialize)]
pub struct Post {
	title: String,
	body: String,
	published: bool
}

impl Post {
	pub fn to_string(&self) -> String {
		format!("<h>{}<\\h><p>{}<\\p>", self.title, self.body)
	}
}

#[derive(Clone)]
pub struct DBManager {
	hostname: String,
	http: HTTPClient,
	db_sizes: HashMap<String, u32>
}

impl DBManager {
	pub fn new() -> Self {
		let mut map = HashMap::new();
		map.insert("Authorization".to_owned(), Self::encode_credentials());

		let m = DBManager {
			hostname: Self::establish_connection(),
			http: HTTPClient::with_headers(map),
			db_sizes: HashMap::new()
		};

		m.load_sizes()
	}

	pub async fn create_doc<S: DocumentType>(&self, database: Databases, data: S) -> Result<String, String> {
		let data = match serde_json::to_string(&data) {
			Ok(val) => val,
			Err(e) => return Err(format!("Error serializing DocumentType: {}", e))
		};

		let res = self.http.request(RequestInfo::post(format!("{}/{}", self.hostname, database.to_string()), data).content_type("application/json".to_owned())).await?;

		match serde_json::from_str::<DocumentCreationResponse>(&res.body) {
			Ok(val) => {
				if val.ok && (res.status_code == 200 || res.status_code == 201) {
					Ok(val.id)
				} else {
					Err("Document creation failed".to_owned())
				}
			},
			Err(e) => Err(format!("Error deserializing body: {}", e))
		}
	}

	pub async fn search_db<S: DocumentType>(&self, database: Databases, search: SearchInfo) -> Result<SearchResult<S>, String> {
		let data = match serde_json::to_string(&search) {
			Ok(val) => val,
			Err(e) => return Err(format!("Error serializing SearchInfo: {}", e))
		};

		let res = self.http.request(RequestInfo::post(format!("{}/{}/_find", self.hostname, database.to_string()), data).content_type("application/json".to_owned())).await?;
		
		match serde_json::from_str::<SearchResult<S>>(&res.body.clone()) {
			Ok(val) => Ok(val),
			Err(e) => Err(format!("Error deserializing response: {}", e))
		}
	}

	pub async fn get_document<S: DocumentType>(&self, database: Databases, id: String) -> Result<Document<S>, String> {
		let res = self.http.request(RequestInfo::get(format!("{}/{}/{}?attachments=false", self.hostname, database.to_string(), id))).await?;
		match serde_json::from_str::<Document<S>>(&res.body) {
			Ok(val) => Ok(val),
			Err(e) => Err(format!("Error deserializing response: {}", e))
		}
	}

	async fn get_db_size(&self, database:Databases) -> Result<u32, String> {
		let res = self.http.request(RequestInfo::get(format!("{}/{}", self.hostname, database.to_string()))).await?;
		match serde_json::from_str::<DatabaseInfo>(&res.body) {
			// Lol so many unwraps and I'm still pretending to fucking error handle 
			Ok(val) => Ok(val.doc_count),
			Err(e) => Err(format!("Error deserializing response: {}", e))
		}
	}

	fn establish_connection() -> String {
		dotenv().ok();
		env::var("DATABASE_URL").expect("DATABASE_URL must be set")
	}

	fn encode_credentials() -> String {
		dotenv().ok();
		format!("Basic {}", base64::encode(env::var("DATABASE_AUTHORIZATION").expect("DATABASE_AUTHORIZATION must be set")))
	}

	#[actix_web::main]
	async fn load_sizes(mut self) -> Self {
		let mp_db_size = self.get_db_size(Databases::MusicPacks).await.expect("Error getting music pack db size");
		let p_db_size = self.get_db_size(Databases::Palettes).await.expect("Error getting palette db size");

		info!("Loaded music packs db with size: {}", mp_db_size);
		info!("Loaded palette db with size: {}", p_db_size);

		self.db_sizes.insert(Databases::MusicPacks.to_string(), mp_db_size);
		self.db_sizes.insert(Databases::MusicPacks.to_string(), self.get_db_size(Databases::MusicPacks).await.unwrap());

		self
	}
}


#[derive(Deserialize, Debug, Serialize)]
pub struct Document<T: DocumentType> {
	pub _id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub _rev: Option<String>,
	#[serde(skip_serializing)]
	pub _attachments: Option<HashMap<String, Attachment>>,
	#[serde(flatten)]
	#[serde(bound(deserialize = "T: DeserializeOwned"))]
	pub fields: T
}

#[derive(Deserialize, Debug)]
pub struct Attachment {
	pub content_type: String,
	pub digest: String,
	pub length: Option<u64>,
	pub revpos: u8,
	pub stub: Option<bool>,
	pub encoding: Option<String>,
	pub encoded_length: Option<u64>,
	pub data: Option<Vec<u8>>
}

#[derive(Serialize, Deserialize, Debug)]
struct DocumentCreationResponse {
		id: String,
		ok: bool,
		rev: String
}

#[derive(Serialize, Deserialize, Debug)]
struct DatabaseInfo {
		db_name: String,
		purge_seq: String,
		update_seq: String,
		sizes: HashMap<String, u32>,
		props: HashMap<String, String>,
		doc_del_count: u32,
		doc_count: u32,
		disk_format_version: u8,
		compact_running: bool,
		cluster: HashMap<String, u8>,
		instance_start_time: String
}

pub enum Databases {
	MusicPacks,
	Palettes,
	Highscores,
	Speedruns,
	Users
}

impl Databases {
	#[cfg(debug_assertions)]
	pub fn to_string(&self) -> String {
		match &self {
			Databases::MusicPacks => "modolumia_music_packs_testing".to_owned(),
			Databases::Highscores => "modolumia_highscores_testing".to_owned(),
			Databases::Palettes => "modolumia_palettes_testing".to_owned(),
			Databases::Speedruns => "modolumia_speedruns_testing".to_owned(),
			Databases::Users => "modolumia_users_testing".to_owned()
		}
	}

	#[cfg(not(debug_assertions))]
	pub fn to_string(&self) -> String {
		match &self {
			Databases::MusicPacks => "modolumia_music_packs".to_owned(),
			Databases::Highscores => "modolumia_highscores".to_owned(),
			Databases::Palettes => "modolumia_palettes".to_owned(),
			Databases::Speedruns => "modolumia_speedruns".to_owned(),
			Databases::Users => "modolumia_users".to_owned()
		}
	}
}