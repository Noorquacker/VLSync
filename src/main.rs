#![windows_subsystem = "windows"]

use qt_widgets::QApplication;

mod roomchooser;
mod player;
mod network;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let con_state = network::ConnectionState::new();
	println!("Client id is {}", con_state.client_id);
    println!("Displaying Qt GUI...");
    QApplication::init(|_| unsafe {
 		let _roomchooser = roomchooser::RoomChooser::new();
// 		let _player = player::Player::new();
		QApplication::exec()
    });
//     Ok(())
}
