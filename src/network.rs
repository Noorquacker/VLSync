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
	rooms: Vec<String>
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
pub async fn get_rooms() -> Result<Vec<String>, Box<dyn std::error::Error>> {
	let resp: RoomResponse = Client::new()
		.get("https://mc.nqind.com/vlsync/rooms.php")
		.send()
		.await?
		.json()
		.await
		.expect("Invalid response from server in rooms.php");
	Ok(resp.rooms)
}
