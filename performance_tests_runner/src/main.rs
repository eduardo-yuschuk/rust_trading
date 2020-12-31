//#[macro_use]
//extern crate num_derive;
extern crate num_traits;

//use num_traits::FromPrimitive;
/*
#[derive(Clone, Debug, FromPrimitive)]
pub enum Ticker {
	EURUSD,
	AUDUSD,
}
*/
#[derive(Clone, Debug)]
pub struct FastQuote {
	pub ticker: i32,
	pub time: i64,
	pub ask: i32,
	pub bid: i32,
	pub ask_size: i32,
	pub bid_size: i32,
}

impl PartialEq for FastQuote {
	fn eq(&self, other: &FastQuote) -> bool {
		//(self.ticker.clone() as i32) == (other.ticker.clone() as i32) && 
		self.ticker == other.ticker &&
		self.time == other.time && 
		self.ask == other.ask && 
		self.bid == other.bid && 
		self.ask_size == other.ask_size && 
		self.bid_size == other.bid_size
	}
}
/*
pub fn interpret_ticker(text: &str) -> Ticker {
	match text {
		"EURUSD"	=> Ticker::EURUSD,
		"AUDUSD"	=> Ticker::AUDUSD,
		_			=> panic!("ticker unknown"),
	}
}

fn parse_fast_quote(text: &str) -> FastQuote {
	let parts: Vec<&str> = text.split('@').collect();
	let ticker_parts:	Vec<&str> = parts[0].split('=').collect();
	let time_parts:		Vec<&str> = parts[1].split('=').collect();
	let ask_parts:		Vec<&str> = parts[2].split('=').collect();
	let bid_parts:		Vec<&str> = parts[3].split('=').collect();
	let ask_size_parts:	Vec<&str> = parts[4].split('=').collect();
	let bid_size_parts: Vec<&str> = parts[5].split('=').collect();
	let quote = FastQuote {
		//ticker:		interpret_ticker(ticker_parts[1]),
		//ticker:		Ticker::from_i32(
		//			ticker_parts	[1].parse::<i32>().unwrap()).unwrap(),
		ticker:		ticker_parts	[1].parse::<i32>().unwrap(),
		time:		time_parts		[1].parse::<i64>().unwrap(),
		ask:		ask_parts		[1].parse::<i32>().unwrap(),
		bid:		bid_parts		[1].parse::<i32>().unwrap(),
		ask_size:	ask_size_parts	[1].parse::<i32>().unwrap(),
		bid_size:	bid_size_parts	[1].parse::<i32>().unwrap(),
	};
	return quote;
}
*/
fn parse_fast_quote(text: &str) -> FastQuote {
	let parts: Vec<&str> = text.split(|c| c == '@' || c == '=').collect();
//	println!("{:?}", parts);
	let quote = FastQuote {
		ticker:		parts[1].parse::<i32>().unwrap(),
		time:		parts[3].parse::<i64>().unwrap(),
		ask:		parts[5].parse::<i32>().unwrap(),
		bid:		parts[7].parse::<i32>().unwrap(),
		ask_size:	parts[9].parse::<i32>().unwrap(),
		bid_size:	parts[11].parse::<i32>().unwrap(),
	};
	return quote;
}

fn main() {

	let mut quote = FastQuote { 
		//ticker: Ticker::EURUSD,
		ticker: 	0i32,
		time:       0i64,
		ask:        0i32,
		bid:        0i32,
		ask_size:   0i32,
		bid_size:   0i32,
	};

	for i in 0..1000000 {  
		
		quote.time = i as i64;
		quote.ask = i;
		quote.bid = i;
		quote.ask_size = i;
		quote.bid_size = i;

		let msg_string = format!(
			"ticker={:?}@time={}@ask={}@bid={}@askSize={}@bidSize={}", 
		    quote.ticker, quote.time, quote.ask, quote.bid, quote.ask_size, 
			quote.bid_size);
		
		let parsed_quote = parse_fast_quote(&msg_string);
		
		assert_eq!(parsed_quote, quote);

		if i % 100000 == 0 {
			println!("{}", i);
		}
	}

	assert_eq!(quote.ask, 1000000-1);
}

