use serde::{Serialize, Deserialize, de::DeserializeOwned};

pub trait DocumentType: DeserializeOwned + Serialize {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Palette {
	pub name: String,
	pub color: [u32; 6],
	pub author: String,
	pub description: String
}

impl Palette {
	pub fn new(name: String, color: [u32; 6], author: String, description: String) -> Self {
		Palette {
			name,
			color,
			author,
			description
		}
	}
}

impl DocumentType for Palette {}

#[derive(Serialize, Deserialize, Debug)]
pub struct MusicPack {
	author: String,
	description: String,
	file_location: String
}

impl MusicPack {
	pub fn new(author: String, description: String, file_location: String) -> Self {
		MusicPack {
			author,
			description,
			file_location
		}
	}
}

impl DocumentType for MusicPack {}

#[derive(Serialize, Deserialize)]
pub struct Account {
	username: String,
	palettes: Vec<String>,
	packs: Vec<String>,
	textures: Vec<String>,

}