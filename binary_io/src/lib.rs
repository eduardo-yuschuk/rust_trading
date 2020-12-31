///
/// comentar...
///
use std::fs::File;
extern crate common;
use common::Ticker;
use common::Bar;
use common::Quote;
use common::IndicatorValue;
extern crate bincode;
extern crate serde;
use std::fs;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;

///
/// lee un archivo .bin conteniendo quotes
/// invoca a una función recibida por parámetro entregando cada quote obtenido
///
pub fn read_quotes_from_bin(path: &str, function: &mut FnMut(Quote), max_quotes_count: i32) {
    match File::open(path) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut quotes_count = 0i32;
            loop {
                match bincode::deserialize_from(&mut reader) {
                    Ok(quote) => {
                        function(quote);
                        quotes_count += 1;
                        if quotes_count == max_quotes_count {
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Error deserializing quote {}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => panic!("error opening {}: {}", path, e),
    }
}

///
/// lee un archivo .bin conteniendo quotes
/// invoca a una función recibida por parámetro entregando cada quote obtenido
///
pub fn read_all_quotes_from_bin(path: &str, ticker: &Ticker, function: &mut FnMut(Quote, &Ticker)) {
    match File::open(path) {
        Ok(file) => match fs::metadata(path) {
            Ok(meta) => {
                let file_size = meta.len();
                //println!("file_size: {}", file_size);
                let quote_size = Quote::serialized_len();
                //println!("quote_size: {}", quote_size);
                let quotes_count = file_size / quote_size;
                //println!("quotes_count: {}", quotes_count);
                assert_eq!(file_size % quote_size, 0);
                let mut reader = BufReader::new(file);
                for _x in 0..quotes_count {
                    match bincode::deserialize_from(&mut reader) {
                        Ok(quote) => {
                            function(quote, ticker);
                        }
                        Err(e) => {
                            println!("Error deserializing quote {:?}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => panic!("Error getting metadata {:?}", e),
        },
        Err(e) => panic!("error opening {}: {}", path, e),
    }
}

///
/// lee un archivo .bin conteniendo quotes
/// retorna un vector con los quotes leídos
///
pub fn get_all_quotes_from_bin<'a>(path: &str, ticker: &Ticker) -> Result<Vec<Quote>, &'a str> {
    //println!("path {}", path);
    if Path::new(path).exists() {
        let mut quotes = vec![];
        read_all_quotes_from_bin(path, ticker, &mut |quote: Quote, _ticker: &Ticker| {
            quotes.push(quote);
        });
        return Ok(quotes);
    }
    println!("path not found {}", path);
    return Err("path not found");
}

pub fn write_quotes_to_bin(quotes: &Vec<Quote>, path: &str) {
    match File::create(path) {
        Ok(file) => {
            let mut writer = BufWriter::new(file);
            //let mut serialized_counter = 0;
            for quote in quotes {
                match bincode::serialize_into(&mut writer, &quote) {
                    Ok(_) => {
                        //serialized_counter += 1;
                    }
                    Err(e) => println!("Error serializing quote {}", e),
                }
            }
            //println!("{} quotes serialized", serialized_counter);
        }
        Err(e) => panic!("Error writing file {}", e),
    }
}

pub fn overwrite_quotes_to_bin(quotes: &Vec<Quote>, path: &str) {
    match fs::remove_file(path) {
        Ok(_) => {
            write_quotes_to_bin(quotes, path);
        }
        Err(e) => panic!("Error removing file {}", e),
    }
}

///////////////////////////////////////////////////////////////////////////////
/// bars

///
/// lee un archivo .bin conteniendo bars
/// invoca a una función recibida por parámetro entregando cada bar obtenido
///
pub fn read_all_bars_from_bin(path: &str, function: &mut FnMut(Bar)) {
    match File::open(path) {
        Ok(file) => match fs::metadata(path) {
            Ok(meta) => {
                let file_size = meta.len();
                let bar_size = Bar::serialized_len();
                let bars_count = file_size / bar_size;
                assert_eq!(file_size % bar_size, 0);
                let mut reader = BufReader::new(file);
                for _x in 0..bars_count {
                    match bincode::deserialize_from(&mut reader) {
                        Ok(bar) => {
                            function(bar);
                        }
                        Err(e) => {
                            println!("Error deserializing bar {:?}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => panic!("Error getting metadata {:?}", e),
        },
        Err(e) => panic!("error opening {}: {}", path, e),
    }
}

pub fn get_all_bars_from_bin(path: &str) -> Result<Vec<Bar>, &str> {
    if Path::new(path).exists() {
        let mut bars = vec![];
        read_all_bars_from_bin(path, &mut |bar: Bar| {
            bars.push(bar);
        });
        return Ok(bars);
    }
    println!("path not found {}", path);
    return Err("path not found");
}

///
/// lee un archivo .bin conteniendo valores de un indicador
/// retorna un vector con los valores de un indicador leídos
///
pub fn get_all_indicator_values_from_bin(path: &str) -> Result<Vec<IndicatorValue>, &str> {
    if Path::new(path).exists() {
        let mut values = vec![];
        read_all_indicator_values(path, &mut |value: IndicatorValue| {
            values.push(value);
        });
        return Ok(values);
    }
    println!("path not found {}", path);
    return Err("path not found");
}

///
/// lee un archivo .bin conteniendo valores de un indicador
/// invoca a una función recibida por parámetro entregando cada valor obtenido
///
pub fn read_all_indicator_values(path: &str, function: &mut FnMut(IndicatorValue)) {
    match File::open(path) {
        Ok(file) => match fs::metadata(path) {
            Ok(meta) => {
                let file_size = meta.len();
                let value_size = IndicatorValue::serialized_len();
                let values_count = file_size / value_size;
                assert_eq!(file_size % value_size, 0);
                let mut reader = BufReader::new(file);
                for _x in 0..values_count {
                    match bincode::deserialize_from(&mut reader) {
                        Ok(value) => {
                            function(value);
                        }
                        Err(e) => {
                            println!("Error deserializing indicator value {:?}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => panic!("Error getting metadata {:?}", e),
        },
        Err(e) => panic!("error opening {}: {}", path, e),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
