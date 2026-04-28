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
    current_price: f64,
    lower_bound: f64, // ie: $99
    upper_bound: f64, // ie: $100
    num_orders: u32, // total orders
}

// trade outputs 
struct TradeDecision {
    action: OrderAction,
    price: f64,
}

// logic for the trade
// note: Vec are like a growable list, (like an array that can expand)
// Our VECTOR returns a list of TradeDecisions.
impl TradeInput {
    fn generate_orders(&self) -> Vec<TradeDecision> {
        let mut orders = Vec::new();

        let step = (self.upper_bound - self.lower_bound) / self.num_orders as f64; 

// Calculates spacing between each order in the price range.
// Example: 99, 100 with 20 orders,  0.05 per step.
// `num_orders` is cast to f64 so division works with decimal values.
            
            
// .push() takes the value inside and adds it to the end of the Vector.
// TradeDecision {..} is our struct literal

            if self.current_price >= price {
                orders.push(TradeDecision {
                    action: OrderAction::Place(OrderSide::Buy),
                    price,
                });
            }
        }

        orders
    }






