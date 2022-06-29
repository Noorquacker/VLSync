#![windows_subsystem = "windows"]

use qt_widgets::QApplication;
// use std::rc::Rc;

mod roomchooser;
mod player;
mod network;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let con_state = roomchooser::network::ConnectionState::new();
	println!("Client id is {}", con_state.client_id);
    println!("Displaying Qt GUI...");
    QApplication::init(|_| unsafe {
 		let _roomchooser = roomchooser::RoomChooser::new(con_state);
// 		let _player = player::Player::new();
		QApplication::exec()
    });
}
