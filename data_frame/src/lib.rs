/*
#[macro_use]
extern crate serde_derive;
extern crate serde;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DataFrame {
}

impl DataFrame {

	pub fn from_csv(path: &str) -> DataFrame {
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
		DataFrame {
		}
	}
}
*/

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
