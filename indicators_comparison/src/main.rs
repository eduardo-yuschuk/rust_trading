/*
Ejecutar primero indicators_building, que generará la siguiente estructura de
indicadores en base a la invocación de la librería ta-lib original (C).

e17:trading user$ ls -la  /Users/user/bars/
total 5400
drwxr-xr-x  11 user  staff     374 Nov 16 10:50 .
drwxr-xr-x+ 28 user  staff     952 Nov 16 10:50 ..
-rw-r--r--   1 user  staff  699292 Nov 16 10:50 EURUSD_BARS_M1.bin
-rw-r--r--   1 user  staff  254288 Nov 16 10:50 EURUSD_BARS_M1_RSI120.bin
-rw-r--r--   1 user  staff  254288 Nov 16 10:50 EURUSD_BARS_M1_RSI15.bin
-rw-r--r--   1 user  staff  254288 Nov 16 10:50 EURUSD_BARS_M1_RSI30.bin
-rw-r--r--   1 user  staff  254288 Nov 16 10:50 EURUSD_BARS_M1_RSI60.bin
-rw-r--r--   1 user  staff  254288 Nov 16 10:50 EURUSD_BARS_M1_SMA120.bin
-rw-r--r--   1 user  staff  254288 Nov 16 10:50 EURUSD_BARS_M1_SMA15.bin
-rw-r--r--   1 user  staff  254288 Nov 16 10:50 EURUSD_BARS_M1_SMA30.bin
-rw-r--r--   1 user  staff  254288 Nov 16 10:50 EURUSD_BARS_M1_SMA60.bin

Para construir estos bars y finalmente los indicadores, requiere del archivo
.csv a partir del cual genera un .bin de quotes basados en ticks.

e17:trading user$ ls -la  /Users/user/quotes/
total 494136
drwxr-xr-x   5 user  staff        170 Nov 16 10:50 .
drwxr-xr-x+ 28 user  staff        952 Nov 16 10:50 ..
-rw-r--r--   1 user  staff   83451792 Nov 16 10:50 EURUSD_Ticks_2019.01.01_2019.01.31.bin
-rw-r--r--   1 user  staff  169536578 Nov 16 10:26 EURUSD_Ticks_2019.01.01_2019.01.31.csv

Esta prueba comparará los valores generados por la implementación Rust de ta-lib
con los valores generados en indicators_building. Validamos de este modo que no haya errores
en la implementación Rust de ta-lib.
*/

extern crate common;
use common::Timeframe;
extern crate binary_io;
extern crate configuration;
//use binary_io::get_all_bars_from_bin;
//use binary_io::get_all_indicator_values_from_bin;
extern crate indicators_building;
use indicators_building::load_close_prices;
extern crate indicators;
use indicators::sma;

fn main() {
    let timeframe = Timeframe::M1;
    let bin_bars_file_path =
        configuration::get_bars_file_path(Timeframe::get_timeframe_string(&timeframe));
    //let bars = get_all_bars_from_bin(&bin_bars_file_path).unwrap();
    //let root = configuration::get_bars_tree_root();
    //let bin_indicator_file_path = root.clone() + "EURUSD_BARS_M1_RSI15.bin";
    //let values = get_all_indicator_values_from_bin(&bin_indicator_file_path).unwrap();
    //assert_eq!(bars.len(), values.len());

    let mut close_prices = load_close_prices(&bin_bars_file_path);
    close_prices.truncate(30);

    let start_idx: i32 = 0i32;
    let end_idx: i32 = close_prices.len() as i32 - 1;
    //let mut in_real: Vec<f64> = vec!();
    //for close_price in &close_prices {
    //    in_real.push(*close_price as f32);
    //}
    let opt_in_time_period: i32 = 15;
    let mut out_beg_idx: i32 = 0i32;
    let mut out_nb_element: i32 = 0i32;
    let mut out_real_rust: Vec<f64> = vec!();//[0f64; close_prices.len()];

    let (out_real_c, _out_begin_c) = indicators_building::sma(opt_in_time_period as u32, &close_prices);
    println!("out_real_c.len(): {}", out_real_c.len());

    match sma(
        start_idx,              // index of the first close to use
        end_idx,                // index of the last close to use
        &close_prices,          // pointer to the first element of the vector
        opt_in_time_period,     // period of the sma
        &mut out_beg_idx,       // set to index of the first close to have an sma value
        &mut out_nb_element,    // set to number of sma values computed
        &mut out_real_rust      // pointer to the first element of the output vector
    ) {
        indicators::RetCode::Success => {},
        _ => { panic!("falla durante el calculo del inidicador"); }
    }
    println!("out_real_rust.len(): {}", out_real_rust.len());

    assert_eq!(out_real_c.len(), out_real_rust.len());

    println!();
    println!("C value   | Rust value");
    for i in 0..out_real_rust.len() {
        println!("{:.6} | {:.6}", out_real_c[i], out_real_rust[i]);
    }
}
