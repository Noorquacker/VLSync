#![windows_subsystem = "windows"]

use qt_widgets::QApplication;
// use std::rc::Rc;

mod roomchooser;
mod player;
mod network;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let con_state = network::ConnectionState::new();
    println!("Displaying Qt GUI...");
    QApplication::init(|_| unsafe {
		let player = player::Player::new(con_state.clone());
		let _roomchooser = roomchooser::RoomChooser::new(con_state.clone(), player);
		//let _player = player::Player::new(con_state);
		QApplication::exec()
    });
}
