use std::fs::File;
//use std::io::prelude::*;
extern crate configuration;
use configuration::{get_text_quotes_file_path, get_bin_quotes_file_path};
extern crate common;
use common::Quote;
extern crate text_io;
use text_io::read_all_quotes_from_csv;
//#[macro_use]
//extern crate serde_derive;
extern crate serde;
use std::io::BufWriter;
//use std::io::Write;
extern crate bincode;

///
/// convierte quotes (ask/bid) de formato texto a binario.
///
pub fn convert_quotes() {
	
	let quotes_file_path = get_text_quotes_file_path();
	let bin_quotes_file_path = get_bin_quotes_file_path();
	
    println!("converting quotes from: \n{} \nto: \n{}", quotes_file_path, bin_quotes_file_path);

    match File::create(bin_quotes_file_path) {
        Ok(file) => {
            let mut writer = BufWriter::new(file);
            println!("binary quotes file created...");
            let mut readed_counter = 0;
            let mut serialized_counter = 0;
            read_all_quotes_from_csv(&get_text_quotes_file_path(), &mut |quote: Quote| {
                readed_counter += 1;
                match bincode::serialize_into(&mut writer, &quote) {
                    Ok(_) => {
                        serialized_counter += 1;
                    },
                    Err(e) => println!("Error serializing quote {}", e)
                }
                if readed_counter % 1000000 == 0 {
                    println!("{} quotes readed, {} quotes serialized", readed_counter, serialized_counter);
                }
            });
            println!("{} quotes readed, {} quotes serialized", readed_counter, serialized_counter);
        },
        Err(e) => panic!("Error creating file {}", e)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
