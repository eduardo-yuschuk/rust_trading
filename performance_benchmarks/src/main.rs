use std::collections::HashMap;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Quote {
	pub ticker: i32,
	pub time: i64,
	pub ask: i32,
	pub bid: i32,
	pub ask_size: i32,
	pub bid_size: i32,
}

impl PartialEq for Quote {
	fn eq(&self, other: &Quote) -> bool {
		self.ticker		== other.ticker		&&
		self.time		== other.time		&& 
		self.ask		== other.ask		&& 
		self.bid		== other.bid		&& 
		self.ask_size	== other.ask_size	&& 
		self.bid_size	== other.bid_size
	}
}

fn main() {
    let iterations = 100;
    let quotes_count = 10000000;
    for _times in 0..iterations {
        let now = Instant::now();
        let mut quotes_by_ticker: HashMap<i32, HashMap<i64, Quote>> = HashMap::new();
        for i in 0..quotes_count {  
            let quote = Quote {
                ticker: 0i32,
                time: i as i64,
                ask: i,
                bid: i,
                ask_size: i,
                bid_size: i,
            };
            insert_quote(quote, &mut quotes_by_ticker, i as usize);
        }
        assert!(quotes_by_ticker.len() == 1);
        //println!("quotesByTicker len: {}", quotes_by_ticker.len());
        for (_ticker, quotes) in quotes_by_ticker.iter() {
            //println!("quotes of ticker {}, len: {}", ticker, quotes.len());
            assert!(quotes.len() == quotes_count as usize);
        }
        let elapsed = now.elapsed();
        println!("Elapsed time: {}.{}", elapsed.as_secs(), elapsed.subsec_millis());
    }
}

fn insert_quote(quote: Quote, quotes_by_ticker: &mut HashMap<i32, HashMap<i64, Quote>>, i: usize) {
    let added = match quotes_by_ticker.get_mut(&quote.ticker) {
        Some(quotes) => {
            assert!(quotes.len() == i);
            quotes.insert(
                quote.time,
                quote.clone(),
            );
            true
        },
        _ => {
            //println!("ticker not found!");
            false
        }
    };
    if !added {
        quotes_by_ticker.insert(
            quote.ticker,
            HashMap::new(),
        );
        insert_quote(quote, quotes_by_ticker, i);
    }
}
