extern crate common;
use common::Ticker;
extern crate configuration;
extern crate binary_io;
use std::fs;

pub fn insert_quote(quote: &common::Quote, ticker: &Ticker) {
    let file_path = quote.get_quotes_file_path(ticker);
    match binary_io::get_all_quotes_from_bin(&file_path, ticker) {
        Ok(mut quotes) => {
            // agrego el quote al vector en el lugar correcto y piso el archivo
            if quote.time > quotes.last().unwrap().time {
                quotes.push(quote.clone());
            } else {
                match quotes.binary_search_by(|q| q.time.cmp(&quote.time)) {
                    Ok(pos) => {
                        quotes[pos] = quote.clone();
                    },
                    Err(pos) => {
                        quotes.insert(pos, quote.clone());
                    }
                }
            }
            binary_io::overwrite_quotes_to_bin(&quotes, &file_path);
        },
        Err(_) => {
            let path = quote.get_quotes_folder_path(ticker);
            match fs::create_dir_all(path) {
                Ok(_) => {
                    // pongo el primer quote en un vector y genero el archivo
                    let mut quotes = vec!();
                    quotes.push(quote.clone());
                    binary_io::write_quotes_to_bin(&quotes, &file_path);
                },
                Err(e) => { panic!("Error creating folders {}", e) }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
