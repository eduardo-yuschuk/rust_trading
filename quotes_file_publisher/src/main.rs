use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
extern crate common;
extern crate os_info;
use std::net::TcpStream;
//use std::io::BufRead;
//use std::io::BufReader;
//use std::io;
use std::io::prelude::*;
extern crate configuration;
extern crate time;
extern crate datetime;
//use datetime::{LocalDateTime, LocalDate, LocalTime, Month};
//use std::fs::File;
//use std::net::TcpListener;
//use std::time::Instant;
extern crate text_io;
//extern crate args;
extern crate getopts;
use getopts::Options;
use std::env;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// IMPL

///
/// ejecuta el consumo de quotes desde un archivo .csv
/// propaga dichos quotes a través de una conexión TCPIP al quotes listener
///
pub fn run_quotes_publisher(slow_quotes: bool, max_quotes_count: i32) {
	println!("Publishing demo quotes");
	let (tx, rx): (Sender<common::Quote>, Receiver<common::Quote>) = mpsc::channel();
	//
	// EJECUCIÓN DEL PUBLICADOR TCPIP (ESCUCHA DENTRO DEL LOOP DE CONEXIÓN LOS QUOTES DEL LECTOR DE CSV)
	//
	thread::spawn(move || {
		publish_quotes(rx);
	});
	//
	// LECTURA DEL CSV Y ENTREGA MEDIANTE EL CHANNEL AL PUBLICADOR TCPIP
	//
	text_io::read_quotes_from_csv(&configuration::get_text_quotes_file_path(), &mut |quote: common::Quote| {
		match tx.send(quote) {
			Ok(_) => {
				if slow_quotes {
					thread::sleep(std::time::Duration::from_millis(1000));
				}
			},
			Err(_) => delayed_print("quote propagation error (reader to publisher)")
		}
	}, max_quotes_count);
}

///
/// se conecta al quotes listener a través de una conexión TCPIP
/// queda esperando quotes provenientes del lector de .csv a través de un canal (rx)
///
fn publish_quotes(rx: Receiver<common::Quote>) {
	let url = format!("{}:{}", configuration::QUOTES_LISTENER_ADDRESS, configuration::QUOTES_LISTENER_PORT);
	match TcpStream::connect(url) {
		Ok(mut stream) => {
			loop {
				let quote: common::Quote = rx.recv().unwrap();
				let ticker = "EURUSD";
				let msg_string = format!("ticker={}@time={}@ask={}@bid={}@askSize={}@bidSize={}", 
					ticker, quote.time, quote.ask, quote.bid, quote.ask_size, quote.bid_size);
				let msg = msg_string.as_bytes();
				match stream.write(&[msg.len() as u8]) {
					Ok(_) =>  {
						match stream.write(msg) {
							Ok(_) => {
								match stream.read(&mut [1]) {
									Ok(_) => {},
									Err(_) => delayed_print("Error al intentar leer la respuesta")
								}
							},
							Err(_) => delayed_print("Error al intentar escribir el mensaje")
						}
					},
					Err(_) => delayed_print("Error al intentar escribir la longitud del mensaje")
				}
			}
		},
		Err(_) => delayed_print("Error al intentar conectar")
	}
}

///
/// impresión con demora usada para reportar fallas dentro de loops
///
fn delayed_print(text: &str) {
	println!("{}", text);
	thread::sleep(std::time::Duration::from_millis(1000));
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// TESTS

#[cfg(test)]
mod tests {
	use std::thread;
	use std::net::TcpListener;
	use std::net::TcpStream;
	use run_quotes_publisher;
	use std::io::Write;
	use std::io::Read;
	extern crate common;
	extern crate configuration;
	use std::str;
	use std::thread::JoinHandle;
	
    #[test]
    fn it_works() {
		let slow_quotes = true;
		let handle = listen_for_quotes();
		run_quotes_publisher(slow_quotes, 1);
		let received_count = handle.join().unwrap();
        assert_eq!(received_count, 1);
    }
	
	///
	/// abre un puerto para la escucha de quotes en formato @ separated
	/// imprime los quotes en consola
	///
	fn listen_for_quotes() -> JoinHandle<i32> {
		return thread::spawn(move || {
			let url = format!("{}:{}", configuration::QUOTES_LISTENER_ADDRESS, configuration::QUOTES_LISTENER_PORT);
			println!("starting quotes listener on {}", url);
			match TcpListener::bind(url) {
				Ok(listener) => {
					for connection in listener.incoming() {
						match connection {
							Ok(mut stream) => {
								return listen_to_quotes_provider(&mut stream);
							},
							Err(e) => { println!("receiving connection error: {}", e) }
						}
					}
				},
				Err(e) => { println!("binding to port error: {}", e) }
			}
			return 0i32;
		});
	}

	///
	/// atiende a un proveedor de quotes en formato @ separated
	///
	fn listen_to_quotes_provider(stream: &mut TcpStream) -> i32 {
		let mut received_count = 0i32;
		while received_count == 0 {
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
									process_provided_quote_bytes(&buffer);
									received_count += 1;
								},
								Err(_) => { break; }
							}
						},
						Err(_) => { break; }
					}
				},
				Err(_) => { break; }
			}
		}
		return received_count;
	}

	///
	/// convierte los datos crudos (vector de bytes) a un texto @ separated
	/// parsea el texto y genera un quote
	///
	fn process_provided_quote_bytes(buffer: &Vec<u8>) {
		match str::from_utf8(buffer) {
			Ok(text) => {
				let quote = common::parse_quote(text);
				println!("{:?}", quote);
			},
			Err(e) => println!("read (msg size) error: {}", e)
		}
	}
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// ARGUMENTS

fn parse_args() -> getopts::Matches {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("", "run_publisher", "Run demo quotes publisher");
	opts.optflag("", "slow_quotes", "Slow quotes publishing");
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

fn main() {
	let arguments = parse_args();
	let run_publisher = arguments.opt_present("run_publisher");
	if run_publisher {
		let slow_quotes = arguments.opt_present("slow_quotes");
		run_quotes_publisher(slow_quotes, 100000000);
	}
}




