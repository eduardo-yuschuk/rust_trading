
#[derive(Debug)]
struct Ticker {
    pub id: u32,
    pub name: String
}

#[derive(Debug)]
struct Quote<'a> {
    pub time: u32,
    pub ticker: &'a Ticker
}

fn main() {

    let mut tickers: Vec<Ticker> = vec![
        Ticker { id: 0u32, name: String::from("EUR/USD") },
        Ticker { id: 1u32, name: String::from("CAD/USD") },
        Ticker { id: 2u32, name: String::from("AUD/USD") },
    ];

    for i in 0..10 {
        tickers.push(Ticker {id: i as u32, name: String::from("?")});
    }
    println!("{:?}", tickers);

    let quote = Quote { time: 0u32, ticker: &tickers[0] };
    println!("{:?}", quote);
}
