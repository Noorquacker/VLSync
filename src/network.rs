use std::rc::Rc;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use rand::Rng;
extern crate base64;

pub struct ConnectionState {
	pub client_id: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoomResponse {
	response: i32,
	stuff: Option<Vec<String>>,
	rooms: Vec<Vec<Option<String>>>
}

impl ConnectionState {
	pub fn new() -> Rc<Self> {
		let client_bytes = rand::thread_rng().gen::<[u8; 20]>();
		let client_id = base64::encode(client_bytes);

		let this = Rc::new(Self {
			client_id
		});
		this
	}
}

/// Gets the rooms.  
/// Will panic (for now) if the server returns a malformed response, aka if the server returns a non-500 error  
/// Returns a String vector containing room names.
pub fn get_rooms() -> Result<Vec<String>, Box<dyn std::error::Error>> {
	let resp: RoomResponse = reqwest::blocking::get("https://mc.nqind.com/vlsync/rooms.php")?
		.json()
		.expect("Invalid response from server in rooms.php");
	let mut rooms: Vec<String> = vec![];
	for i in resp.rooms.iter() {
		rooms.push(i[0].as_ref().unwrap_or(&"Unnamed".to_string()).clone());
	}
	Ok(rooms)
}
