#![windows_subsystem = "windows"]

use cpp_core::{Ptr, StaticUpcast};
use qt_core::{slot, QBox, SlotNoArgs, qs, QObject, QTimer};
use qt_widgets::{QWidget, QFrame, QSlider, QHBoxLayout, QPushButton, QLabel, QVBoxLayout};
use qt_gui::QPalette;
use std::rc::Rc;

pub struct Player {
	widget: QBox<QWidget>,
	vframe: QBox<QFrame>,
	palette: cpp_core::Ref<QPalette>, // REALLY HOPING MY GUESS IS CORRECT
	position_slider: QBox<QSlider>,
	h_box: QBox<QHBoxLayout>,
	force_sync: QBox<QPushButton>,
	play: QBox<QPushButton>,
	stop: QBox<QPushButton>,
	load: QBox<QPushButton>,
	users_label: QBox<QLabel>,
	exit: QBox<QPushButton>,
	volume_slider: QBox<QSlider>,
	v_box: QBox<QVBoxLayout>,
	timer: QTimer
}

impl StaticUpcast<QObject> for Player {
	unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
		ptr.widget.as_ptr().static_upcast()
	}
}

impl Player {
	pub fn new() -> Player {
		unsafe {
			
			let widget = QWidget::new_0a();
			let vframe = QFrame::new_0a();
			let palette = vframe.palette();
			// (*palette).setColor(qt_gui::q_palette::window); // ????????????? not even gonna try
			let position_slider = QSlider::new();
			
			let h_box = QHBoxLayout::new_0a();
			
			let force_sync = QPushButton::new();
			
			let this = Rc::new(Self {
				widget,
				vframe,
				palette,
				position_slider,
				h_box,
				force_sync
			});
			this
		}
	}
}
