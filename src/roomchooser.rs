use cpp_core::{Ptr, StaticUpcast};
use qt_core::{slot, QBox, SlotNoArgs, qs, QObject, QString};
use qt_widgets::{QWidget, QVBoxLayout, QListWidget, QLineEdit, QPushButton, QListWidgetItem, SlotOfQListWidgetItem, QMessageBox};
use std::rc::Rc;

use crate::network::{RoomListed, ConnectionState};
use crate::player::Player;

pub struct RoomChooser {
	widget: QBox<QWidget>,
	room_list: QBox<QListWidget>,
	create_room_button: QBox<QPushButton>,
	username_box: QBox<QLineEdit>,
	net_state: Rc<ConnectionState>,
	player: Rc<Player>
}

impl StaticUpcast<QObject> for RoomChooser {
	unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
		ptr.widget.as_ptr().static_upcast()
	}
}

impl RoomChooser {
	pub fn new(net_state: Rc<ConnectionState>, player: Rc<Player>) -> Rc<RoomChooser> {
		unsafe {
			let widget = QWidget::new_0a();
			widget.set_window_title(&QString::from_std_str("VLSync-rs"));
			let hthing = QVBoxLayout::new_1a(&widget);
			
			let room_list = QListWidget::new_0a();
			hthing.add_widget(&room_list);
			
			let username_box = QLineEdit::new();
			username_box.set_placeholder_text(&qs("Username"));
			hthing.add_widget(&username_box);
			
			let create_room_button = QPushButton::new();
			create_room_button.set_text(&qs("Create Room"));
			hthing.add_widget(&create_room_button);
			
			widget.show();
			
			let this = Rc::new(Self {
				widget,
				room_list,
				create_room_button,
				username_box,
				net_state,
				player
			});
			this.init();
			this
		}
	}
	
	unsafe fn init(self: &Rc<Self>) {

		// wait why does it insert "slot_" behind my function names
		// this is as confusing as python decorators

		self.create_room_button.clicked().connect(&self.slot_on_create_room());
		self.room_list.item_double_clicked().connect(&self.slot_on_join_room());
		self.username_box.return_pressed().connect(&self.slot_on_create_room());
		
		// WHAT THE FRICK OD YOU MEAN IT ALL HAS TO BE ASYNC???
		// nvm enabled reqwest::blocking
		let rooms: Vec<RoomListed> = self.net_state.get_rooms().unwrap();
		
		
		for i in rooms {
			self.room_list.add_item_q_string(&QString::from_std_str(i.name));
		}
		
	}
	
	fn display_err(self: &Rc<Self>, msg: String) {
		unsafe {
			QMessageBox::from_icon2_q_string(qt_widgets::q_message_box::Icon::Critical, &QString::from_std_str("Internal error"), &QString::from_std_str(msg)).exec();
		}
	}
	
	/// Yes, this literally just checks if username is empty and pops a message box if it is
	fn username_check(self: &Rc<Self>) -> bool {
		let t: String;
		unsafe {
			t = self.username_box.text().to_std_string();
		}
		if t.is_empty() {
			println!("No username given, displaying error");
			self.display_err("Enter a username, dangit".to_string());
			return false;
		}
		return true;
	}
	
	#[slot(SlotNoArgs)]
	unsafe fn on_create_room(self: &Rc<Self>) {
		let str: String = self.username_box.text().to_std_string();
		if self.username_check() {
			println!("Creating and joining room {}", str);
			match self.net_state.create_room(self.username_box.text().to_std_string()) {
				Ok(r_info) => {
					println!("Successfully made room {}", r_info.id.expect("server did not return room ID"));
					self.widget.close();
					self.player.start();
				},
				Err(e) => self.display_err(e)
			}
			
		}
	}
	
	#[slot(SlotOfQListWidgetItem)]
	unsafe fn on_join_room(self: &Rc<Self>, room: Ptr<QListWidgetItem>) {
		let r: String = room.text().to_std_string();
		if self.username_check() {
			let mut tgt: Option<RoomListed> = None;
			for i in self.net_state.get_rooms().unwrap() {
				if i.name == r {
					tgt = Some(i.clone());
				}
			}
			match tgt {
				Some(ref room) => {
					println!("Joining room {}", r);
					match self.net_state.join_room(room.id.clone(), self.username_box.text().to_std_string()) {
						Ok(r_info) => {
							println!("Returning r_info {:?}", r_info);
							if r_info.response == 500 {
								self.display_err(r_info.err.unwrap_or("No error provided".to_string()));
							}
							else {
								println!("Closing RoomChooser");
								self.widget.close();
								println!("Showing Player");
								self.player.start();
							}
						},
						Err(e) => self.display_err(e)
					}
				},
				_ => self.display_err("No room found???".to_string())
			}
		}
	}
}
