use std::fs::File;
use std::io::BufWriter;
extern crate common;
extern crate configuration;
use common::Bar;
use common::Quote;
use common::Timeframe;
use common::Ticker;
extern crate binary_io;
use binary_io::read_all_quotes_from_bin;
extern crate bincode;

///
/// construye bars y las almacena en un único archivo
///
pub fn build_bars(bin_quotes_file_path: &str, bin_bars_file_path: &str, timeframe: Timeframe, ticker: &Ticker) {
    println!("creating bars file {}", bin_bars_file_path);
    match File::create(bin_bars_file_path) {
        Ok(file) => {
            let mut writer = BufWriter::new(file);
            println!("binary bars file created...");
            let mut readed_counter = 0;
            let mut serialized_counter = 0;
            let mut bar = Bar::empty();
            let mut bar_index = 0u32;
            read_all_quotes_from_bin(bin_quotes_file_path, ticker, &mut |quote: Quote, _ticker: &Ticker| {
                //println!("quote.time: {}", quote.time);
                //println!("quote.get_datetime(): {:?}", quote.get_datetime());
                readed_counter += 1;
                let new_bar_index = (quote.time / 1000i64 / timeframe.get_seconds() as i64) as u32;
                //println!("new_bar_index: {}", new_bar_index);
                let is_new_bar = new_bar_index != bar_index;
                if is_new_bar {
                    //bar.println_tf(&timeframe);
                    match bincode::serialize_into(&mut writer, &bar) {
                        Ok(_) => {
                            serialized_counter += 1;
                        }
                        Err(e) => println!("Error serializing bar {}", e),
                    }
                    bar.initialize(new_bar_index, quote.ask);
                } else {
                    //bar.println_tf(&timeframe);
                    bar.update(quote.ask);
                }
                bar_index = new_bar_index;
                if readed_counter % 1000000 == 0 {
                    println!(
                        "{} quotes readed, {} bars serialized",
                        readed_counter, serialized_counter
                    );
                }
            });
            println!(
                "{} quotes readed, {} bars serialized",
                readed_counter, serialized_counter
            );
        }
        Err(e) => panic!("Error creating file {}", e),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
