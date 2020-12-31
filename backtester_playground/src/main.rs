extern crate common;
extern crate configuration;
use common::Bar;
use common::Quote;
use common::IndicatorValue;
use common::Timeframe;
use common::Ticker;
extern crate backtester;
use backtester::{do_backtesting, do_backtesting_on_bars, do_backtesting_on_bars_and_indicators, Side, Signal, TradingSession};
//use std::thread;
use std::path::Path;
extern crate text_to_binary;
use text_to_binary::convert_quotes;
extern crate bars_building;
use bars_building::build_bars;
extern crate indicators_building;
// no compatible con windows
//use indicators_building::build_indicators;
use std::collections::BTreeMap;

fn main() {
    ensure_data_availability();
    
    ///////////////////////////////////////////////////////////////////////////
    ////open_and_close_immediately_on_quotes();
    ////open_let_be_closed_open_another_on_quotes();
    ////just_print_bars();
    
    //open_let_be_closed_open_another_on_bars();
    
    ////just_print_bars_and_indicators();
}

///////////////////////////////////////////////////////////////////////////////
/// auxiliares

pub fn ensure_data_availability() {
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
        build_bars(&bin_quotes_file_path, &bin_bars_file_path, timeframe, &ticker);
    } else {
        println!("Binary bars file exists.");
    }
    println!("Building binary indicators file...");
    // no compatible con windows
    //build_indicators(&bin_bars_file_path);
}

///////////////////////////////////////////////////////////////////////////////
/// estrategias

pub fn open_and_close_immediately_on_quotes() {
    let bin_quotes_file_path = configuration::get_bin_quotes_file_path();
    println!("starting backtesting...");
    let mut id = 0u32;
    let trading_session = do_backtesting(
        &bin_quotes_file_path,
        &Ticker::EURUSD,
        &mut |quote: &Quote, trading_session: &mut TradingSession| {
            if trading_session.is_positioned() {
                println!("closing positions...");
                trading_session.close_all(Signal::StrategyClose, quote);
            } else {
                println!("opening position...");
                trading_session.open(
                    id,
                    Side::Buy,
                    Signal::StrategyOpen,
                    quote,
                    0.0020f32,
                    0.0020f32,
                );
                id += 1;
            }
        },
    );
    trading_session.println();
}

pub fn open_let_be_closed_open_another_on_quotes() {
    let bin_quotes_file_path = configuration::get_bin_quotes_file_path();
    println!("starting backtesting...");
    let mut id = 0u32;
    let trading_session = do_backtesting(
        &bin_quotes_file_path,
        &Ticker::EURUSD,
        &mut |quote: &Quote, trading_session: &mut TradingSession| {
            if trading_session.is_positioned() {
                trading_session.update(quote);
            } else {
                trading_session.open(
                    id,
                    Side::Buy,
                    Signal::StrategyOpen,
                    quote,
                    0.0020f32,
                    0.0020f32,
                );
                id += 1;
            }
        },
    );
    trading_session.print_trades();
    trading_session.build_report().println();
}

pub fn just_print_bars() {
    let bin_bars_file_path =
        configuration::get_bars_file_path(Timeframe::get_timeframe_string(&Timeframe::M1));
    println!("starting backtesting...");
    let trading_session = do_backtesting_on_bars(
        &bin_bars_file_path,
        &mut |bar: &Bar, _trading_session: &mut TradingSession| {
            bar.println_tf(&Timeframe::M1);
        },
    );
    trading_session.print_trades();
    trading_session.build_report().println();
}

pub fn just_print_bars_and_indicators() {
    let bin_bars_file_path =
        configuration::get_bars_file_path(Timeframe::get_timeframe_string(&Timeframe::M1));
    println!("starting backtesting...");

    let mut indicators_file_paths: BTreeMap<&str, String> = BTreeMap::new();
    // TODO corregir
    let root = configuration::get_bars_tree_root();
    indicators_file_paths.insert("RSI120", root.clone() + "EURUSD_BARS_M1_RSI120.bin");
    indicators_file_paths.insert("RSI15", root.clone() + "EURUSD_BARS_M1_RSI15.bin");
    indicators_file_paths.insert("RSI30", root.clone() + "EURUSD_BARS_M1_RSI30.bin");
    indicators_file_paths.insert("RSI60", root.clone() + "EURUSD_BARS_M1_RSI60.bin");
    indicators_file_paths.insert("SMA120", root.clone() + "EURUSD_BARS_M1_SMA120.bin");
    indicators_file_paths.insert("SMA15", root.clone() + "EURUSD_BARS_M1_SMA15.bin");
    indicators_file_paths.insert("SMA30", root.clone() + "EURUSD_BARS_M1_SMA30.bin");
    indicators_file_paths.insert("SMA60", root.clone() + "EURUSD_BARS_M1_SMA60.bin");

    let trading_session = do_backtesting_on_bars_and_indicators(
        &bin_bars_file_path,
        &indicators_file_paths,
        &mut |bar: &Bar, indicators: &BTreeMap<&str, IndicatorValue>, _trading_session: &mut TradingSession| {
            bar.println_tf(&Timeframe::M1);
            println!("indicators {:?}", indicators);
        },
    );
    //trading_session.print_trades();
    trading_session.build_report().println();
}

pub fn open_let_be_closed_open_another_on_bars() {
    let bin_bars_file_path =
        configuration::get_bars_file_path(Timeframe::get_timeframe_string(&Timeframe::M1));
    println!("starting backtesting...");
    let mut id = 0u32;
    let trading_session = do_backtesting_on_bars(
        &bin_bars_file_path,
        &mut |bar: &Bar, trading_session: &mut TradingSession| {
            if trading_session.is_positioned() {
                trading_session.update_on_bar(bar);
            } else {
                trading_session.open_on_bar(
                    id,
                    Side::Buy,
                    Signal::StrategyOpen,
                    bar,
                    0.0025f32,
                    0.0025f32,
                );
                id += 1;
            }
        },
    );
    trading_session.print_trades();
    trading_session.build_report().println();
}
