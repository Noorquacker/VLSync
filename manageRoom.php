<?php
require 'config.php';


if($_SERVER['REQUEST_METHOD'] === 'POST') {
	$req = use_json();

	try{
		$id = intval($req['id']);
		$client = base64_decode($req['client']);
		$action = $req['action'];
		if($client === '' || $id === '' || $action === '') {
			throw new Exception('Missing id/client/action in request json');
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
		$resp = array('response' => 400, 'err' => 'Invalid room ID');
		echo json_encode($resp);
		die();
	}
	if(!$authorized && ($action == 'modify' || $action == 'delete')) {
		$resp = array('response' => 401, 'err' => 'You are not the owner');
		echo json_encode($resp);
		die();
	}
	$client_esc = $mysqli->real_escape_string($client);
	switch($action) {
		case 'modify':
			$mysqli->query("update `members` set `keepAlive` = current_timestamp() where `client` = '$client_esc'");
			$statement = $mysqli->prepare('update `rooms` set `name` = ?, `movie` = ?, `sha1` = ? where `id` = ?');
			$args = $req['args'];
			$statement->bind_param('sssi', $args['roomName'], $args['movieName'], $args['sha1'], $id);
			if($statement->execute() === false) {
				$resp = array('response' => 500, 'err' => $mysqli->error);
				echo json_encode($resp);
				die();
			}
			$resp = array('response' => 200, 'roomName' => $args['roomName']);
			echo json_encode($resp);
			$mysqli->close();
			break;
		case 'delete':
			if(!($mysqli->query("delete from `rooms` where `id` = $id") && $mysqli->query("delete from room_state where id = $id") && $mysqli->query("delete from members where id = $id"))) {
				$resp = array('response' => 500, 'err' => $mysqli->error);
				echo json_encode($resp);
				die();
			}
			$resp = array('response' => 200);
			echo json_encode($resp);
			$mysqli->close();
			break;
		case 'time':
			$mysqli->query("update `members` set `keepAlive` = current_timestamp() where `client` = '$client_esc'");
			$statement = $mysqli->prepare('update `room_state` set `timecode` = ?, `pause` = ?, `modified` = current_timestamp() where `id` = ?');
			$args = $req['args'];
			$paused = $args['paused'] ? 1 : 0; //Thank you pass-by-reference for this line
			$statement->bind_param('sii', $args['timecode'], $paused, $id);
			if($statement->execute() === false) {
				$resp = array('response' => 500, 'err' => $mysqli->error);
				echo json_encode($resp);
				die();
			}
			$resp = array('response' => 200, 'id' => $id, 'timecode' => $args['timecode'], 'paused' => $args['paused']);
			echo json_encode($resp);
			$mysqli->close();
			break;
		default:
			$resp = array('response' => 400, 'err' => 'Invalid action');
			echo json_encode($resp);
			die();
			break;
	}
}
else {
	echo 'The frick you doin\'?';
}
