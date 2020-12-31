/*
Linux:

wget http://prdownloads.sourceforge.net/ta-lib/ta-lib-0.4.0-src.tar.gz
tar xvzf ta-lib-0.4.0-src.tar.gz
rm ta-lib-0.4.0-src.tar.gz
cd ta-lib
./configure
make
sudo make install

Mac:

brew install ta-lib

(si, sólo eso)

*/
// no compatible con windows

use std::fs::File;
use std::io::BufWriter;
extern crate common;
extern crate configuration;
use common::Bar;
use common::IndicatorValue;
extern crate bincode;
extern crate ta_lib_wrapper;
use ta_lib_wrapper::TA_Integer;
use ta_lib_wrapper::TA_MAType;
use ta_lib_wrapper::TA_Real;
use ta_lib_wrapper::TA_RetCode;
use ta_lib_wrapper::TA_MA;
use ta_lib_wrapper::TA_RSI;
//use ta_lib_wrapper::TA_ATR;
extern crate binary_io;
use binary_io::read_all_bars_from_bin;
use std::f64;
#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate serde_derive;
extern crate serde;
use std::path::Path;

pub fn build_indicators(bin_bars_file_path: &str) {
    let close_prices = load_close_prices(bin_bars_file_path);
    let periods = vec![15u32, 30u32, 60u32, 120u32];
    let indicators = vec![Indicator::SMA, Indicator::RSI];
    for indicator in indicators.iter() {
        for period in periods.iter() {
            build_indicator(
                indicator,
                &build_indicator_file_path(
                    bin_bars_file_path,
                    &(indicator.get_indicator_string().to_owned() + &period.to_string()),
                ),
                &close_prices,
                *period,
            );
        }
    }
}

pub fn build_indicator_file_path(bin_bars_file_path: &str, indicator_name: &str) -> String {
    let ending = "_".to_owned() + &indicator_name.to_owned() + &".bin".to_owned();
    let bin_indicator_file_path: String = bin_bars_file_path.replace(".bin", &ending);
    return bin_indicator_file_path;
}

pub fn load_close_prices(bin_bars_file_path: &str) -> Vec<TA_Real> {
    let mut close_prices: Vec<TA_Real> = Vec::new();
    read_all_bars_from_bin(bin_bars_file_path, &mut |bar: Bar| {
        close_prices.push(bar.close as TA_Real);
    });
    return close_prices;
}

enum_from_primitive! {
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Indicator {
SMA,
RSI,
ATR,
}
}

impl Indicator {
    pub fn get_indicator_string(self: &Indicator) -> &'static str {
        match self {
            Indicator::SMA => "SMA",
            Indicator::RSI => "RSI",
            Indicator::ATR => "ATR",
        }
    }
}

///
/// construye indicadores y los almacena en un único archivo
///
pub fn build_indicator(
    indicator: &Indicator,
    bin_indicator_file_path: &str,
    close_prices: &Vec<TA_Real>,
    period: u32,
) {
    if Path::new(&bin_indicator_file_path).exists() == false {
        println!(
            "Building binary indicator file on {:?}:{}...",
            indicator, period
        );
        match File::create(bin_indicator_file_path) {
            Ok(file) => {
                let (sma_values, begin) = match indicator {
                    Indicator::SMA => sma(period, close_prices),
                    Indicator::RSI => rsi(period, close_prices),
                    //Indicator::ATR => atr(period, close_prices),
                    _ => panic!("indicador no soportado!"),
                };

                let mut writer = BufWriter::new(file);
                let mut serialized_counter = 0usize;
                for _ in 0..begin {
                    match bincode::serialize_into(&mut writer, &IndicatorValue { value: f64::NAN })
                    {
                        Ok(_) => serialized_counter += 1,
                        Err(e) => println!("Error serializing indicator value {}", e),
                    }
                }
                for (_index, value) in sma_values.iter().enumerate() {
                    match bincode::serialize_into(&mut writer, &IndicatorValue { value: *value }) {
                        Ok(_) => serialized_counter += 1,
                        Err(e) => println!("Error serializing indicator value {}", e),
                    }
                }
                assert_eq!(close_prices.len(), serialized_counter);
            }
            Err(e) => panic!("Error writing file {}", e),
        }
    } else {
        println!(
            "Binary indicator file on {:?}:{} exists.",
            indicator, period
        );
    }
}

pub fn sma(period: u32, close_prices: &Vec<TA_Real>) -> (Vec<TA_Real>, TA_Integer) {
    let mut out: Vec<TA_Real> = Vec::with_capacity(close_prices.len());
    let mut out_begin: TA_Integer = 0;
    let mut out_size: TA_Integer = 0;

    unsafe {
        let ret_code = TA_MA(
            0,                             // index of the first close to use
            close_prices.len() as i32 - 1, // index of the last close to use
            close_prices.as_ptr(),         // pointer to the first element of the vector
            period as i32,                 // period of the sma
            TA_MAType::TA_MAType_SMA,      // type of the MA, here forced to sma
            &mut out_begin,                // set to index of the first close to have an sma value
            &mut out_size,                 // set to number of sma values computed
            out.as_mut_ptr(),              // pointer to the first element of the output vector
        );
        match ret_code {
            // Indicator was computed correctly, since the vector was filled by TA-lib C library,
            // Rust doesn't know what is the new length of the vector, so we set it manually
            // to the number of values returned by the TA_MA call
            TA_RetCode::TA_SUCCESS => out.set_len(out_size as usize),
            // An error occured
            _ => panic!("Could not compute indicator, err: {:?}", ret_code),
        }
    }
    (out, out_begin)
}

fn rsi(period: u32, close_prices: &Vec<TA_Real>) -> (Vec<TA_Real>, TA_Integer) {
    let mut out: Vec<TA_Real> = Vec::with_capacity(close_prices.len());
    let mut out_begin: TA_Integer = 0;
    let mut out_size: TA_Integer = 0;

    unsafe {
        let ret_code = TA_RSI(
            0,                             // index of the first close to use
            close_prices.len() as i32 - 1, // index of the last close to use
            close_prices.as_ptr(),         // pointer to the first element of the vector
            period as i32,                 // period of the rsi
            &mut out_begin,                // set to index of the first close to have an rsi value
            &mut out_size,                 // set to number of sma values computed
            out.as_mut_ptr(),              // pointer to the first element of the output vector
        );
        match ret_code {
            // Indicator was computed correctly, since the vector was filled by TA-lib C library,
            // Rust doesn't know what is the new length of the vector, so we set it manually
            // to the number of values returned by the TA_RSI call
            TA_RetCode::TA_SUCCESS => out.set_len(out_size as usize),
            // An error occured
            _ => panic!("Could not compute indicator, err: {:?}", ret_code),
        }
    }
    (out, out_begin)
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    extern crate common;
    extern crate configuration;
    use common::Ticker;
    use common::Timeframe;
    extern crate text_to_binary;
    use text_to_binary::convert_quotes;
    extern crate bars_building;
    use crate::build_indicators;
    use bars_building::build_bars;
    use binary_io::get_all_bars_from_bin;
    use binary_io::get_all_indicator_values_from_bin;

    #[test]
    fn it_works() {
        let bin_quotes_file_path = configuration::get_bin_quotes_file_path();
        if Path::new(&bin_quotes_file_path).exists() == false {
            println!("Building binary quotes file...");
            convert_quotes();
        } else {
            println!("Binary quotes file exists.");
        }
        let timeframe = Timeframe::M1;
        let ticker = Ticker::EURUSD;
        let bin_bars_file_path =
            configuration::get_bars_file_path(Timeframe::get_timeframe_string(&timeframe));
        if Path::new(&bin_bars_file_path).exists() == false {
            println!("Building binary bars file...");
            build_bars(
                &bin_quotes_file_path,
                &bin_bars_file_path,
                timeframe,
                &ticker,
            );
        } else {
            println!("Binary bars file exists.");
        }
        println!("Building binary indicators file...");
        build_indicators(&bin_bars_file_path);
        // TESTING
        {
            let timeframe = Timeframe::M1;
            let bin_bars_file_path =
                configuration::get_bars_file_path(Timeframe::get_timeframe_string(&timeframe));
            match get_all_bars_from_bin(&bin_bars_file_path) {
                Ok(bars) => {
                    let root = configuration::get_bars_tree_root();
                    let bin_indicator_file_path = root.clone() + "EURUSD_BARS_M1_RSI15.bin";
                    match get_all_indicator_values_from_bin(&bin_indicator_file_path) {
                        Ok(values) => {
                            assert_eq!(bars.len(), values.len());
                        }
                        Err(e) => panic!("error reading generated indicator: {}", e),
                    }
                }
                Err(e) => panic!("error reading generated bars: {}", e),
            }
        }
    }
}
