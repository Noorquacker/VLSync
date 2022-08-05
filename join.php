<?php
require 'config.php';


if($_SERVER['REQUEST_METHOD'] === 'POST') {
	$req = use_json();

	try{
		$id = intval($req['id']);
		$username = $req['username'];
		$client = base64_decode($req['client']);
		$password = $req['password'];
		if($client === '' || $id === '') {
			throw new Exception('Missing id/client in request json');
		}
	}
	catch(Exception $e) {
		$resp = array('response' => 400, 'err' => $e->getMessage());
		echo json_encode($resp);
		die();
	}
	$ids = $mysqli->query('select `id`, `password` from `rooms`')->fetch_all();
	$validId = false;
	$authorized = false;
	foreach($ids as $i) {
		if($id === intval($i[0])) {
			$authorized = $password !== null ? $i[1] === $password : true;
			// Yes, this is the wrong way to store passwords.
			// What are you gonna do, leak the passwords?
			$validId = true;
			break;
		}
	}
	if(!$validId) {
		$resp = array('response' => 400, 'err' => 'Invalid room ID');
		echo json_encode($resp);
		die();
	}
	if(!$authorized) {
		$resp = array('response' => 401, 'err' => 'Invalid password');
		echo json_encode($resp);
		die();
	}
	$statement = $mysqli->prepare('insert into `members` (`id`, `client`, `username`) values (?, ?, ?)');
	$statement->bind_param('iss', $id, $client, $username);
	if($statement->execute() === false) {
		$resp = array('response' => 500, 'err' => $mysqli->error);
		echo json_encode($resp);
		die();
	}
	$users = array();
	foreach($mysqli->query("select `username` from `members` where `id` = $id")->fetch_all() as $user) {
		array_push($users, $user[0]);
	}
	$resp = array('response' => 200, 'users' => $users);
	echo json_encode($resp);
	$mysqli->close();
}
else {
	echo 'The frick you doin\'?';
}
