use std::rc::Rc;
use std::cell::RefCell;

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
//use serde_json::from_str;
use rand::Rng;
extern crate base64;

pub struct ConnectionState {
	pub client_id: String, // YES WE ARE STORING THE CLIENT ID AS A BASE64 ENCODED STRING PLEASE STOP DOING client_id.to_string()
	rq_client: Client,
	pub room_state: RefCell<RoomState>
}

pub struct RoomState {
	pub users: Vec<String>,
	pub in_room: bool,
	pub room_id: Option<String>,
	pub position: i32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoomResponse {
	response: i32,
	stuff: Option<Vec<String>>,
	rooms: Vec<RoomListed>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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
	username: String,
	password: Option<String>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RoomJoinSuccess {
	pub response: i32,
	pub users: Option<Vec<String>>,
	pub err: Option<String>
}

#[derive(Deserialize, Serialize)]
pub struct RoomCreate {
	client: String,
	username: String
}

#[derive(Deserialize, Serialize, Clone)]
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

#[derive(Serialize)]
pub struct HeartbeatReq {
	client: String,
	id: String
}

#[derive(Deserialize, Debug)]
pub struct RoomStateHeartbeat {
	pub timecode: i32,
	pub paused: bool,
	pub modified: i32,
	pub offset: i32
}

#[derive(Deserialize, Debug)]
pub struct HeartbeatResp {
	pub response: i32,
	pub users: Vec<String>,
	pub room_state: RoomStateHeartbeat,
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
			room_state: RefCell::new(
				RoomState { users: Vec::new(), in_room: false, room_id: None, position: 0 }
			),
		});
		this
	}
	
	
	// TODO test this function!
	pub fn join_room(self: &Rc<Self>, room_id: String, username: String) -> Result<RoomJoinSuccess, String> {
		println!("Joining room ID {} with client ID {}", room_id, &self.client_id);
		let room_join = RoomJoin {
			client: self.client_id.clone(),
			username,
			id: room_id.clone(), // TODO TODO THIS IS NOT HOW ROOMS ACTUALLY WORK OH FRICK OH FRICK
			password: None
		};
		let resp: Result<String, reqwest::Error> = self.rq_client.post("https://www.nqind.com/vlsync/join.php")
			.json(&room_join)
			.send()
			.ok()
			.unwrap()
			.text();
		let txt = resp.unwrap();
		let a: Result<RoomJoinSuccess, serde_json::Error> = serde_json::from_str(&txt);
		match a {
			Ok(r) => {
				if let Some(k) = r.clone().users {
					let mut room_state = self.room_state.borrow_mut();
					
					for i in k {
						room_state.users.push(i);
					}
					room_state.in_room = true;
					room_state.room_id = Some(room_id.clone());
				}
				Ok(r)
			},
			Err(_e) => {
				Err(txt)
			}
		}
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
			Err(_) => RoomCreateSuccess {
				response: 400,
				err: Some(format!("Failed to parse JSON: {}", resp)),
				id: None, name: None, owner: None, pass: None
				
			}
		};
		
		let mut room_state = self.room_state.borrow_mut();
		
		room_state.in_room = true;
		room_state.room_id = Some(ret.id.expect("Room was created, but no room ID was provided?").to_string());
		
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
		Ok(resp.rooms)
	}
	
	/// Heartbeat request for telling the server we're here and asking where we should be
	pub fn heartbeat(self: &Rc<Self>) -> Result<HeartbeatResp, String>{
		let room_state = self.room_state.borrow();
		if let Some(room_id) = room_state.room_id.clone() {
			let heartbeat_req = HeartbeatReq { 
				client: self.client_id.clone(),
				id: room_id.to_string()
			};
			
			let resp: Result<HeartbeatResp, _> = self.rq_client.post("https://www.nqind.com/vlsync/heartbeat.php")
				.json(&heartbeat_req)
				.send()
				.ok()
				.expect("bruh")
				.json();
			
			match resp {
				Ok(r) => Ok(r),
				Err(e) => {
					Err(format!("Error getting heartbeat from server: {}", e.to_string()))
				}
			}
		}
		else {
			Err("Attempt to heartbeat with no room ID specified".to_string())
		}
	}

}

