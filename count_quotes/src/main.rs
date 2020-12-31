extern crate configuration;
extern crate common;
extern crate text_io;

fn main() {
	let quotes_file_path = configuration::get_text_quotes_file_path();
    println!("reading {}", quotes_file_path);
	let mut counter = 0;
	text_io::read_all_quotes_from_csv(&quotes_file_path, &mut |_quote: common::Quote| {
		counter += 1;
		//if counter % 100000 == 0 {
		//	println!("{} quotes readed", counter);
		//}
	});
    println!("{} quotes readed", counter);
}
