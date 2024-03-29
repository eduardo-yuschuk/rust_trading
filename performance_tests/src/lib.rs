#[cfg(test)]
mod tests {

    #[derive(Clone, Debug)]
    pub enum Ticker {
        EURUSD,
        AUDUSD,
        UNKNOWN,
    }

    #[derive(Clone, Debug)]
    pub struct Quote {
        pub ticker: Ticker,
        pub time: i64,
        pub ask: f32,
        pub bid: f32,
        pub ask_size: f32,
        pub bid_size: f32,
    }

    #[derive(Clone, Debug)]
    pub struct FastQuote {
        pub ticker: Ticker,
        pub time: i64,
        pub ask: i32,
        pub bid: i32,
        pub ask_size: i32,
        pub bid_size: i32,
    }

    impl PartialEq for Quote {
        fn eq(&self, other: &Quote) -> bool {
            (self.ticker.clone() as i32) == (other.ticker.clone() as i32) && 
            self.time == other.time && 
            self.ask == other.ask && 
            self.bid == other.bid && 
            self.ask_size == other.ask_size && 
            self.bid_size == other.bid_size
        }
    }

    impl PartialEq for FastQuote {
        fn eq(&self, other: &FastQuote) -> bool {
            (self.ticker.clone() as i32) == (other.ticker.clone() as i32) && 
            self.time == other.time && 
            self.ask == other.ask && 
            self.bid == other.bid && 
            self.ask_size == other.ask_size && 
            self.bid_size == other.bid_size
        }
    }

    pub fn interpret_ticker(text: &str) -> Ticker {
        match text {
            "EURUSD"	=> Ticker::EURUSD,
            "AUDUSD"	=> Ticker::AUDUSD,
            _			=> Ticker::UNKNOWN,
        }
    }
/*
    pub fn parse_quote(text: &str) -> Quote {
        //let msg_string = format!("ticker={}@time={}@ask={}@bid={}@askSize={}@bidSize={}", 
        //ticker, quote.time, quote.ask, quote.bid, quote.ask_size, quote.bid_size);
        let parts: Vec<&str> = text.split('@').collect();
        let ticker_parts:	Vec<&str> = parts[0].split('=').collect();
        let time_parts:		Vec<&str> = parts[1].split('=').collect();
        let ask_parts:		Vec<&str> = parts[2].split('=').collect();
        let bid_parts:		Vec<&str> = parts[3].split('=').collect();
        let ask_size_parts:	Vec<&str> = parts[4].split('=').collect();
        let bid_size_parts: Vec<&str> = parts[5].split('=').collect();
        let quote = Quote {
            ticker:		interpret_ticker(ticker_parts[1]),
            time:		time_parts		[1].parse::<i64>().unwrap(),
            ask:		ask_parts		[1].parse::<f32>().unwrap(),
            bid:		bid_parts		[1].parse::<f32>().unwrap(),
            ask_size:	ask_size_parts	[1].parse::<f32>().unwrap(),
            bid_size:	bid_size_parts	[1].parse::<f32>().unwrap(),
        };
        return quote;
    }
*/
    fn parse_fast_quote(text: &str) -> FastQuote {
        //let msg_string = format!("ticker={}@time={}@ask={}@bid={}@askSize={}@bidSize={}", 
        //ticker, quote.time, quote.ask, quote.bid, quote.ask_size, quote.bid_size);
        let parts: Vec<&str> = text.split('@').collect();
        let ticker_parts:	Vec<&str> = parts[0].split('=').collect();
        let time_parts:		Vec<&str> = parts[1].split('=').collect();
        let ask_parts:		Vec<&str> = parts[2].split('=').collect();
        let bid_parts:		Vec<&str> = parts[3].split('=').collect();
        let ask_size_parts:	Vec<&str> = parts[4].split('=').collect();
        let bid_size_parts: Vec<&str> = parts[5].split('=').collect();
        let quote = FastQuote {
            ticker:		interpret_ticker(ticker_parts[1]),
            time:		time_parts		[1].parse::<i64>().unwrap(),
            ask:		ask_parts		[1].parse::<i32>().unwrap(),
            bid:		bid_parts		[1].parse::<i32>().unwrap(),
            ask_size:	ask_size_parts	[1].parse::<i32>().unwrap(),
            bid_size:	bid_size_parts	[1].parse::<i32>().unwrap(),
        };
        return quote;
    }
/*
    #[test]
    fn one_million_quote_parsings() {

        for i in 0..1000000 {
            
            let quote = Quote { 
                ticker: Ticker::EURUSD,
                time: i as i64,
                ask: i as f32,
                bid: i as f32,
                ask_size: i as f32,
                bid_size: i as f32,
            };
            
            let msg_string = format!("ticker={:?}@time={}@ask={}@bid={}@askSize={}@bidSize={}", 
                quote.ticker, quote.time, quote.ask, quote.bid, quote.ask_size, quote.bid_size);
            
            let parsed_quote = parse_quote(&msg_string);
            
            assert_eq!(parsed_quote, quote);
        }
    }
*/
    #[test]
    fn one_million_fast_quote_parsings() {

        let mut quote = FastQuote { 
            ticker: Ticker::EURUSD,
            time:       0i64,
            ask:        0i32,
            bid:        0i32,
            ask_size:   0i32,
            bid_size:   0i32,
        };

        //let format1: &'const str = "ticker={:?}@time={}@ask={}@bid={}@askSize={}@bidSize={}";

        for i in 0..1000000 {  
            
            quote.time = i as i64;
            quote.ask = i;
            quote.bid = i;
            quote.ask_size = i;
            quote.bid_size = i;

            let msg_string = format!(
                "ticker={:?}@time={}@ask={}@bid={}@askSize={}@bidSize={}", 
                quote.ticker, quote.time, quote.ask, quote.bid, quote.ask_size, quote.bid_size);
            
            let parsed_quote = parse_fast_quote(&msg_string);
            
            assert_eq!(parsed_quote, quote);
        }

        assert_eq!(quote.ask, 1000000-1);
    }
}
