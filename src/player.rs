use cpp_core::{Ptr, StaticUpcast, CppBox};
use qt_core::{slot, QBox, SlotNoArgs, SlotOfInt, QObject, QTimer, QString};
use qt_widgets::{QWidget, QFrame, QSlider, QHBoxLayout, QPushButton, QLabel, QVBoxLayout, QFileDialog};
use qt_gui::{QPalette, QColor, q_palette};
// use crate::q_palette::ColorRole;
use std::rc::Rc;

use vlc::{Instance, MediaPlayer};

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
	timer: QBox<QTimer>,
	media_player: MediaPlayer

}

impl StaticUpcast<QObject> for Player {
	unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
		ptr.widget.as_ptr().static_upcast()
	}
}

impl Player {
	pub fn new() -> Rc<Player> {
		unsafe {
			
			let widget = QWidget::new_0a();

			let vframe = QFrame::new_0a();
			let palette = vframe.palette();
			palette.set_color_2a(q_palette::ColorRole::Window, &QColor::from_rgb_3a(0, 0, 0));
			vframe.set_palette(palette);
			vframe.set_auto_fill_background(true);

			let position_slider = QSlider::new();
			position_slider.set_orientation(qt_core::Orientation::Horizontal);
			position_slider.set_tool_tip(&QString::from_std_str("Position")); // eugh
			position_slider.set_maximum(1000);
			position_slider.set_tracking(false);
			
			let h_box = QHBoxLayout::new_0a();
			
			let force_sync = QPushButton::new();
			force_sync.set_text(&QString::from_std_str("Sync"));
			h_box.add_widget_1a(&force_sync);

			let play = QPushButton::new();
			play.set_text(&QString::from_std_str("Play"));
			h_box.add_widget_1a(&play);

			let stop = QPushButton::new();
			stop.set_text(&QString::from_std_str("Stop"));
			h_box.add_widget_1a(&stop);

			let load = QPushButton::new();
			load.set_text(&QString::from_std_str("Load"));
			h_box.add_widget_1a(&load);

			let users_label = QLabel::new();
			users_label.set_text(&QString::from_std_str("No users"));
			h_box.add_widget_1a(&users_label);

			let exit = QPushButton::new();
			exit.set_text(&QString::from_std_str("Exit"));
			h_box.add_widget_1a(&exit);

			h_box.add_stretch_1a(1);

			let volume_slider = QSlider::new();
			volume_slider.set_maximum(100);
			volume_slider.set_tool_tip(&QString::from_std_str("Volume"));
			h_box.add_widget_1a(&volume_slider);

			let v_box = QVBoxLayout::new_0a();
			v_box.add_widget_1a(&vframe);
			v_box.add_widget_1a(&position_slider);
			v_box.add_layout_1a(&h_box);

			widget.set_layout(&v_box);

			let timer = QTimer::new_0a();
			timer.set_interval(200);

			widget.show();

			// VLC init
			let vlc_instance = Instance::new().unwrap();
			let media_player = MediaPlayer::new(&vlc_instance).unwrap();

			let this = Rc::new(Self {
				widget,
				vframe,
				palette,
				position_slider,
				h_box,
				force_sync,
				play,
				stop,
				load,
				users_label,
				exit,
				volume_slider,
				v_box,
				timer,
				media_player
			});
			this.init();
			this
		}
	}

	unsafe fn init(self: &Rc<Self>) {
		self.position_slider.slider_moved().connect(&self.slot_mark_position());
		self.position_slider.slider_released().connect(&self.slot_set_position());

		self.force_sync.clicked().connect(&self.slot_hard_sync());
		self.play.clicked().connect(&self.slot_play_update());
		self.stop.clicked().connect(&self.slot_stop());
		self.load.clicked().connect(&self.slot_open_file());
		self.exit.clicked().connect(&self.slot_close_all());

		self.volume_slider.value_changed().connect(&self.slot_set_volume());

		self.timer.timeout().connect(&self.slot_update_ui());

	}

	#[slot(SlotOfInt)]
	unsafe fn mark_position(self: &Rc<Self>, pos: i32) {
		// TODO idfk
	}

	#[slot(SlotNoArgs)]
	unsafe fn set_position(self: &Rc<Self>) {
		// TODO ACTUAL NETWORKING CRAP
	}

	// Force a sync
	#[slot(SlotNoArgs)]
	unsafe fn hard_sync(self: &Rc<Self>) {
		self.sync(true);
	}

	fn sync(self: &Rc<Self>, force_sync: bool) {
		// TODO Player.sync
	}

	#[slot(SlotNoArgs)]
	unsafe fn play_update(self: &Rc<Self>) {
		// TODO PlayPause_andupdateserver
	}

	#[slot(SlotNoArgs)]
	unsafe fn stop(self: &Rc<Self>) {
		// this should be easy whenever I do it
	}

	#[slot(SlotNoArgs)]
	unsafe fn open_file(self: &Rc<Self>) {
		// holy FRICK rust
		// QBox<QWidget> doesn't implement Copy so I can't just freaking pass the pointer all willy nilly
		let nullptr: Ptr<QWidget> = Ptr::null();
		let filename: CppBox<QString> = QFileDialog::get_open_file_name_4a(nullptr, &QString::from_std_str("Open File"), &QString::from_std_str("/"), &QString::from_std_str("Video files (*.mkv *.mp4 *.webm *.mov)"));
		println!("We got {}", filename.to_std_string());
	}

	#[slot(SlotNoArgs)]
	unsafe fn close_all(self: &Rc<Self>) {
	}

	#[slot(SlotOfInt)]
	unsafe fn set_volume(self: &Rc<Self>, vol: i32) {

	}

	#[slot(SlotNoArgs)]
	unsafe fn update_ui(self: &Rc<Self>) {
		if !self.position_slider.is_slider_down() {
			self.position_slider.set_value(0); // TODO convert from python `self.mediaplayer.get_position() * 1000`
		}
		// TODO AAAAAA MEDIA PLAYER STUFF
	}
}
