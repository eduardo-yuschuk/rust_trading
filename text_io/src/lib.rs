use std::io::BufRead;
use std::io::BufReader;
extern crate common;
use std::fs::File;

#[allow(bare_trait_objects)]

///
/// lee un archivo .csv conteniendo quotes
/// convierte cada línea en un quote
/// invoca a una función recibida por parámetro entregando cada quote obtenido
///
pub fn read_quotes_from_csv(path: &str, function: &mut FnMut(common::Quote), max_quotes_count: i32) {
	match File::open(path) {
		Ok(file) => {
			let reader = BufReader::new(file);
			let mut header_readed = false;
			let mut quotes_count = 0i32;
			for line in reader.lines() {
				if header_readed {
					match line {
						Ok(text) => {
							//2018.01.01 22:00:08.661,1.20143,1.20102,0.75,1.5
							let parts: Vec<&str> = text.split(',').collect();
							let quote = common::Quote {
								//ticker:		common::Ticker::EURUSD,
								time:		common::build_millis(parts[0]),
								ask:		parts[1].parse::<f32>().unwrap(),
								bid:		parts[2].parse::<f32>().unwrap(),
								ask_size:	parts[3].parse::<f32>().unwrap(),
								bid_size:	parts[4].parse::<f32>().unwrap(),
							};
							function(quote);
							quotes_count += 1;
							if quotes_count == max_quotes_count {
								break;
							}
						},
						Err(e) => { println!("error reading line: {}", e) }
					}
				} else {
					header_readed = true;
				}
			}
		},
		Err(e) => { println!("error opening {}: {}", path, e) }
	}
}
#[allow(bare_trait_objects)]
///
/// lee un archivo .csv conteniendo quotes
/// convierte cada línea en un quote
/// invoca a una función recibida por parámetro entregando cada quote obtenido
///
pub fn read_all_quotes_from_csv(path: &str, function: &mut FnMut(common::Quote)) {
	match File::open(path) {
		Ok(file) => {
			let reader = BufReader::new(file);
			let mut header_readed = false;
			for line in reader.lines() {
				if header_readed {
					match line {
						Ok(text) => {
							//2018.01.01 22:00:08.661,1.20143,1.20102,0.75,1.5
							let parts: Vec<&str> = text.split(',').collect();
							let quote = common::Quote {
								//ticker:		common::Ticker::EURUSD,
								time:		common::build_millis(parts[0]),
								ask:		parts[1].parse::<f32>().unwrap(),
								bid:		parts[2].parse::<f32>().unwrap(),
								ask_size:	parts[3].parse::<f32>().unwrap(),
								bid_size:	parts[4].parse::<f32>().unwrap(),
							};
							function(quote);
						},
						Err(e) => { println!("error reading line: {}", e) }
					}
				} else {
					header_readed = true;
				}
			}
		},
		Err(e) => { println!("error opening {}: {}", path, e) }
	}
}

#[cfg(test)]
mod tests {
	extern crate configuration;
	extern crate common;
	use read_quotes_from_csv;
	use read_all_quotes_from_csv;

	#[test]
    fn it_reads_one_line() {
		let mut counter = 0;
		read_quotes_from_csv(&configuration::get_text_quotes_file_path(), &mut |_quote: common::Quote| {
			counter += 1;
		}, 1);
        assert_eq!(counter, 1);
    }
	
	#[test]
    fn it_reads_two_lines() {
		let mut counter = 0;
		read_quotes_from_csv(&configuration::get_text_quotes_file_path(), &mut |_quote: common::Quote| {
			counter += 1;
		}, 2);
        assert_eq!(counter, 2);
    }

	#[test]
    fn it_reads_all_lines() {
		let mut counter = 0;
		read_all_quotes_from_csv(&configuration::get_text_quotes_file_path(), &mut |_quote: common::Quote| {
			counter += 1;
		});
        assert_eq!(counter, 12160515);
    }
}
