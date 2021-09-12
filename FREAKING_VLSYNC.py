#!/usr/bin/python3
import requests, base64, vlc, sys, time, os.path, random
from PyQt5 import QtGui, QtCore
from PyQt5 import QtWidgets
from PyQt5.QtCore import QThread, QObject, pyqtSignal
from PyQt5.QtWidgets import QApplication, QWidget

class RoomChooser(QtWidgets.QMainWindow):
	rooms = []
	def __init__(self, clientID, master=None):
		QtWidgets.QMainWindow.__init__(self, master)
		self.setWindowTitle('VLSync - Choose a room')
		self.createUI()
		self.timer = QtCore.QTimer(self)
		self.timer.setInterval(2500)
		self.timer.timeout.connect(self.updateList)
		self.timer.start()
		self.clientID = clientID
		self.updateList()

	def updateList(self):
		a = requests.get('https://mc.nqind.com/vlsync/rooms.php')
		self.rooms = a.json()['rooms']
		self.roomList.clear()
		for room in self.rooms:
			self.roomList.addItem(room['name'])

	def roomSelected(self, room):
		print(type(room))
		roomID = False
		for i in self.rooms:
			if room.text() == i['name']:
				roomID = int(i['id'])
		if not roomID:
			QtWidgets.QMessageBox(QtWidgets.QMessageBox.Icon.Critical, 'Internal error', 'Invalid room').exec()
			return
		username = self.usernameBox.text()
		if not username:
			QtWidgets.QMessageBox(QtWidgets.QMessageBox.Icon.Critical, 'Bruh moment', 'Enter a username, dangit').exec()
			return
		print(f'Joining room ID {i["id"]}')
		a = requests.post('https://mc.nqind.com/vlsync/join.php', json={'client':self.clientID, 'id':roomID, 'username':username})
		if a.json()['response'] == 200:
			self.close()
			player = Player(self.clientID, {'id': roomID, 'users': a.json()['users']})
			player.show()
			player.resize(640, 480)
		else:
			QtWidgets.QMessageBox(QtWidgets.QMessageBox.Icon.Critical, 'Server error', a.json()['err']).exec()

	def createRoom(self):
		username = self.usernameBox.text()
		if not username:
			QtWidgets.QMessageBox(QtWidgets.QMessageBox.Icon.Critical, 'Bruh moment', 'Enter a username, dangit').exec()
			return
		a = requests.post('https://mc.nqind.com/vlsync/createRoom.php', json={'client':self.clientID, 'username':username})
		print(a.text)
		self.roomID = a.json()['id']
		print(f'Joining CREATED room ID {i["id"]}')
		self.close()
		player = Player(self.clientID, {'id': roomID, 'users': a.json()['users']})
		player.show()
		player.resize(640, 480)

	def createUI(self):
		self.widget = QtWidgets.QWidget(self)
		self.setCentralWidget(self.widget)
		self.hthing = QtWidgets.QVBoxLayout()
		self.roomList = QtWidgets.QListWidget()
		self.hthing.addWidget(self.roomList)
		self.usernameBox = QtWidgets.QLineEdit()
		self.usernameBox.setPlaceholderText('Username...')
		self.hthing.addWidget(self.usernameBox)
		self.createRoomButton = QtWidgets.QPushButton('Create Room')
		self.hthing.addWidget(self.createRoomButton)
		self.createRoomButton.clicked.connect(self.createRoom)
		self.widget.setLayout(self.hthing)
		self.roomList.itemDoubleClicked.connect(self.roomSelected)

class Player(QtWidgets.QMainWindow):
	endSync = pyqtSignal()
	def __init__(self, clientID, roomInfo, master=None):
		QtWidgets.QMainWindow.__init__(self, master)
		self.setWindowTitle('VLSync thing please help')
		self.instance = vlc.Instance()
		self.mediaplayer = self.instance.media_player_new()
		self.createUI()
		self.isPaused = False
		self.starttime = 0
		self.room = roomInfo
		self.roomID = roomInfo['id']
		self.clientID = clientID
		self.threadStart()
		event_manager = self.mediaplayer.event_manager()
		event_manager.event_attach(vlc.EventType.MediaPlayerPlaying, self.log_delay)

	def log_delay(self, _):
		print(time.monotonic() - self.starttime)

	def sync(self, force_sync=False):
		b = requests.post('https://mc.nqind.com/vlsync/heartbeat.php', json={'client': self.clientID, 'id': self.room['id']}).json()
		a = b['room_state']
		print(f'[SYNC] Response: {b}')
		self.userslabel.setText(f'Users: {"".join([i + ", " for i in b["users"]])}')
		# adjust for current timecode if not paused
		timecode = a['timecode'] + (0 if a['paused'] else a['offset'])
		time_diff = self.mediaplayer.get_time() - (timecode * 1000)
		print(f'[SYNC] Diff: {str(time_diff)}ms')
		if force_sync:
			self.mediaplayer.play()
		if abs(time_diff) > 2500 or not self.isPaused == a['paused'] or force_sync:
			self.timeshift(timecode, a['paused'])
	
	def timeshift(self, timecode, paused):
		targetTime = int(timecode * 1000)
		print(f'[SYNC] Shifting to {targetTime}')
		self.mediaplayer.set_time(targetTime)
		if not self.isPaused == paused:
			self.PlayPause()
			print('[SYNC] Toggled pause')
	
	def keyPressEvent(self, ev):
		if ev.key() == 32:
			self.PlayPause()
			self.updateUI()
		elif ev.key() == 70:
			print('fullscreen')
			#self.mediaplayer.set_fullscreen(0 if self.mediaplayer.get_fullscreen == 1 else 1)
			self.widget.showFullScreen()
	
	def threadStart(self):
		# Remember when we were multithreaded?
		# Yeah, me neither
		self.syncTimer = QtCore.QTimer(self)
		self.syncTimer.setInterval(2500)
		self.syncTimer.timeout.connect(self.sync)
		self.syncTimer.start()

	def closeAll(self):
		self.syncTimer.stop()
		self.close()

	def createUI(self):
		self.widget = QtWidgets.QWidget(self)
		self.setCentralWidget(self.widget)
		# In this widget, the video will be drawn
		self.videoframe = QtWidgets.QFrame()
		self.palette = self.videoframe.palette()
		self.palette.setColor (QtGui.QPalette.Window, QtGui.QColor(0,0,0))
		self.videoframe.setPalette(self.palette)
		self.videoframe.setAutoFillBackground(True)
		self.positionslider = QtWidgets.QSlider(QtCore.Qt.Horizontal, self)
		self.positionslider.setToolTip('Position')
		self.positionslider.setMaximum(1000)
		self.positionslider.setTracking(False)
		self.positionslider.sliderMoved.connect(self.markPosition)
		self.positionslider.sliderReleased.connect(self.setPosition)
		self.hbuttonbox = QtWidgets.QHBoxLayout()
		self.forcesync = QtWidgets.QPushButton('Sync')
		self.hbuttonbox.addWidget(self.forcesync)
		self.forcesync.clicked.connect(lambda: self.sync(True))
		self.playbutton = QtWidgets.QPushButton('Play')
		self.hbuttonbox.addWidget(self.playbutton)
		self.playbutton.clicked.connect(self.PlayPause_andupdateserver)
		self.stopbutton = QtWidgets.QPushButton('Stop')
		self.hbuttonbox.addWidget(self.stopbutton)
		self.stopbutton.clicked.connect(self.Stop)
		self.loadbutton = QtWidgets.QPushButton('Load')
		def openfile():
			print('Opening file.')
			self.OpenFile()
		self.loadbutton.clicked.connect(openfile)
		self.hbuttonbox.addWidget(self.loadbutton)
		self.userslabel = QtWidgets.QLabel('No users')
		self.hbuttonbox.addWidget(self.userslabel)
		self.exitbutton = QtWidgets.QPushButton('Exit')
		self.exitbutton.clicked.connect(self.closeAll)
		self.hbuttonbox.addWidget(self.exitbutton)
		self.hbuttonbox.addStretch(1)
		self.volumeslider = QtWidgets.QSlider(QtCore.Qt.Horizontal, self)
		self.volumeslider.setMaximum(100)
		self.volumeslider.setValue(self.mediaplayer.audio_get_volume() or 100)
		self.volumeslider.setToolTip('Volume')
		self.hbuttonbox.addWidget(self.volumeslider)
		self.volumeslider.valueChanged.connect(self.setVolume)
		self.setVolume(self.volumeslider.value())
		self.vboxlayout = QtWidgets.QVBoxLayout()
		self.vboxlayout.addWidget(self.videoframe)
		self.vboxlayout.addWidget(self.positionslider)
		self.vboxlayout.addLayout(self.hbuttonbox)
		self.widget.setLayout(self.vboxlayout)
		self.timer = QtCore.QTimer(self)
		self.timer.setInterval(200)
		self.timer.timeout.connect(self.updateUI)

	def PlayPause_andupdateserver(self):
		print('[SYNC] Toggling Pause to server!')
		r = requests.post('https://mc.nqind.com/vlsync/manageRoom.php', json={'client': self.clientID,'id': self.roomID, 'action':'time','args':{'timecode': self.mediaplayer.get_time() / 1000, 'paused': not self.isPaused}})
		self.PlayPause()

	def PlayPause(self):
		if self.mediaplayer.is_playing():
			self.mediaplayer.pause()
			self.playbutton.setText('Play')
			self.isPaused = True
		else:
			if self.mediaplayer.play() == -1:
				self.OpenFile()
			time.sleep(0.5)
			self.playbutton.setText('Pause')
			self.timer.start()
			self.isPaused = False
			self.starttime = time.monotonic()

	def Stop(self):
		self.mediaplayer.stop()
		self.playbutton.setText('Play')

	def OpenFile(self, filename=None):
		if filename is None:
			filename = QtWidgets.QFileDialog.getOpenFileName(self, 'Open File', os.path.expanduser('~'), 'Video files (*.mkv *.mp4);;Images (*.png *.xpm *.jpg)')
			if filename is not None:
				filename = filename[0]
		if not filename:
			return
		self.media = self.instance.media_new(filename)
		self.mediaplayer.set_media(self.media)
		self.media.parse()
		self.setWindowTitle(self.media.get_meta(0))
		# Why the frick is this platform-specific
		if sys.platform.startswith('linux'): # X only, fools
			self.mediaplayer.set_xwindow(self.videoframe.winId())
		elif sys.platform == 'win32':
			self.mediaplayer.set_hwnd(self.videoframe.winId())
		elif sys.platform == 'darwin':
			# If this is actually mecb00k compatible, I will shame them all
			[print('mecb00k user', file=sys.stderr) for i in range(1, 500)]
			self.mediaplayer.set_nsobject(self.videoframe.winId())

	def setVolume(self, Volume):
		self.mediaplayer.audio_set_volume(Volume)

	def markPosition(self, position):
		self.position = position

	def setPosition(self):
		self.mediaplayer.set_position(self.position / 1000.0)
		newTimecode = int(self.mediaplayer.get_time() / 1000)
		print(f'[SYNC] NEW TIMECODE: {newTimecode}')
		r = requests.post('https://mc.nqind.com/vlsync/manageRoom.php', json={'client': self.clientID,'id': self.roomID, 'action':'time','args':{'timecode': newTimecode, 'paused': self.isPaused}})
		if r.status_code == 200:
			if not r.json()['response'] == 200:
				QtWidgets.QMessageBox(QtWidgets.QMessageBox.Icon.Critical, f'Server error {str(r.json()["response"])}', r.json()['err']).exec()
		else:
			QtWidgets.QMessageBox(QtWidgets.QMessageBox.Icon.Critical, 'Server error', f'Server returned {str(r.json()["response"])}, is it up?').exec()

	def updateUI(self):
		if not self.positionslider.isSliderDown():
			self.positionslider.setValue(int(self.mediaplayer.get_position() * 1000))
		if not self.mediaplayer.is_playing():
			self.timer.stop()
			if not self.isPaused:
				# after the video finished, the play button stills shows
				# "Pause", not the desired behavior of a media player
				# this will fix it
				self.Stop()

if __name__ == "__main__":
	clientID = base64.b64encode(os.urandom(20)).decode('ascii')
	app = QApplication(sys.argv)
	roomChooser = RoomChooser(clientID)
	roomChooser.show()
	roomChooser.resize(1000, 1000)
	#player = Player(clientID, {})
	#player.show()
	#player.resize(640, 480)
	if sys.argv[1:]:
		print(sys.argv[1])
		player.OpenFile(sys.argv[1])
	sys.exit(app.exec_())
