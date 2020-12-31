extern crate text_io;
use text_io::read_all_quotes_from_csv;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let mut counter = 0;
    read_all_quotes_from_csv(&configuration::get_text_quotes_file_path(), &mut |_quote: common::Quote| {
        if counter % 1000000 == 0 {
           println!("{}", counter);
        }
        counter += 1;
    });
    println!("{}", counter);

    let elapsed = now.elapsed();
    println!("Elapsed time: {}.{}", elapsed.as_secs(), elapsed.subsec_millis());
}
