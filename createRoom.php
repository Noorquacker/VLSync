<?php
require 'config.php';


if($_SERVER['REQUEST_METHOD'] === 'POST') {
	$req = use_json();

	$statement = $mysqli->prepare('insert into `rooms` values (?, ?, NULL, NULL, ?, ?)');
	try{
		$roomName = $req['name'];
		$username = $req['username'];
		$client = base64_decode($req['client']);
		$passwd = $req['pass'];
		if($client === '' || $username === '') {
			throw new Exception('Missing client/username in request json. You posted ' . file_get_contents('php://input'));
		}
		if($roomName === '' || $roomName === null) {
			$roomName = $username . '\'s Room';
		}
		if($passwd === '') {
			$passwd = null;
		}
	}
	catch(Exception $e) {
		$resp = array('response' => 400, 'err' => $e->getMessage());
		echo json_encode($resp);
		die();
	}
	$ids = $mysqli->query('select `id` from `rooms`')->fetch_all();
	$id = 1;
	while(true) {
		$idOld = $id;
		foreach($ids as $i) {
			if(intval($i[0]) == $id) {
				$id = intval($i[0]) + 1;
			}
		}
		if($idOld === $id) {
			break;
		}
	}
	$statement->bind_param('isss', $id, $roomName, $client, $passwd);
	if($statement->execute() === false || $mysqli->query("insert into `room_state` (`id`) values ($id)") === false || $mysqli->query("insert into `members` (`id`, `client`, `username`) values ($id, '$client', '$username')") === false) {
		$resp = array('response' => 500, 'err' => $mysqli->error);
		echo json_encode($resp);
		die();
	}
	$resp = array('response' => 200, 'id' => $id, 'name' => $roomName, 'owner' => base64_encode($client), 'pass' => $passwd);
	echo json_encode($resp);
	$mysqli->close();
}
else {
	echo 'The frick you doin\'?';
}
