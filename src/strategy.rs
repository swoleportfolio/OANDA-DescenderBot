
// TODO: Connect the strat to the api?
// Get the actual strategy implemented on the exchange.

use std::collections::HashSet;

// actions I can take in the trade
enum OrderSide {
    Buy,
    Sell,
}

enum OrderAction {
    Place(OrderSide),
    Cancel,
    Hold,
}

// trade inputs
struct TradeInput { 
    lower_bound: f64, // ie: $99
    upper_bound: f64, // ie: $100
    num_orders: u32, // total orders

    executed_levels: HashSet<i64>, //contains memory of whats been traded.
}

// trade outputs 
struct TradeDecision {
    action: OrderAction,
    price: f64,
}



impl TradeInput {
    fn generate_orders(&mut self, current_price: f64) -> Vec<TradeDecision> {
        let mut orders = Vec::new();

        let step = (self.upper_bound - self.lower_bound) / self.num_orders as f64;

        // Calculates spacing between each order in the price range.
        // Example: 99, 100 with 20 orders,  0.05 per step.
        // `num_orders` is cast to f64 so division works with decimal values.
            
        let mut price = self.upper_bound;

        while price >= self.lower_bound {
            let key_level = (price * 100.0) as i64; // we use * 100, to fix floating point errors (99.8 instead of 99.79 etc. 1 wouldnt work)
            let current_key = (self.current_price * 100.0) as i64;

            // trigger ONLY if price has crossed into or below level
            if current_key <= key_level && !self.executed_levels.contains(&key_level) { // this line asks current price below upper bound? and have we NOT traded here before (at this price)
                
                orders.push(TradeDecision {
                    action: OrderAction::Place(OrderSide::Buy),
                    price,
                });

                self.executed_levels.insert(key_level); // records that we have executed a trade at this particular level, ie: $98.0
            }

            price -= step; // move to the next lower price level --> price = 100, 99.90, 99.80 etc.
        }

        orders // returns our vector.
    }
}

// Our function looks like this
// For each grid level
// if price crosses level AND NOT executed
// Place order
// mark level as exectuted
// return all new orders.


use request::Client; // imports client type so can make HTTPS requests for API
use std::env;        // lets us access environment variables, allows access to api keys etc.
use serde::Deserialize; // used to convert JSON/Rust back n forth.