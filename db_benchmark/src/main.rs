extern crate configuration;
use configuration::get_bin_quotes_file_path;
extern crate common;
use common::Quote;
use common::Ticker;
extern crate binary_io;
use binary_io::read_all_quotes_from_bin;
extern crate db;
extern crate postgres;
use postgres::{Connection, TlsMode};
//#[macro_use]
extern crate postgres_shared;
extern crate args;
extern crate getopts;
use getopts::Options;
use std::env;
use std::time::Instant;

macro_rules! or_panic {
    ($e:expr) => {
        match $e {
            Ok(ok) => ok,
            Err(err) => panic!("{:#?}", err),
        }
    };
}

pub fn load_binary_data_to_db() {
    let conn = or_panic!(Connection::connect(db::POSTGRESQL_CONNECTION_STRING, TlsMode::None));
    db::ensure_db_support(&conn);
    let bin_quotes_file_path = get_bin_quotes_file_path();
    let mut count = 0i32;
    let trans = or_panic!(conn.transaction());
    let stmt = or_panic!(trans.prepare(
        "INSERT INTO quote (ticker, time, ask, bid, ask_size, bid_size) VALUES ($1, $2, $3, $4, $5, $6)"
    ));
    read_all_quotes_from_bin(&bin_quotes_file_path, &Ticker::EURUSD, &mut |q: Quote, ticker: &Ticker| {
        let ticker_as_int = /*q.*/ticker.clone() as i16;
        stmt.execute(&[&ticker_as_int, &q.time, &q.ask, &q.bid, &q.ask_size, &q.bid_size]).unwrap();
        if count % 100000 == 0 {
            println!("count {}", count);
        }
        count += 1;
    });
    assert!(trans.commit().is_ok());
    println!("count {}", count);
}

pub fn read_db_data() {
    println!("Reading data...");
    let conn = or_panic!(Connection::connect(db::POSTGRESQL_CONNECTION_STRING, TlsMode::None));
    let mut readed_count = 0i32;
    db::read_all_quotes(&common::Ticker::EURUSD, &conn, &mut |_q: Quote| {
        if readed_count % 100000 == 0 {
            println!("readed count {}", readed_count);
        }
        readed_count += 1;
    });
    println!("readed count {}", readed_count);
}

pub fn read_first_db_data(count: i64) {
    println!("Reading data...");
    let conn = or_panic!(Connection::connect(db::POSTGRESQL_CONNECTION_STRING, TlsMode::None));
    let mut readed_count = 0i64;
    db::read_first_quotes(&common::Ticker::EURUSD, count, &conn, &mut |_q: Quote| {
        if readed_count % 100000 == 0 {
            println!("readed count {}", readed_count);
        }
        readed_count += 1;
    });
    println!("readed count {}", readed_count);
}

////////////////////////////////////////////////////////////////////////////////
/// arguments

fn parse_args() -> getopts::Matches {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("", "populate_db", "Load data from binary file to db");
	opts.optflag("", "help", "Print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("help") {
        print_usage(&program, opts);
        std::process::exit(0);
    }
	return matches
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

////////////////////////////////////////////////////////////////////////////////
/// main

fn main() {
    let now = Instant::now();

    let arguments = parse_args();

	let populate_db = arguments.opt_present("populate_db");
    if populate_db {
        load_binary_data_to_db();
    }
    
    read_db_data();
    //read_first_db_data(2_000_000i64);

    let elapsed = now.elapsed();
    println!("Elapsed time: {}.{}", elapsed.as_secs(), elapsed.subsec_millis());
}
