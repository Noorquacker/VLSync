# VLSync backend

## Prerequisites

You must have the following:

- MySQL/MariaDB server
- PHP 7+ website

Basically the LAMP/LEMP stack. We run nginx on Ubuntu 20.04 with MariaDB for reference

## Installing

1. Make a MySQL user with permission to its own database
2. Either use the phpMyAdmin import function on the database and import `vlsync.sql`, or open a MySQL terminal and use the database and then run the SQL script.
3. Copy `config.php.example` to `config.php` and change its settings
4. Pray it works

## Known Issues

- Rooms are not deleted automatically when all players disconnect
- Users still show up as in rooms even when they leave
	- Meaning if a user crashes while in a room and tries to rejoin, there are 2 of them in the room
