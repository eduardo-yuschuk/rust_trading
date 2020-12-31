use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
extern crate common;
extern crate configuration;
use std::time::Instant;
use std::net;
use std::net::Ipv4Addr;
//use std::net::UdpSocket;

#[derive(Clone, Debug)]
pub struct Counterparty {
	pub id:		u64,
	pub ip:		Ipv4Addr,
	pub port:	i32,
}

#[derive(Clone, Debug)]
pub struct AgentInfo {
	pub counterparty:	Counterparty,
	pub counterparties:	Vec<Counterparty>,
	pub is_master:		bool,
	pub priority:		i32,
	pub snapshot_time:	u64,
}

fn load_counterparties() -> Vec<Counterparty> {
	// TODO obtener realmente las contrapartes (configuración)
	return vec![];
}

fn run_connection_watcher(tx_connected: &Sender<bool>) {
	
	fn is_connected() -> bool {
		// TODO verificar realmente la conexión
		return true;
	}
	
	loop {
		let connected = is_connected();
		match tx_connected.send(connected) {
			Ok(_) => {},
			Err(_) => common::delayed_print("Error al notificar el estado de conexión"),
		}
		thread::sleep(std::time::Duration::from_millis(configuration::CONNECTION_WATCHER_TIMEOUT));
	}
}


fn run_receptor(_tx_received: &Sender<bool>) {
	loop {
		let url = format!("{}:{}", configuration::HSB_RECEPTOR_ADDRESS, configuration::HSB_RECEPTOR_PORT);
		println!("receptor waiting on {}", url);
		let attempt = net::UdpSocket::bind(url);
		match attempt {
			Ok(socket) => {
				println!("receptor binded");
				//thread::spawn(move || {
				let mut buf: [u8; 1] = [0; 1];
				let result = socket.recv_from(&mut buf);
				println!("receptor received");
				drop(socket);
				match result {
					Ok((amt, src)) => {
						println!("Received data from {}", src);
						let data = Vec::from(&buf[0..amt]);
						println!("Got {} bytes", data.len());
					},
					Err(err) => panic!("Read error: {}", err)
				}
				//});
			},
			Err(e) => { println!("bind function failed: {:?}", e); },
		}
		//thread::sleep(std::time::Duration::from_millis(1000));
	}
}

fn run_propagator() {
	loop {
		thread::sleep(std::time::Duration::from_millis(1000));
	}
}

fn run_analyzer(rx_connected: &Receiver<bool>) {
	let mut previous_instant = Instant::now();
	loop {
		// estoy conectado?
		let mut connected: Option<bool> = None;
		match rx_connected.try_recv() {
			Ok(value) => {
				//println!("received connected {}", connected);
				connected = Some(value);
			},
			Err(_e) => {
				//println!("received connected error {:?}", e);
			},
		}
		// ...
		// ...
		// análisis basado en esos estados
		match connected {
			Some(_) => {
				if previous_instant.elapsed().as_secs() >= configuration::ANALYZER_TIMEOUT {
					println!("analizando basado en connected {:?}", connected);
					previous_instant = Instant::now();			
				}
			},
			None => {},
		}
	}
}

fn run_agent(_counterparties: &Vec<Counterparty>) {
	let (tx_connected, rx_connected): (Sender<bool>, Receiver<bool>) = mpsc::channel();
	let t1 = thread::spawn(move || { run_connection_watcher(&tx_connected) });
	let (tx_received, _rx_received): (Sender<bool>, Receiver<bool>) = mpsc::channel();
	let t2 = thread::spawn(move || { run_receptor(&tx_received) });
	let t3 = thread::spawn(move || { run_propagator() });
	let t4 = thread::spawn(move || { run_analyzer(&rx_connected) });
	println!("internal threads executed");
	let keep_running = true;
	while keep_running {
		thread::sleep(std::time::Duration::from_millis(1000));
	}
	println!("exit signal received");
	println!("waiting for internal threads");
	match t1.join() { Ok(_) => {}, Err(e) => { println!("error {:?}", e) }}
	match t2.join() { Ok(_) => {}, Err(e) => { println!("error {:?}", e) }}
	match t3.join() { Ok(_) => {}, Err(e) => { println!("error {:?}", e) }}
	match t4.join() { Ok(_) => {}, Err(e) => { println!("error {:?}", e) }}
}

fn main() {
	let counterparties = load_counterparties();
	run_agent(&counterparties);    
}
