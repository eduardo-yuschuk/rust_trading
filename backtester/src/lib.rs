///
/// permite ejecutar backtestings
///
extern crate common;
use common::Bar;
use common::IndicatorValue;
use common::Quote;
use common::Ticker;
extern crate binary_io;
use binary_io::read_all_bars_from_bin;
use binary_io::read_all_quotes_from_bin;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct TradingSession {
    pub trades: Vec<Trade>,
    pub active_trades_indexes: Vec<usize>,
}

impl TradingSession {
    pub fn new() -> TradingSession {
        TradingSession {
            trades: vec![],
            active_trades_indexes: vec![],
        }
    }

    pub fn open(
        self: &mut TradingSession,
        id: u32,
        side: Side,
        signal: Signal,
        quote: &Quote,
        stop_loss_pips: f32,
        take_profit_pips: f32,
    ) {
        let trade = Trade::new(id, side, signal, quote, stop_loss_pips, take_profit_pips);
        self.trades.push(trade);
        self.active_trades_indexes.push(self.trades.len() - 1);
        //self.println();
    }

    pub fn open_on_bar(
        self: &mut TradingSession,
        id: u32,
        side: Side,
        signal: Signal,
        bar: &Bar,
        stop_loss_pips: f32,
        take_profit_pips: f32,
    ) {
        let trade = Trade::new_on_bar(id, side, signal, bar, stop_loss_pips, take_profit_pips);
        self.trades.push(trade);
        self.active_trades_indexes.push(self.trades.len() - 1);
        //self.println();
    }

    pub fn is_positioned(self: &TradingSession) -> bool {
        self.active_trades_indexes.len() > 0
    }

    pub fn close_all(self: &mut TradingSession, signal: Signal, quote: &Quote) {
        for active_trade_index in &self.active_trades_indexes {
            self.trades[*active_trade_index].close(signal.clone(), quote);
        }
        self.active_trades_indexes.clear();
        //self.println();
    }

    pub fn update(self: &mut TradingSession, quote: &Quote) {
        let mut i = 0usize;
        // puedo clonar este vector porque es chico
        for active_trade_index in &self.active_trades_indexes.clone() {
            let was_closed = self.trades[*active_trade_index].update(quote);
            if was_closed {
                self.active_trades_indexes.remove(i);
            }
            i += 1;
        }
    }

    pub fn update_on_bar(self: &mut TradingSession, bar: &Bar) {
        let mut i = 0usize;
        // puedo clonar este vector porque es chico
        for active_trade_index in &self.active_trades_indexes.clone() {
            let was_closed = self.trades[*active_trade_index].update_on_bar(bar);
            if was_closed {
                self.active_trades_indexes.remove(i); // ESTO ESTÁ MAL!!!!!!!!!!!!!!!
            }
            i += 1;
        }
    }

    pub fn println(self: &TradingSession) {
        println!("trades count {}", self.trades.len());
        println!(
            "active_trades_indexes count {}",
            self.active_trades_indexes.len()
        );
        for trade in &self.trades {
            trade.println();
        }
    }

    pub fn print_trades(self: &TradingSession) {
        println!("trades count {}", self.trades.len());
        println!(
            "active_trades_indexes count {}",
            self.active_trades_indexes.len()
        );
        for trade in &self.trades {
            let pnl = trade.effective_close_price - trade.effective_open_price;
            println!("Id: {:06}, PNL: {:+.*}", trade.id, 5, pnl);
        }
    }

    pub fn build_report(self: &TradingSession) -> TradingSessionReport {
        TradingSessionReport::new(self)
    }

    pub fn save(self: &TradingSession, file_path: &str) {
        match serde_json::to_string(&self) {
            Ok(json_text) => {
                let path = Path::new(file_path);
                match File::create(&path) {
                    Ok(mut file) => match file.write(json_text.as_bytes()) {
                        Ok(_) => {}
                        Err(e) => panic!("couldn't write json file {}", e),
                    },
                    Err(e) => panic!("error creating json file {}", e),
                }
            }
            Err(e) => panic!("error building json content {}", e),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Trade {
    pub id: u32,
    pub state: TradeState,
    pub side: Side,
    pub signal_open: Signal,
    pub signal_close: Signal,
    // para backtesting basado en bars
    pub bar_index_open: u32,
    pub bar_index_close: u32,
    // para backtesting basado en quotes
    pub time_open: i64,
    pub time_close: i64,
    // effective execution prices
    pub effective_open_price: f32,
    pub effective_close_price: f32,
    // quotes based prices
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub stop_loss: f32,
    pub take_profit: f32,
    pub opening_commission: f32,
    pub closing_commission: f32,
}

impl Trade {
    pub fn new(
        id: u32,
        side: Side,
        signal: Signal,
        quote: &Quote,
        stop_loss_pips: f32,
        take_profit_pips: f32,
    ) -> Trade {
        Trade {
            id: id,
            state: TradeState::Opened,
            side: side.clone(),
            signal_open: signal,
            signal_close: Signal::Undefined,
            bar_index_open: 0u32,
            bar_index_close: 0u32,
            time_open: quote.time,
            time_close: 0i64,
            effective_open_price: quote.ask,
            effective_close_price: 0f32,
            open: quote.ask,
            high: quote.ask,
            low: quote.ask,
            close: quote.ask,
            stop_loss: match side {
                Side::Buy => quote.ask - stop_loss_pips,
                Side::Sell => quote.ask + stop_loss_pips,
            },
            take_profit: match side {
                Side::Buy => quote.ask + take_profit_pips,
                Side::Sell => quote.ask - take_profit_pips,
            },
            opening_commission: 0f32,
            closing_commission: 0f32,
        }
    }

    pub fn new_on_bar(
        id: u32,
        side: Side,
        signal: Signal,
        bar: &Bar,
        stop_loss_pips: f32,
        take_profit_pips: f32,
    ) -> Trade {
        Trade {
            id: id,
            state: TradeState::Opened,
            side: side.clone(),
            signal_open: signal,
            signal_close: Signal::Undefined,
            bar_index_open: bar.index,
            bar_index_close: 0u32,
            time_open: 0i64,
            time_close: 0i64,
            effective_open_price: bar.close,
            effective_close_price: 0f32,
            open: bar.close,
            high: bar.close,
            low: bar.close,
            close: bar.close,
            stop_loss: match side {
                Side::Buy => bar.close - stop_loss_pips,
                Side::Sell => bar.close + stop_loss_pips,
            },
            take_profit: match side {
                Side::Buy => bar.close + take_profit_pips,
                Side::Sell => bar.close - take_profit_pips,
            },
            opening_commission: 0f32,
            closing_commission: 0f32,
        }
    }

    pub fn is_open(self: &Trade) -> bool {
        self.state == TradeState::Opened
    }

    fn update_prices(self: &mut Trade, quote: &Quote) {
        //println!("updating prices {:?}", quote);
        //print!(".");
        if quote.ask > self.high {
            self.high = quote.ask;
        }
        if quote.ask < self.low {
            self.low = quote.ask;
        }
        self.close = quote.ask;
    }

    fn update_prices_on_bar(self: &mut Trade, bar: &Bar) {
        if bar.close > self.high {
            self.high = bar.close;
        }
        if bar.close < self.low {
            self.low = bar.close;
        }
        self.close = bar.close;
    }

    fn close(self: &mut Trade, signal: Signal, quote: &Quote) {
        //println!("closing trade with id {}, signal {:?}", self.id, signal);
        self.state = TradeState::Closed;
        self.signal_close = signal;
        self.time_close = quote.time;
        self.effective_close_price = quote.ask;
        self.update_prices(quote);
    }

    fn close_on_bar(self: &mut Trade, signal: Signal, bar: &Bar) {
        //println!("closing trade with id {}, signal {:?}", self.id, signal);
        self.state = TradeState::Closed;
        self.signal_close = signal;
        self.bar_index_close = bar.index;
        self.effective_close_price = bar.close;
        self.update_prices_on_bar(bar);
    }

    fn update(self: &mut Trade, quote: &Quote) -> bool {
        match self.side {
            Side::Buy => {
                if quote.ask >= self.take_profit {
                    self.close(Signal::TakeProfit, quote);
                    return true;
                }
                if quote.ask <= self.stop_loss {
                    self.close(Signal::StopLoss, quote);
                    return true;
                }
            }
            Side::Sell => {
                if quote.ask <= self.take_profit {
                    self.close(Signal::TakeProfit, quote);
                    return true;
                }
                if quote.ask >= self.stop_loss {
                    self.close(Signal::StopLoss, quote);
                    return true;
                }
            }
        }
        // considero que el close actualiza los precios
        self.update_prices(quote);
        return false;
    }

    fn update_on_bar(self: &mut Trade, bar: &Bar) -> bool {
        match self.side {
            Side::Buy => {
                if bar.close >= self.take_profit {
                    self.close_on_bar(Signal::TakeProfit, bar);
                    return true;
                }
                if bar.close <= self.stop_loss {
                    self.close_on_bar(Signal::StopLoss, bar);
                    return true;
                }
            }
            Side::Sell => {
                if bar.close <= self.take_profit {
                    self.close_on_bar(Signal::TakeProfit, bar);
                    return true;
                }
                if bar.close >= self.stop_loss {
                    self.close_on_bar(Signal::StopLoss, bar);
                    return true;
                }
            }
        }
        // considero que el close actualiza los precios
        self.update_prices_on_bar(bar);
        return false;
    }

    fn compute_pnl(self: &Trade) -> f32 {
        match self.side {
            Side::Buy => self.effective_close_price - self.effective_open_price,
            Side::Sell => self.effective_open_price - self.effective_close_price,
        }
    }

    pub fn println(self: &Trade) {
        // println!("Trade {{");
        // println!("  id: {:?}", self.id);
        // println!("  state: {:?}", self.state);
        // println!("  side: {:?}", self.side);
        // println!("  signal_open: {:?}", self.signal_open);
        // println!("  signal_close: {:?}", self.signal_close);
        // println!("  time_open: {}", self.time_open);
        // println!("  time_close: {}", self.time_close);
        // println!("  open: {}", self.open);
        // println!("  high: {}", self.high);
        // println!("  low: {}", self.low);
        // println!("  close: {}", self.close);
        // println!("  stop_loss: {}", self.stop_loss);
        // println!("  take_profit: {}", self.take_profit);
        // println!("  opening_commission: {}", self.opening_commission);
        // println!("  closing_commission: {}", self.closing_commission);
        // println!("}}");
        println!("{:?}", self);
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TradeState {
    Opened,
    Closed,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Signal {
    StrategyOpen,
    StrategyClose,
    StopLoss,
    TakeProfit,
    BlackZone,
    Undefined,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TradingSessionReport {
    pub trades_count: usize,
    pub pnl: f32,
}

impl TradingSessionReport {
    pub fn new(trading_session: &TradingSession) -> TradingSessionReport {
        let mut pnl = 0f32;
        for trade in &trading_session.trades {
            pnl += trade.compute_pnl();
        }
        TradingSessionReport {
            trades_count: trading_session.trades.len(),
            pnl: pnl,
        }
    }

    pub fn println(self: &TradingSessionReport) {
        println!("{:?}", self);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Order {
    pub state: OrderState,
    pub limit: f32,
    pub stop: f32,
    pub take: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OrderState {
    Creaded,
    Opened,
    Filled,
    Closed,
    Canceled,
}

pub fn do_backtesting(
    bin_quotes_file_path: &str,
    ticker: &Ticker,
    function: &mut FnMut(&Quote, &mut TradingSession),
) -> TradingSession {
    let mut trading_session = TradingSession::new();
    read_all_quotes_from_bin(
        bin_quotes_file_path,
        ticker,
        &mut |quote: Quote, _ticker: &Ticker| {
            function(&quote, &mut trading_session);
        },
    );
    trading_session
}

pub fn do_backtesting_on_bars(
    bars_file_path: &str,
    function: &mut FnMut(&Bar, &mut TradingSession),
) -> TradingSession {
    let mut trading_session = TradingSession::new();
    read_all_bars_from_bin(bars_file_path, &mut |bar: Bar| {
        function(&bar, &mut trading_session);
    });
    trading_session
}

pub fn do_backtesting_on_bars_and_indicators(
    bars_file_path: &str,
    indicators_file_paths: &BTreeMap<&str, String>,
    function: &mut FnMut(&Bar, &BTreeMap<&str, IndicatorValue>, &mut TradingSession),
) -> TradingSession {
    let mut trading_session = TradingSession::new();

    match binary_io::get_all_bars_from_bin(bars_file_path) {
        Ok(bars) => {
            let samples_count = bars.len();
            let mut indicators_values: BTreeMap<&str, Vec<IndicatorValue>> = BTreeMap::new();
            for (indicator_name, indicator_file_path) in indicators_file_paths.iter() {
                match binary_io::get_all_indicator_values_from_bin(indicator_file_path) {
                    Ok(values) => {
                        assert_eq!(samples_count, values.len());
                        indicators_values.insert(indicator_name, values);
                    }
                    Err(e) => panic!("error reading indicator {}: {}", indicator_name, e),
                }
            }

            // values propagation
            for i in 0..samples_count {
                let bar = bars[i].clone();
                let mut indicators_value: BTreeMap<&str, IndicatorValue> = BTreeMap::new();
                for indicator_name in indicators_values.keys() {
                    indicators_value.insert(
                        &indicator_name,
                        indicators_values[indicator_name][i].clone(),
                    );
                }
                function(&bar, &indicators_value, &mut trading_session);
            }
        }
        Err(e) => panic!("error reading bars {}: {}", bars_file_path, e),
    }

    trading_session
}

// TODO agregar un backtesting basado en bars

#[cfg(test)]
mod tests {
    extern crate common;
    extern crate configuration;
    use common::Quote;
    use {do_backtesting, Side, Signal, Trade, TradingSession};
    #[test]
    fn it_works() {
        let bin_quotes_file_path = configuration::get_bin_quotes_file_path();
        println!("starting backtesting...");
        let mut id = 0u32;
        let ts = do_backtesting(
            &bin_quotes_file_path,
            &Ticker::EURUSD,
            &mut |quote: &Quote, trading_session: &mut TradingSession| {
                if trading_session.is_positioned() == false {
                    println!("opening position...");
                    let trade = Trade::new(
                        id,
                        Side::Buy,
                        Signal::StrategyOpen,
                        quote,
                        0.0020f32,
                        0.0020f32,
                    );
                    id += 1;
                    trading_session.trades.push(trade);
                }
            },
        );
        assert_eq!(ts.trades.len(), 1);
    }
}
