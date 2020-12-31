use std::collections::HashMap;

/// Representa una orden en el mercado.
pub struct Order {
    pub creation_time: i64,
    pub prices: Vec<f32>,
}

impl Order {}

/// Gestiona las órdenes en el mercado.
pub struct OrdersManager {
    pub open_orders: Vec<Order>,
}

impl OrdersManager {
    pub fn println(self: &OrdersManager) {
        println!("OrdersManager ...");
    }
}

/// Mantiene la información de una operación en el mercado.
/// Ésta puede estar compuesta de varios elementos de interacción con el mercado.
pub struct MarketOperation {
    pub recipe: MarketOperationRecipe,
}

/// Define una receta de operación en el mercado.
/// Una secuencia de acciones disparadas por tiempo o por eventos de mercado.
pub struct MarketOperationRecipe {
    pub steps: Vec<Box<RecipeStep>>,
}

/// Determina un paso dentro de una receta de operación en el mercado.
/// Existen diversos tipos de pasos con características diferentes.
pub trait RecipeStep {
    fn execute(&self);
}

/// Un paso simple que define que debe ejecutarse una operación a mercado.
/// La opración puede ser de compra o venta, y por una cantidad determinada de un instrumento.
pub struct RunAtMarket {
    pub side: Side,
    pub instrument: u16,
    pub size: i32,
}

/// Side de la operación en el mercado (compra o venta).
pub enum Side {
    Buy,
    Sell,
}

/// Proveedor de tickers a partir de identificadores de instrumentos.
pub trait TickerProvider {
    fn get_ticker(&self, instrument: u16) -> String;
}

/// Registro de instrumentos.
pub trait InstrumentsRegistry {
    fn add_instrument(&mut self, instrument: u16, ticker: &str);
}

/// Gestor general de instrumentos presentes en el sistema.
pub struct Instruments {
    pub instruments: HashMap<u16, String>,
}

impl Instruments {
    pub fn new() -> Instruments {
        Instruments {
            instruments: HashMap::new(),
        }
    }
}

impl TickerProvider for Instruments {
    fn get_ticker(&self, instrument: u16) -> String {
        match self.instruments.get(&instrument) {
            Some(ticker) => ticker.clone(),
            None => panic!("Instrument not found"),
        }
    }
}

impl InstrumentsRegistry for Instruments {
    fn add_instrument(&mut self, instrument: u16, ticker: &str) {
        self.instruments.insert(instrument, String::from(ticker));
    }
}

#[cfg(test)]
mod tests {
    use crate::Instruments;
    use crate::TickerProvider;
    use crate::InstrumentsRegistry;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn ticker_provider_works() {
        let mut instruments = Instruments::new();
        let instrument = 0u16;
        instruments.add_instrument(instrument, "EURUSD");
        let ticker = instruments.get_ticker(instrument);
        /*fn get_ticker<T: TickerProvider>(t: &T, instrument: u16) -> String {
            return t.get_ticker(instrument);
        }*/
        assert_eq!(ticker, "EURUSD");
    }
}
