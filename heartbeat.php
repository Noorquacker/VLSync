<?php
require 'config.php';


if($_SERVER['REQUEST_METHOD'] === 'POST') {
	$req = use_json();

	try{
		$id = intval($req['id']);
		$client = base64_decode($req['client']);
		if($client === '' || $id === '') {
			throw new Exception('Missing id/client in request json');
		}
	}
	catch(Exception $e) {
		$resp = array('response' => 400, 'err' => $e->getMessage());
		echo json_encode($resp);
		die();
	}
	$ids = $mysqli->query('select `id`, `owner` from `rooms`')->fetch_all();
	$validId = false;
	$authorized = false;
	foreach($ids as $i) {
		if($id === intval($i[0])) {
			$authorized = $i[1] === $client;
			$validId = true;
			break;
		}
	}
	if(!$validId) {
		$resp = array('response' => 400, 'err' => 'Room does not exist (anymore)');
		echo json_encode($resp);
		die();
	}
	$client_esc = $mysqli->real_escape_string($client);
	$mysqli->query("update `members` set `keepAlive` = current_timestamp() where `client` = '$client_esc'");
	$users = array();
	foreach($mysqli->query("select `username` from `members` where `id` = $id")->fetch_all() as $user) {
		array_push($users, $user[0]);
	}
	$room_state = $mysqli->query("select * from room_state where `id` = $id")->fetch_all()[0];
	$modified = DateTime::createFromFormat('Y-m-d H:i:s', $room_state[3])->getTimestamp();
	$offset = (new DateTime('NOW'))->getTimestamp() - $modified;
	$room_state = array('timecode' => floatval($room_state[1]), 'paused' => $room_state[2] === '1', 'modified' => $modified, 'offset' => $offset);
	
	
	$resp = array('response' => 200, 'users' => $users, 'room_state' => $room_state);
	echo json_encode($resp);
	$mysqli->close();
}
else {
	echo 'The frick you doin\'?';
}
