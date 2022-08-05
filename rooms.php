<?php
require 'config.php';


if($_SERVER['REQUEST_METHOD'] === 'GET') {
	header('Content-Type: application/json');
	$table = $mysqli->query('select `id`, `name`, `movie`, `sha1`, `password` from `rooms`');
	$rooms = array();
	// FOR CLIENT DEBUG USE ONLY
	if(true) {
		$rooms = array(array('id' => 'bruh', 'name' => 'nameHere', 'movie' => 'movieHere', 'sha1' => null, 'password' => false));
	}
	while($row = $table->fetch_assoc()) {
		$row['password'] = $row['password'] !== null;
		array_push($rooms, $row);
	}
	$resp = array('response' => 200, 'stuff' => null, 'rooms' => $rooms);
	echo json_encode($resp);
	$mysqli->close();
}
else {
	header('Content-Type: application/json');
	$resp = array('response' => 400, 'err' => 'rooms.php only allows GET');
	echo json_encode($resp);
}
