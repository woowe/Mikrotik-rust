
extern crate router_os;

use router_os::ApiRos;


use std::net::TcpStream;
use std::io::BufRead;
use std::io;

fn get_line() -> String {
    let stdin_u = io::stdin();
    let mut stdin = stdin_u.lock();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
	line.pop();
    return line;
}

fn main() {
	let mut stream = TcpStream::connect("192.168.1.1:8728").unwrap();

	let mut apiros = ApiRos::new(&mut stream);
	apiros.login("admin".to_string(), "admin".to_string());

	let mut input_sentence: Vec<String> = Vec::new();
	let mut has_written = false;
	let mut was_command = false;

	println!("Type '#quit#' to exit program");

	'main_loop: loop {
		if has_written {
			'reply_loop: loop {
				let replies = apiros.read_sentence();
				if replies.len() == 0 {
					continue;
				}
				if replies[0] == "!done" {
					has_written = false;
					break 'reply_loop;
				}
			}
		}else {
			let input = get_line();

			if &input[..] == "#quit#" {
				break 'main_loop;
			}

			if &input[..] == "" && was_command {
				apiros.write_sentence(input_sentence.clone());
				input_sentence.clear();
				was_command = false;
				has_written = true;
			}else {
				input_sentence.push(input);
				was_command = true;
				has_written = false;
			}
		}
	}

	println!("Goodbye!");
}
