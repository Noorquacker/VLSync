use std::rc::Rc;

use reqwest::Client;
use rand::Rng;
extern crate base64;

pub struct ConnectionState {
	pub client_id: String
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

async fn get_rooms() -> Result<(), Box<dyn std::error::Error>> {
	let resp = Client::new()
		.get("https://mc.nqind.com/vlsync/rooms.php")
		.send()
		.await?
		.json()
		.await
		.expect("Invalid response from server in rooms.php");
	Ok(())
}
