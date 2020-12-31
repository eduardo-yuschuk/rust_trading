extern crate common;
extern crate binary_io;
extern crate configuration;
extern crate fs;

fn main() {
    
    //let quotes_tree_root = configuration::get_quotes_tree_root();
    //std::fs::remove_dir_all(quotes_tree_root).unwrap();
            
    let bin_quotes_file_path = configuration::get_bin_quotes_file_path();
    
    //binary_io::read_quotes_from_bin(bin_quotes_file_path, &mut |quote: common::Quote| {
    //    fs::insert_quote(&quote);
    //}, 3);

    binary_io::read_all_quotes_from_bin(&bin_quotes_file_path, &common::Ticker::EURUSD, &mut |quote: common::Quote, ticker: &common::Ticker| {
        fs::insert_quote(&quote, &ticker);
    });
}
