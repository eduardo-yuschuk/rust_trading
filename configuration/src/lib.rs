extern crate os_info;

pub const QUOTES_LISTENER_ADDRESS: &str = "127.0.0.1";
pub const QUOTES_LISTENER_PORT: i32 = 8000;

pub const QUOTES_SERVER_ADDRESS: &str = "127.0.0.1";
pub const QUOTES_SERVER_PORT: i32 = 8001;

pub const HSB_RECEPTOR_ADDRESS: &str = "127.0.0.1";
pub const HSB_RECEPTOR_PORT: i32 = 8002;

pub const CONNECTION_WATCHER_TIMEOUT: u64 = 10000u64;
pub const ANALYZER_TIMEOUT: u64 = 4u64;

///////////////////////////////////////////////////////////////////////////////
/// data

//const WINDOWS_QUOTES_FOLDER: &str = "C:\\Users\\user\\quotes\\";
const WINDOWS_QUOTES_FOLDER: &str = "D:\\100 ROOT\\250 TRADING\\000 RAW DATA\\";
const LINUX_QUOTES_FOLDER: &str = "/home/user/quotes/";
const MACOSX_QUOTES_FOLDER: &str = "/Users/user/quotes/";
//const QUOTES_PREFIX: &str = "EURUSD_Ticks_2019.01.01_2019.01.31";
const QUOTES_PREFIX: &str = "EURUSD_Ticks_2020.01.01_2020.09.07";
const TEXT_EXTENSION: &str = ".csv";
const BIN_EXTENSION: &str = ".bin";

///////////////////////////////////////////////////////////////////////////////
/// quotes

fn windows_quotes_file_path() -> String {
	return WINDOWS_QUOTES_FOLDER.to_owned() + &QUOTES_PREFIX.to_owned();
}

fn linux_quotes_file_path() -> String {
	return LINUX_QUOTES_FOLDER.to_owned() + &QUOTES_PREFIX.to_owned();
}

fn macosx_quotes_file_path() -> String {
	return MACOSX_QUOTES_FOLDER.to_owned() + &QUOTES_PREFIX.to_owned();
}

fn unknown_os_quotes_file_path() -> String {
	return "".to_owned();
}

fn windows_text_quotes_file_path() -> String {
	return windows_quotes_file_path() + &TEXT_EXTENSION.to_owned();
}

fn linux_text_quotes_file_path() -> String {
	return linux_quotes_file_path() + &TEXT_EXTENSION.to_owned();
}

fn macosx_text_quotes_file_path() -> String {
	return macosx_quotes_file_path() + &TEXT_EXTENSION.to_owned();
}

fn unknown_os_text_quotes_file_path() -> String {
	return unknown_os_quotes_file_path();
}

fn windows_bin_quotes_file_path() -> String {
	return windows_quotes_file_path() + &BIN_EXTENSION.to_owned();
}

fn linux_bin_quotes_file_path() -> String {
	return linux_quotes_file_path() + &BIN_EXTENSION.to_owned();
}

fn macosx_bin_quotes_file_path() -> String {
	return macosx_quotes_file_path() + &BIN_EXTENSION.to_owned();
}

fn unknown_os_bin_quotes_file_path() -> String {
	return unknown_os_quotes_file_path();
}

///
/// obtiene el path compatible con la plataforma subyacente del archivo .csv
///
pub fn get_text_quotes_file_path() -> String {
	//println!("os_info::get().os_type(){}", os_info::get().os_type());
	match os_info::get().os_type() {
		os_info::Type::Linux => linux_text_quotes_file_path(),
		os_info::Type::Ubuntu => linux_text_quotes_file_path(),
		os_info::Type::Windows => windows_text_quotes_file_path(),
		os_info::Type::Macos => macosx_text_quotes_file_path(),
		_ => unknown_os_text_quotes_file_path(),
	}
}

///
/// obtiene el path compatible con la plataforma subyacente del archivo .bin
///
pub fn get_bin_quotes_file_path() -> String {
	//println!("os_info::get().os_type(){}", os_info::get().os_type());
	match os_info::get().os_type() {
		os_info::Type::Linux => linux_bin_quotes_file_path(),
		os_info::Type::Ubuntu => linux_bin_quotes_file_path(),
		os_info::Type::Windows => windows_bin_quotes_file_path(),
		os_info::Type::Macos => macosx_bin_quotes_file_path(),
		_ => unknown_os_bin_quotes_file_path(),
	}
}

///
/// obtiene el path compatible con la plataforma subyacente del raíz del tree de quotes
///
pub fn get_quotes_tree_root() -> String {
	//println!("os_info::get().os_type(){}", os_info::get().os_type());
	match os_info::get().os_type() {
		os_info::Type::Linux => linux_quotes_file_path(),
		os_info::Type::Ubuntu => linux_quotes_file_path(),
		os_info::Type::Windows => windows_quotes_file_path(),
		os_info::Type::Macos => macosx_quotes_file_path(),
		_ => unknown_os_quotes_file_path(),
	}
}

///////////////////////////////////////////////////////////////////////////////
/// bars

const WINDOWS_BARS_FOLDER: &str = "C:\\Users\\user\\bars\\";
const LINUX_BARS_FOLDER: &str = "/home/user/bars/";
const MACOSX_BARS_FOLDER: &str = "/Users/user/bars/";
const BARS_PREFIX: &str = "EURUSD_BARS_";

fn windows_bars_file_path(timeframe: &str) -> String {
	return WINDOWS_BARS_FOLDER.to_owned() + &BARS_PREFIX.to_owned() + &timeframe.to_owned() + &BIN_EXTENSION.to_owned();
}

fn linux_bars_file_path(timeframe: &str) -> String {
	return LINUX_BARS_FOLDER.to_owned() + &BARS_PREFIX.to_owned() + &timeframe.to_owned() + &BIN_EXTENSION.to_owned();
}

fn macosx_bars_file_path(timeframe: &str) -> String {
	return MACOSX_BARS_FOLDER.to_owned() + &BARS_PREFIX.to_owned() + &timeframe.to_owned() + &BIN_EXTENSION.to_owned();
}

fn unknown_os_bars_file_path(_timeframe: &str) -> String {
	return "".to_owned();
}

///
/// obtiene el path compatible con la plataforma subyacente del archivo .bin
///
pub fn get_bars_file_path(timeframe: &str) -> String {
	//println!("os_info::get().os_type(){}", os_info::get().os_type());
	match os_info::get().os_type() {
		os_info::Type::Linux => linux_bars_file_path(timeframe),
		os_info::Type::Ubuntu => linux_bars_file_path(timeframe),
		os_info::Type::Windows => windows_bars_file_path(timeframe),
		os_info::Type::Macos => macosx_bars_file_path(timeframe),
		_ => unknown_os_bars_file_path(timeframe),
	}
}

///
/// obtiene el path compatible con la plataforma subyacente del raíz del tree de quotes
///
pub fn get_bars_tree_root() -> String {
	//println!("os_info::get().os_type(){}", os_info::get().os_type());
	match os_info::get().os_type() {
		os_info::Type::Linux => LINUX_BARS_FOLDER.to_owned(),
		os_info::Type::Ubuntu => LINUX_BARS_FOLDER.to_owned(),
		os_info::Type::Windows => WINDOWS_BARS_FOLDER.to_owned(),
		os_info::Type::Macos => MACOSX_BARS_FOLDER.to_owned(),
		_ => "".to_owned(),
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
