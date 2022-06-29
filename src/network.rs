use std::rc::Rc;

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use rand::Rng;
extern crate base64;

pub struct ConnectionState {
	pub client_id: String, // YES WE ARE STORING THE CLIENT ID AS A BASE64 ENCODED STRING PLEASE STOP DOING client_id.to_string()
	rq_client: Client,
	curr_room: Option<String>,
	in_room: bool
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoomResponse {
	response: i32,
	stuff: Option<Vec<String>>,
	rooms: Vec<RoomListed>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoomListed {
	pub id: String,
	pub name: String,
	pub movie: Option<String>,
	sha1: Option<String>,
	password: bool
}

#[derive(Deserialize, Serialize)]
pub struct RoomJoin {
	client: String,
	id: String,
	username: String
}

#[derive(Deserialize, Serialize)]
pub struct RoomJoinSuccess {
	response: i32,
	users: Vec<String>,
	err: Option<String>
}

#[derive(Deserialize, Serialize)]
pub struct RoomCreate {
	client: String,
	username: String
}

#[derive(Deserialize, Serialize)]
pub struct RoomCreateSuccess {
	response: i32,
	
	#[serde(default)]
	pub id: Option<i32>,
	
	#[serde(default)]
	pub name: Option<String>,
	
	#[serde(default)]
	pub owner: Option<String>,
	
	#[serde(default)]
	pub pass: Option<String>,
	
	#[serde(default)]
	err: Option<String>
}

impl ConnectionState {
	pub fn new() -> Rc<Self> {
		let client_bytes = rand::thread_rng().gen::<[u8; 20]>();
		let client_id = base64::encode(client_bytes);
		let rq_client = reqwest::blocking::Client::new();
		
		println!("Initialized client ID {}", client_id);

		let this = Rc::new(Self {
			client_id,
			rq_client,
			curr_room: None,
			in_room: false
		});
		this
	}
	
	
	// TODO test this function!
	pub fn join_room(self: &Rc<Self>, room_id: String, username: String) -> Result<RoomJoinSuccess, String> {
		println!("Joining room ID {}", room_id);
		let room_join = RoomJoin {
			client: self.client_id.clone(),
			username,
			id: room_id // TODO TODO THIS IS NOT HOW ROOMS ACTUALLY WORK OH FRICK OH FRICK
		};
		let resp: RoomJoinSuccess = self.rq_client.post("https://www.nqind.com/vlsync/join.php")
			.json(&room_join)
			.send()
			.ok()
			.expect("Could not join room")
			.json()
			.expect("Invalid response from server");
		
		Ok(resp)
	}
	
	pub fn create_room(self: &Rc<Self>, username: String) -> Result<RoomCreateSuccess, String> {
		let room_create = RoomCreate {
			client: self.client_id.clone(),
			username
		};
		let resp = self.rq_client.post("https://www.nqind.com/vlsync/createRoom.php")
			.json(&room_create)
			.send()
			.ok()
			.expect("Could not create room")
			.text()
			.ok()
			.expect("Completely invalid response from server");
		let ret: RoomCreateSuccess = match serde_json::from_str(&resp) {
			Ok(e) => e,
			Err(e) => RoomCreateSuccess {
				response: 400,
				err: Some(format!("Failed to parse JSON {}", resp)),
				id: None, name: None, owner: None, pass: None
				
			}
		};
		
		if ret.response == 400 {
			return Err(ret.err.unwrap_or("No error provided".to_string()));
		}
		Ok(ret)
	}
	
	/// Gets the rooms.  
	/// Will panic (for now) if the server returns a malformed response, aka if the server returns a non-500 error  
	/// Returns a String vector containing room names.
	/// Note that this does _not_ use the same Reqwest client used in future requests
	pub fn get_rooms(self: &Rc<Self>) -> Result<Vec<RoomListed>, Box<dyn std::error::Error>> {
		let resp: RoomResponse = self.rq_client.get("https://www.nqind.com/vlsync/rooms.php")
			.send()?
			.json()
			.expect("Invalid response from server in rooms.php");
// 		let mut rooms: Vec<String> = vec![];
// 		for i in resp.rooms.iter() {
// 			rooms.push(i.name.as_ref().unwrap_or(&"Unnamed".to_string()).clone());
// 		}
		Ok(resp.rooms)
	}

}

