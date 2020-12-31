///
/// imprime los quotes del archivo binario
/// 

extern crate configuration;
use configuration::get_bin_quotes_file_path;
extern crate common;
use common::Quote;
use common::Ticker;
extern crate binary_io;
use binary_io::read_all_quotes_from_bin;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let bin_quotes_file_path = get_bin_quotes_file_path();
    
    let mut q: Option<Quote> = None;
    let mut count = 0i32;
    
    read_all_quotes_from_bin(&bin_quotes_file_path, &Ticker::EURUSD, &mut |quote: Quote, _ticker: &Ticker| {
        match q {
            Some(_) => {},
            None => {
                print!("First: ");
                quote.println();
            }
        }
        q = Some(quote.clone());
        count += 1;
    });
    
    print!("Last:  ");
    q.unwrap().println();
    
    println!("Quotes count: {}", count);

    let elapsed = now.elapsed();
    println!("Elapsed time: {}.{}", elapsed.as_secs(), elapsed.subsec_millis());
}
