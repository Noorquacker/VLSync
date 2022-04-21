use cpp_core::{Ptr, StaticUpcast};
use qt_core::{slot, QBox, SlotNoArgs, qs, QObject, QString};
use qt_widgets::{QWidget, QVBoxLayout, QListWidget, QLineEdit, QPushButton, QListWidgetItem, SlotOfQListWidgetItem};
use std::rc::Rc;

pub struct RoomChooser {
	widget: QBox<QWidget>,
	room_list: QBox<QListWidget>,
	create_room_button: QBox<QPushButton>,
	username_box: QBox<QLineEdit>
}

impl StaticUpcast<QObject> for RoomChooser {
	unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
		ptr.widget.as_ptr().static_upcast()
	}
}

impl RoomChooser {
	pub fn new() -> Rc<RoomChooser> {
		unsafe {
			let widget = QWidget::new_0a();
			let hthing = QVBoxLayout::new_1a(&widget);
			
			let room_list = QListWidget::new_0a();
			hthing.add_widget(&room_list);
			
			let username_box = QLineEdit::new();
			username_box.set_placeholder_text(&qs("Username"));
			hthing.add_widget(&username_box);
			
			let create_room_button = QPushButton::new();
			create_room_button.set_text(&qs("Create Room"));
			hthing.add_widget(&create_room_button);

			room_list.add_item_q_string(&QString::from_std_str("bruh"));
			
			widget.show();
			
			let this = Rc::new(Self {
				widget,
				room_list,
				create_room_button,
				username_box
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
	}
	
	#[slot(SlotNoArgs)]
	unsafe fn on_create_room(self: &Rc<Self>) {
		let str: String = self.username_box.text().to_std_string();
		println!("Creating and joining room {}", str);
	}
	
	#[slot(SlotOfQListWidgetItem)]
	unsafe fn on_join_room(self: &Rc<Self>, room: Ptr<QListWidgetItem>) {
		let r: String = room.text().to_std_string();
		println!("Joining room {}", r);
	}
}
