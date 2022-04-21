#![windows_subsystem = "windows"]

use rand::Rng;
extern crate base64;
use qt_widgets::QApplication;

mod roomchooser;
mod player;

fn main() {
	let client_bytes = rand::thread_rng().gen::<[u8; 20]>();
	let client_id = base64::encode(client_bytes);
	println!("Client id is {}", client_id);
    println!("Displaying Qt GUI...");
    QApplication::init(|_| unsafe {
 		let _roomchooser = roomchooser::RoomChooser::new();
//		let _player = player::Player::new();
		QApplication::exec()
    });
}
