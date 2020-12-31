#[macro_use]
extern crate chan;
use std::thread;
use std::io;
use std::io::Read;
extern crate postgres;
use postgres::{Connection, TlsMode};
use std::time::Instant;


pub fn tick_boom() {
  let tick = chan::tick_ms(100);
  let boom = chan::after_ms(500);
  loop {
      chan_select! {
          default => { println!("   ."); thread::sleep(std::time::Duration::from_millis(50)); },
          tick.recv() => println!("tick."),
          boom.recv() => { println!("BOOM!"); return; },
      }
  }	
}

pub fn send_recv() {
	let (s, r) = chan::sync(0);
	
	thread::spawn(move || {
		println!("Starting receiver");
		loop {
			chan_select! {
				r.recv() -> val => {
					println!("{:?}", val);
				}
			}
		}
	});
	
	let s1 = s.clone();
	thread::spawn(move || {
		println!("Starting sender 1");
		loop {
			s1.send(1);
			thread::sleep(std::time::Duration::from_millis(100));
		}
	});
	
	let s2 = s.clone();
	thread::spawn(move || {
		println!("Starting sender 2");
		loop {
			s2.send(2);
			thread::sleep(std::time::Duration::from_millis(100));
		}
	});	
}

fn main() {
	
	//tick_boom();
	//send_recv()
	
	ensure_db_support();
	
	let mut counter = 0i32;
	let mut now = Instant::now();
	
	let conn = Connection::connect(POSTGRESQL_CONNECTION_STRING, TlsMode::None).unwrap();

	for i in 0..1000000 {
		let quote = Quote {
			ticker:		Ticker::EURUSD,
			time:		i as i64,
			ask:		0f64,
			bid:		0f64,
			ask_size:	0f64,
			bid_size:	0f64,
		};
		insert_quote(&quote, &conn);
		counter += 1;
		if now.elapsed().as_secs() >= 1 {
			println!("{} samples/sec", counter);
			counter = 0;
			now = Instant::now();
		}
	}

	wait_for_exit();
}

fn wait_for_exit() {
	println!("Press ENTER to exit");
	io::stdin().read(&mut [0u8]).unwrap();
}

const POSTGRESQL_CONNECTION_STRING : &str = "postgres://postgres:postgres@localhost:5432";

///
/// garantiza la existencia del soporte para el almacenamiento de quotes
///
fn ensure_db_support() {
    let conn = Connection::connect(POSTGRESQL_CONNECTION_STRING, TlsMode::None).unwrap();
	conn.execute("DROP TABLE IF EXISTS quote", &[]).unwrap();
    conn.execute("CREATE TABLE quote (
                    ticker		bigint,
                    time		bigint,
                    ask			double precision,
                    bid			double precision,
                    ask_size	double precision,
                    bid_size	double precision,
					PRIMARY KEY (ticker, time)
                  )", &[]).unwrap();
}

///
/// inserta un quote en la base de datos
///
fn insert_quote(quote: &Quote, conn: &Connection) {
	//let conn = Connection::connect(POSTGRESQL_CONNECTION_STRING, TlsMode::None).unwrap();
	let ticker = quote.ticker.clone() as i64;
	conn.execute("INSERT INTO quote (ticker, time, ask, bid, ask_size, bid_size) VALUES ($1, $2, $3, $4, $5, $6)", 
		&[&ticker, &quote.time, &quote.ask, &quote.bid, &quote.ask_size, &quote.bid_size]).unwrap();
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTS

#[derive(Clone, Debug)]
struct Quote {
	ticker: Ticker,
	time: i64,
	ask: f64,
	bid: f64,
	ask_size: f64,
	bid_size: f64,
}

#[derive(Clone, Debug)]
pub enum Ticker {
	EURUSD,
	AUDUSD,
	UNKNOWN,
}


