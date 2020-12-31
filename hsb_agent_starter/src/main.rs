use std::process::Command;
use std::thread;

fn main() {
	loop {
		println!("starting hsb_agent...");
		match Command::new("cd ../hsb_agent; cargo run").status() {
			Ok(_) => { println!("startup completed") },
			Err(e) => {
				println!("error {}", e);
				thread::sleep(std::time::Duration::from_millis(1000));
			}
		}
	}
}
