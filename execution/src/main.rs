// TODO:
// persistir los quotes en archivo
// persistir los quotes en db

use std::io;
use std::io::prelude::*;
use std::thread;
use std::net::TcpListener;
use std::net::TcpStream;
use std::time::Instant;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
extern crate time;
extern crate datetime;
extern crate os_info;
extern crate postgres;
use postgres::{Connection, TlsMode};
extern crate args;
extern crate getopts;
use getopts::Options;
use std::env;
//extern crate quotes_file_publisher;
extern crate db;
extern crate common;
extern crate configuration;

////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {
	let arguments = parse_args();

	let use_db = arguments.opt_present("use_db");
	let print_quotes = arguments.opt_present("print_quotes");
	run_quotes_management(use_db, print_quotes);

	run_clients_listener();
	
	thread::sleep(std::time::Duration::from_millis(1000));
	
	

	wait_for_exit();
}

fn wait_for_exit() {
	println!("Press ENTER to exit");
	io::stdin().read(&mut [0u8]).unwrap();
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// ARGUMENTS

fn parse_args() -> getopts::Matches {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("", "run_publisher", "Run demo quotes publisher");
	opts.optflag("", "slow_quotes", "Slow quotes publishing");
	opts.optflag("", "print_quotes", "Print quotes");
    opts.optflag("", "use_db", "Use a database to save the quotes");
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// QUOTES MANAGEMENT

///
/// ejecuta la recepción de quotes a través de un puerto TCPIP
/// gestiona un almacén caliente de quotes
/// ejecuta un servidor TCPIP en un puerto para entregar quotes a demanda
///
fn run_quotes_management(use_db: bool, print_quotes: bool) {
	thread::spawn(move || {
		if use_db {
			let conn = Connection::connect(db::POSTGRESQL_CONNECTION_STRING, TlsMode::None).unwrap();
			db::ensure_db_support(&conn);
		}
		let (tx_new_quotes, rx_new_quotes): (Sender<common::Quote>, Receiver<common::Quote>) = mpsc::channel();
		thread::spawn(move || {
			listen_for_quotes(use_db, tx_new_quotes);
		});
		
		/*
		let url = format!("{}:{}", QUOTES_SERVER_ADDRESS, QUOTES_SERVER_PORT);
		println!("starting quotes server on {}", url);
		match TcpListener::bind(url) {
			Ok(listener) => {
				for connection in listener.incoming() {
					match connection {
						Ok(mut stream) => {
							let tx = tx.clone();
							thread::spawn(move || {
								listen_to_quotes_provider(&mut stream, use_db, tx);
							});
						},
						Err(e) => { println!("receiving connection error: {}", e) }
					}
			    }
			},
			Err(e) => { println!("binding to port error: {}", e) }
		}
		*/
		
		loop {
			let quote: common::Quote = rx_new_quotes.recv().unwrap();
			if print_quotes {
				println!("{:?}", quote);
			}
		}
	});
}

///
/// abre un puerto para la escucha de quotes en formato @ separated
/// propaga los Quote obtenidos de los distintos proveedores a través de un canal (tx)
///
fn listen_for_quotes(use_db: bool, tx: Sender<common::Quote>) {
	let url = format!("{}:{}", configuration::QUOTES_LISTENER_ADDRESS, configuration::QUOTES_LISTENER_PORT);
	println!("starting quotes listener on {}", url);
	match TcpListener::bind(url) {
		Ok(listener) => {
			for connection in listener.incoming() {
				match connection {
					Ok(mut stream) => {
						let tx = tx.clone();
						thread::spawn(move || {
							listen_to_quotes_provider(&mut stream, use_db, tx);
						});
					},
					Err(e) => { println!("receiving connection error: {}", e) }
				}
		    }
		},
		Err(e) => { println!("binding to port error: {}", e) }
	}
}

///
/// atiende a un proveedor de quotes en formato @ separated
/// propaga los Quote obtenidos de un proveedor a través de un canal (tx)
///
fn listen_to_quotes_provider(stream: &mut TcpStream, use_db: bool, tx: Sender<common::Quote>) {
	let mut counter = 0i32;
	let mut now = Instant::now();
	let conn = Connection::connect(db::POSTGRESQL_CONNECTION_STRING, TlsMode::None).unwrap();
	loop {
		let mut buffer = [0; 1];
		match stream.read(&mut buffer) {
			Ok(_) => {
				let length = buffer[0] as usize;
				let mut buffer = vec![0; length];
				match stream.read_exact(&mut buffer) {
					Ok(_) => {
						let response = &[0u8];
						match stream.write(response) {
							Ok(_) => {
								process_provided_quote_bytes(&buffer, use_db, &tx, &conn);
								counter += 1;
								if now.elapsed().as_secs() >= 1 {
									println!("{} samples/sec", counter);
									counter = 0;
									now = Instant::now();
								}
							},
							Err(e) => { println!("write error: {}", e); break; }
						}
					},
					Err(e) => { println!("read (msg) error: {}", e); break; }
				}
			},
			Err(e) => { println!("read (msg size) error: {}", e); break; }
		}
	}
}

///
/// convierte los datos crudos (vector de bytes) a un texto @ separated
/// parsea el texto y genera un Quote
/// almacena el Quote en la db
/// propaga el Quote a través de un canal (tx)
///
fn process_provided_quote_bytes(buffer: &Vec<u8>, use_db: bool, tx: &Sender<common::Quote>, conn: &Connection) {
	match std::str::from_utf8(buffer) {
		Ok(text) => {
			let quote = common::Quote::parse(text);
			if use_db {
				// FIXME el ticker debería venir como info de la propagación
				db::insert_quote(&quote, &common::Ticker::EURUSD, &conn);
			}
			//println!("{:?}", quote);
			// envío el quote al QuotesManager
			match tx.send(quote) {
				Ok(_) => {},
				Err(_) => common::delayed_print("Error al intentar enviar el quote al manager")
			}
		},
		Err(e) => println!("read (msg size) error: {}", e)
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// EXTERNAL CLIENTS LISTENER (QUOTES CONSUMERS)

fn run_clients_listener() {
	thread::spawn(move || {
		listen_for_clients();
	});
}

fn listen_for_clients() {
	println!("Listening for clients");
}

