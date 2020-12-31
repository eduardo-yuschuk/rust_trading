use std::thread;
extern crate datetime;
extern crate time;
use datetime::{LocalDate, LocalDateTime, LocalTime, Month};
extern crate bincode;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate enum_primitive;
extern crate num;
use datetime::DatePiece;
use datetime::TimePiece;
use num::FromPrimitive;
extern crate configuration;

////////////////////////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTS

///
/// Contiene la información acerca de las cotizaciones de un activo
/// en los extremos comprador y vendedor.
///
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quote {
//	pub ticker: Ticker,
	pub time: i64,
	pub ask: f32,
	pub bid: f32,
	pub ask_size: f32,
	pub bid_size: f32,
}

impl Quote {
	pub fn println(self: &Quote) {
		println!(
			//"Quote {{ ticker: {:?}, time: {:?}, ask: {}, bid: {}, ask_size: {}, bid_size: {} }}",
			"Quote {{ time: {:?}, ask: {}, bid: {}, ask_size: {}, bid_size: {} }}",
			//self.ticker,
			LocalDateTime::at_ms(self.time / 1000i64, (self.time % 1000i64) as i16),
			self.ask,
			self.bid,
			self.ask_size,
			self.bid_size
		);
	}

	pub fn empty() -> Quote {
		Quote {
			//ticker: Ticker::UNKNOWN,
			time: 0i64,
			ask: 0f32,
			bid: 0f32,
			ask_size: 0f32,
			bid_size: 0f32,
		}
	}

	pub fn serialized_len() -> u64 {
		bincode::serialized_size(&Quote::empty()).unwrap()
	}

	///
	/// obtiene una instancia de quote a partir de una representación string @ separated
	///
	pub fn parse(text: &str) -> Quote {
		//let msg_string = format!("ticker={}@time={}@ask={}@bid={}@askSize={}@bidSize={}",
		//ticker, quote.time, quote.ask, quote.bid, quote.ask_size, quote.bid_size);
		let parts: Vec<&str> = text.split('@').collect();
		//let ticker_parts: Vec<&str> = parts[0].split('=').collect();
		let time_parts: Vec<&str> = parts[0].split('=').collect();
		let ask_parts: Vec<&str> = parts[1].split('=').collect();
		let bid_parts: Vec<&str> = parts[2].split('=').collect();
		let ask_size_parts: Vec<&str> = parts[3].split('=').collect();
		let bid_size_parts: Vec<&str> = parts[4].split('=').collect();
		let quote = Quote {
			//ticker: Ticker::interpret_ticker(ticker_parts[1]),
			time: time_parts[1].parse::<i64>().unwrap(),
			ask: ask_parts[1].parse::<f32>().unwrap(),
			bid: bid_parts[1].parse::<f32>().unwrap(),
			ask_size: ask_size_parts[1].parse::<f32>().unwrap(),
			bid_size: bid_size_parts[1].parse::<f32>().unwrap(),
		};
		return quote;
	}

	pub fn get_quotes_folder_path(self: &Quote, ticker: &Ticker) -> String {
		let local_datetime = build_local_datetime(self.time);
		let root = configuration::get_quotes_tree_root();
		let ticker = /*self.*/ticker.get_ticker_string();
		let path = format!(
			"{}\\{}\\{}\\{}\\{}",
			root,
			ticker,
			local_datetime.year(),
			local_datetime.month().months_from_january() + 1usize,
			local_datetime.day()
		);
		return path;
	}

	pub fn get_quotes_file_path(self: &Quote, ticker: &Ticker) -> String {
		let quotes_folder_path = self.get_quotes_folder_path(ticker);
		let local_datetime = build_local_datetime(self.time);
		let path = format!("{}\\{}.bin", quotes_folder_path, local_datetime.hour());
		return path;
	}

	pub fn get_datetime(self: &Quote) -> LocalDateTime {
		let local_datetime = build_local_datetime(self.time as i64);
		return local_datetime;
	}
}

///
/// obtiene una instancia de quote a partir de una representación string @ separated
///
pub fn parse_quote(text: &str) -> Quote {
	let parts: Vec<&str> = text.split('@').collect();
	//let ticker_parts: Vec<&str> = parts[0].split('=').collect();
	let time_parts: Vec<&str> = parts[0].split('=').collect();
	let ask_parts: Vec<&str> = parts[1].split('=').collect();
	let bid_parts: Vec<&str> = parts[2].split('=').collect();
	let ask_size_parts: Vec<&str> = parts[3].split('=').collect();
	let bid_size_parts: Vec<&str> = parts[4].split('=').collect();
	let quote = Quote {
		//ticker: Ticker::interpret_ticker(ticker_parts[1]),
		time: time_parts[1].parse::<i64>().unwrap(),
		ask: ask_parts[1].parse::<f32>().unwrap(),
		bid: bid_parts[1].parse::<f32>().unwrap(),
		ask_size: ask_size_parts[1].parse::<f32>().unwrap(),
		bid_size: bid_size_parts[1].parse::<f32>().unwrap(),
	};
	return quote;
}

enum_from_primitive! {
#[derive(Clone, Debug, Serialize, Deserialize)]
///
/// Identificador de activo financiero.
///
pub enum Ticker {
	EURUSD,
	AUDUSD,
	UNKNOWN,
}
}

impl Ticker {
	pub fn interpret_ticker(text: &str) -> Ticker {
		match text {
			"EURUSD" => Ticker::EURUSD,
			"AUDUSD" => Ticker::AUDUSD,
			_ => Ticker::UNKNOWN,
		}
	}

	pub fn get_ticker_string(self: &Ticker) -> &'static str {
		match self {
			Ticker::EURUSD => "EURUSD",
			Ticker::AUDUSD => "AUDUSD",
			Ticker::UNKNOWN => "UNKNOWN",
		}
	}

	pub fn get_from_i16(ticker: i16) -> Ticker {
		return Ticker::from_i32(ticker as i32).unwrap();
	}

	pub fn get_from_i32(ticker: i32) -> Ticker {
		return Ticker::from_i32(ticker).unwrap();
	}
}

///
/// obtiene la cantidad de milisegundos desde la época para un datetime expresado en formato texto
///
pub fn build_millis(text: &str) -> i64 {
	match time::strptime(text, "%Y.%m.%d %H:%M:%S.%f") {
		Ok(tm) => {
			let date = LocalDate::ymd(
				tm.tm_year as i64 + 1900i64,
				Month::from_zero(tm.tm_mon as i8).unwrap(),
				tm.tm_mday as i8,
			)
			.unwrap();
			let time = LocalTime::hms_ms(
				tm.tm_hour as i8,
				tm.tm_min as i8,
				tm.tm_sec as i8,
				(tm.tm_nsec / 1000000) as i16,
			)
			.unwrap();
			let dt = LocalDateTime::new(date, time);
			let instant = dt.to_instant();
			let ms = instant.seconds() as i64 * 1000i64 + instant.milliseconds() as i64;
			return ms;
		}
		Err(e) => {
			println!("Error: {}", e);
			return 0i64;
		}
	};
}

pub fn build_local_datetime(millis: i64) -> LocalDateTime {
	LocalDateTime::at_ms(millis / 1000, (millis % 1000i64) as i16)
}

///
/// impresión con demora usada para reportar fallas dentro de loops
///
pub fn delayed_print(text: &str) {
	println!("{}", text);
	thread::sleep(std::time::Duration::from_millis(1000));
}

enum_from_primitive! {
#[derive(Clone, Debug, Serialize, Deserialize)]
///
/// Identificador de timeframe.
///
pub enum Timeframe {
	M1,
	M5,
	M15,
	H1,
	H5,
	D1,
	D5,
	UNKNOWN,
}
}

impl Timeframe {
	pub fn interpret_timeframe(text: &str) -> Timeframe {
		match text {
			"M1" => Timeframe::M1,
			"M5" => Timeframe::M5,
			"M15" => Timeframe::M15,
			"H1" => Timeframe::H1,
			"H5" => Timeframe::H5,
			"D1" => Timeframe::D1,
			"D5" => Timeframe::D5,
			_ => Timeframe::UNKNOWN,
		}
	}

	pub fn get_timeframe_string(self: &Timeframe) -> &'static str {
		match self {
			Timeframe::M1 => "M1",
			Timeframe::M5 => "M5",
			Timeframe::M15 => "M15",
			Timeframe::H1 => "H1",
			Timeframe::H5 => "H5",
			Timeframe::D1 => "D1",
			Timeframe::D5 => "D5",
			Timeframe::UNKNOWN => "UNKNOWN",
		}
	}

	pub fn get_seconds(self: &Timeframe) -> i32 {
		match self {
			Timeframe::M1 => 60,
			Timeframe::M5 => 60 * 5,
			Timeframe::M15 => 60 * 15,
			Timeframe::H1 => 3600,
			Timeframe::H5 => 3600 * 5,
			Timeframe::D1 => 3600 * 24,
			Timeframe::D5 => 3600 * 24 * 5,
			Timeframe::UNKNOWN => panic!("unable to compute seconds from UNKNOWN Timeframe"),
		}
	}
}

///
/// Contiene la información acerca de las cotizaciones de un activo
/// en los extremos comprador y vendedor.
///
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bar {
	// el ticker se va a repetir por cada bar, una tontería
	//pub ticker: Ticker,
	// el time no tiene mucha utilidad, el index nos dice que bar estamos mirando
	// considerando epoch como la referencia. nos ahorramos 4 bytes por bar.
	//pub time: i64,
	pub index: u32,
	pub quotes_count: u16,
	pub open: f32,
	pub high: f32,
	pub low: f32,
	pub close: f32,
}

impl Bar {
	pub fn println(self: &Bar) {
		println!(
			//"Bar {{ ticker: {:?}, time: {:?}, open: {}, high: {}, low: {}, close: {} }}",
			"Bar {{ index: {}, quotes_count: {}, open: {}, high: {}, low: {}, close: {} }}",
			//self.ticker,
			//LocalDateTime::at_ms(self.time / 1000i64, (self.time % 1000i64) as i16),
			self.index,
			self.quotes_count,
			self.open,
			self.high,
			self.low,
			self.close
		);
	}

	pub fn println_tf(self: &Bar, timeframe: &Timeframe) {
		let time = self.get_datetime(timeframe);
		println!(
			"Bar {{ index: {}, time: {:?}, quotes_count: {}, open: {}, high: {}, low: {}, close: {} }}",
			self.index,
			time,
			self.quotes_count,
			self.open,
			self.high,
			self.low,
			self.close
		);
	}

	pub fn empty() -> Bar {
		Bar {
			//ticker: Ticker::UNKNOWN,
			//time: 0i64,
			index: 0u32,
			quotes_count: 0u16,
			open: 0f32,
			high: 0f32,
			low: 0f32,
			close: 0f32,
		}
	}

	pub fn initialize(self: &mut Bar, index: u32, price: f32) {
		//self.ticker = ticker;
		//self.time = time;
		self.index = index;
		self.quotes_count = 1u16;
		self.open = price;
		self.high = price;
		self.low = price;
		self.close = price;
	}

	pub fn update(self: &mut Bar, price: f32) {
		self.quotes_count += 1;
		if price > self.high {
			self.high = price;
		}
		if price < self.low {
			self.low = price;
		}
		self.close = price;
	}

	pub fn serialized_len() -> u64 {
		bincode::serialized_size(&Bar::empty()).unwrap()
	}

	///
	/// obtiene una instancia de bar a partir de una representación string @ separated
	///
	pub fn parse(text: &str) -> Bar {
		//"index={}@quotes_count={}@open={}@high={}@low={}@close={}"
		let parts: Vec<&str> = text.split('@').collect();
		//let ticker_parts: Vec<&str> = parts[0].split('=').collect();
		let index_parts: Vec<&str> = parts[0].split('=').collect();
		let quotes_count_parts: Vec<&str> = parts[1].split('=').collect();
		let open_parts: Vec<&str> = parts[2].split('=').collect();
		let high_parts: Vec<&str> = parts[3].split('=').collect();
		let low_parts: Vec<&str> = parts[4].split('=').collect();
		let close_parts: Vec<&str> = parts[5].split('=').collect();
		let bar = Bar {
			//ticker: Ticker::interpret_ticker(ticker_parts[1]),
			index: index_parts[1].parse::<u32>().unwrap(),
			quotes_count: quotes_count_parts[1].parse::<u16>().unwrap(),
			open: open_parts[1].parse::<f32>().unwrap(),
			high: high_parts[1].parse::<f32>().unwrap(),
			low: low_parts[1].parse::<f32>().unwrap(),
			close: close_parts[1].parse::<f32>().unwrap(),
		};
		return bar;
	}

	pub fn get_bars_folder_path(self: &Bar, timeframe: &Timeframe, ticker: &Ticker) -> String {
		//let time = self.index * timeframe.get_seconds()
		let local_datetime = self.get_datetime(timeframe); //build_local_datetime(time);
		let root = configuration::get_bars_tree_root();
		let ticker_string = ticker.get_ticker_string();
		let path = format!(
			"{}\\{}\\{}\\{}\\{}",
			root,
			ticker_string,
			local_datetime.year(),
			local_datetime.month().months_from_january() + 1usize,
			local_datetime.day()
		);
		return path;
	}

	pub fn get_bars_file_path(self: &Bar, timeframe: &Timeframe, ticker: &Ticker) -> String {
		let bars_folder_path = self.get_bars_folder_path(timeframe, ticker);
		let local_datetime = self.get_datetime(timeframe);
		let path = format!("{}\\{}.bin", bars_folder_path, local_datetime.hour());
		return path;
	}

	pub fn get_datetime(self: &Bar, timeframe: &Timeframe) -> LocalDateTime {
		let time = self.index as u64 * timeframe.get_seconds() as u64 * 1000u64;
		let local_datetime = build_local_datetime(time as i64);
		return local_datetime;
	}
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IndicatorValue {
	pub value: f64,
}

impl IndicatorValue {

	pub fn empty() -> IndicatorValue {
		IndicatorValue {
			value: 0f64,
		}
	}
	
	pub fn println(self: &IndicatorValue) {
		println!("IndicatorValue {{ value: {} }}", self.value);
	}

	pub fn serialized_len() -> u64 {
		bincode::serialized_size(&IndicatorValue::empty()).unwrap()
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
