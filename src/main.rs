#![windows_subsystem = "windows"]

use rand::Rng;
extern crate base64;
use cpp_core::{Ptr};
use qt_core::{slot, QBox, SlotNoArgs};
use qt_widgets::{QApplication, QWidget, QVBoxLayout, QHBoxLayout, QListWidget, QLineEdit, QPushButton, QListWidgetItem, SlotOfQListWidgetItem};
use std::rc::Rc;

struct RoomChooser {
	widget: QBox<QWidget>,
	hthing: QBox<QVBoxLayout>,
	room_list: QBox<QListWidget>,
	create_room_button: QBox<QPushButton>,
	username_box: QBox<QLineEdit>
}

impl RoomChooser {
	fn new() -> Rc<RoomChooser> {
		unsafe {
			let widget = QWidget::new_0a();
			let hthing = QVBoxLayout::new_1a(&widget);
			
			let room_list = QListWidget::new();
			hthing.add_widget(&room_list);
			
			let username_box = QLineEdit::new();
			username_box.set_placeholder_text("Username");
			hthing.add_widget(&username_box);
			
			let create_room_button = QPushButton::new();
			create_room_button.set_text("Create Room");
			hthing.add_widget(&create_room_button);
			
			let this = Rc::new(Self {
				widget,
				hthing,
				room_list,
				create_room_button,
				username_box
			});
			this.init();
			this
		}
	}
	
	unsafe fn init(self: &Rc<Self>) {
		self.create_room_button.clicked().connect(&self.on_create_room());
		self.username_box.item_double_clicked().connect(&self.on_join_room());
	}
	
	#[slot(SlotNoArgs)]
	unsafe fn on_create_room(self: &Rc<Self>) {
	
	}
	
	#[slot(SlotOfQListWidgetItem)]
	unsafe fn on_join_room(self: &Rc<Self>, room: Ptr<QListWidgetItem>) {
	
	}
}

fn main() {
	let client_bytes = rand::thread_rng().gen::<[u8; 20]>();
	let client_id = base64::encode(client_bytes);
	println!("{:?}", client_id);
    println!("Doing Qt stuff...");
    QApplication::init(|_| unsafe {
		let _roomchooser = RoomChooser::new();
		QApplication::exec()
    });
}