extern crate postgres;
extern crate common;
use postgres::Connection;
use postgres::stmt::Statement;
use postgres::transaction::Transaction;
//#[macro_use]
//extern crate enum_primitive;
//extern crate num;
//use num::FromPrimitive;

////////////////////////////////////////////////////////////////////////////////////////////////////
// POSTGRESQL STORAGE SUPPORT

pub const POSTGRESQL_CONNECTION_STRING : &str = "postgres://postgres:postgres@localhost:5432";

///
/// garantiza la existencia del soporte para el almacenamiento de quotes
///
pub fn ensure_db_support(conn: &Connection) {
	conn.execute("DROP TABLE IF EXISTS quote", &[]).unwrap();
    conn.execute("CREATE TABLE quote (
                    ticker		smallint,
                    time		bigint,
                    ask			real,
                    bid			real,
                    ask_size	real,
                    bid_size	real,
					PRIMARY KEY (ticker, time)
                  )", &[]).unwrap();
}

///
/// inserta un quote en la base de datos
///
pub fn insert_quote(quote: &common::Quote, ticker: &common::Ticker, conn: &Connection) {
	let ticker = ticker.clone() as i16;
	conn.execute(
        "INSERT INTO quote (ticker, time, ask, bid, ask_size, bid_size) VALUES ($1, $2, $3, $4, $5, $6)", 
		&[&ticker, &quote.time, &quote.ask, &quote.bid, &quote.ask_size, &quote.bid_size]
    ).unwrap();
}

pub fn prepare_quote(conn: &Connection) -> Statement {
	return conn.prepare(
        "INSERT INTO quote (ticker, time, ask, bid, ask_size, bid_size) VALUES ($1, $2, $3, $4, $5, $6)"
    ).unwrap();
}

pub fn execute_quote(quote: &common::Quote, ticker: &common::Ticker, stmt: &Statement) {
	let ticker = ticker.clone() as i16;
	stmt.execute(
		&[&ticker, &quote.time, &quote.ask, &quote.bid, &quote.ask_size, &quote.bid_size]
    ).unwrap();
}

pub fn prepare_quote_on_transaction(tran: &Transaction<'static>) -> Statement<'static> {
	return tran.prepare(
        "INSERT INTO quote (ticker, time, ask, bid, ask_size, bid_size) VALUES ($1, $2, $3, $4, $5, $6)"
    ).unwrap();
}

pub fn read_all_quotes(ticker: &common::Ticker, conn: &Connection, function: &mut FnMut(common::Quote)) {
    let int_ticker = ticker.clone() as i16;
    for row in &conn.query(
        "SELECT ticker, time, ask, bid, ask_size, bid_size FROM quote WHERE ticker = $1 ORDER BY time", &[&int_ticker]
        ).unwrap() {
        //let ticker:     i16 = row.get(0);
        function(common::Quote {
            //ticker:     common::Ticker::get_from_i16(ticker),
            time:       row.get(1),
            ask:        row.get(2),
            bid:        row.get(3),
            ask_size:   row.get(4),
            bid_size:   row.get(5)
        });
    }
}

pub fn read_first_quotes(ticker: &common::Ticker, count: i64, conn: &Connection, function: &mut FnMut(common::Quote)) {
    let int_ticker = ticker.clone() as i16;
    for row in &conn.query(
        "SELECT ticker, time, ask, bid, ask_size, bid_size FROM quote WHERE ticker = $1 ORDER BY time LIMIT $2", &[&int_ticker, &count]
        ).unwrap() {
        //let ticker:     i16 = row.get(0);
        function(common::Quote {
            //ticker:     common::Ticker::get_from_i16(ticker),
            time:       row.get(1),
            ask:        row.get(2),
            bid:        row.get(3),
            ask_size:   row.get(4),
            bid_size:   row.get(5)
        });
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
