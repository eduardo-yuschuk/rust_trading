extern crate configuration;
use configuration::get_bin_quotes_file_path;
extern crate common;
use common::Quote;
extern crate binary_io;
use binary_io::read_quotes_from_bin;
//use std::time::Instant;

fn main() {
    let bin_quotes_file_path = get_bin_quotes_file_path();
    
    read_quotes_from_bin(&bin_quotes_file_path, &mut |quote: Quote| {
        quote.println();
    }, 4i32);
}