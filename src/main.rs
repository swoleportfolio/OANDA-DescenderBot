// README:
// This bot is very simple and deterministic.
// We enter orders at a price statically, while the bot allows us to implement those trades n number of times
// without have to manually input each order.
// This bot can also use DCA (or a decrement like) to add positions
// ie: Our upper bound is 100, our lower bound is 99, the bot will place orders incrementally based on the amount of orders in the range of 99 to 100
// so if you placed 20 orders in the range of 100 to 99, you would place them every 5 cents.




use reqwest::Client; // imports client type so can make HTTPS requests for API
use std::env;        // lets us access environment variables, allows access to api keys etc.
use serde::Deserialize; // used to convert JSON -> Rust structs
use std::collections::HashSet;

// Structs that match OANDA's JSON response
#[derive(Debug, Deserialize)]
struct AccountList {
    accounts: Vec<Account>,
}


#[derive(Debug, Deserialize)]
struct Account {
    id: String,
}


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

    executed_levels: HashSet<i64>, // doesnt allow duplicate elements (good for keeping memory/track of already made orders)
}

// trade outputs 
struct TradeDecision {
    action: OrderAction,
    price: f64,
}


#[derive(Deserialize)]
struct OrderResponse {
    orderCreateTransaction: Option<OrderTransaction>,
    errorMessage: Option<String>,
}

#[derive(Deserialize)]
struct OrderTransaction {
    id: String,
    time: String,
    instrument: String,
    price: Option<String>,
    units: String,
}


impl TradeInput {
    fn generate_orders(&mut self, current_price: f64) -> Vec<TradeDecision> {
        let mut orders = Vec::new();

        // avoid division by zero if num_orders == 0 or 1
        if self.num_orders < 2 {
            return orders;
        }

        let step = (self.upper_bound - self.lower_bound) / (self.num_orders - 1) as f64;
        let current_key = (current_price * 100.0).round() as i64;

        for i in 0..self.num_orders {
            let price = self.upper_bound - i as f64 * step;
            let key_level = (price * 100.0).round() as i64;

            // trigger when price has traded into or below this level
            if current_key <= key_level && !self.executed_levels.contains(&key_level) {
                orders.push(TradeDecision {
                    action: OrderAction::Place(OrderSide::Buy),
                    price,
                });

                self.executed_levels.insert(key_level);
            }
        }

        orders
    }
}


pub async fn execute_order(
    client: &Client,
    api_key: &str,
    account_id: &str,
    decision: &TradeDecision,
) -> Result<(), Box<dyn std::error::Error>> {

    match decision.action {
        OrderAction::Place(OrderSide::Buy) => {
            let body = serde_json::json!({ // creates JSON object, allows rust expressions inside JSON, produces a serde_json::Value.
                "order": {
                    "units": "1",
                    "instrument": "EUR_USD",
                    "timeInForce": "GTC",
                    "type": "LIMIT",
                    "price": format!("{:.5}", decision.price), // format is string placeholder --> ie: this will bring in our price to 5 decimal places.
                    "positionFill": "DEFAULT"
                }
            });

            let url = format!(
                "https://api-fxpractice.oanda.com/v3/accounts/{}/orders",
                account_id
            );

            println!("POST {}", url);



            let res = client
                .post(&url)
                .header("Authorization", format!("Bearer {}", api_key))
                .json(&body)
                .send()
                .await?;

            let status = res.status();
            let text = res.text().await?;

            if !status.is_success() {
                println!("Order failed ({}): {}", status, text);
                return Ok(());
            }

            println!("Sending order: {}", body);
            println!("Order success: {}", text);
        }

        _ => {
            
        }
    }

    Ok(())
}


#[tokio::main] // sets up async runtime so we can use .await
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // API KEY input from environment
    let api_key = env::var("OANDA_API_KEY")
        .expect("OANDA_API_KEY must be set");

    let client = Client::new(); // creates an HTTP client

    // think logically through let res = client, how the network moves:
    // 1. build request (method + URL)
    // 2. attach headers (auth)
    // 3. send over network (DNS, TCP, TLS)
    // 4. receive response (status + headers + body stream)
    let res = client
        .get("https://api-fxpractice.oanda.com/v3/accounts") // GET request to OANDA
        .header("Authorization", format!("Bearer {}", api_key)) // authentication header
        .send() // send request
        .await?; // wait for response (async)

    // check status BEFORE consuming response
    if !res.status().is_success() {
        println!("Request failed: {}", res.status());
        return Ok(()); // early exit
    }

    // instead of raw text, deserialize JSON into Rust struct
    let data: AccountList = res.json().await?;

    // now working with structured data (not raw JSON)
   for acc in &data.accounts {
    println!("Account ID: {}", acc.id);
}

    // 1. get account
    let account_id = &data.accounts[0].id;

// 2. create strategy
    

let mut strat = TradeInput {
    lower_bound: 99.0,
    upper_bound: 100.0,
    num_orders: 20,
    executed_levels: HashSet::new(), // Hash set allows for non duplicate elements.
};

// 3. get price (for now hardcoded)
    let current_price = 99.5;

// 4. generate decisions
    let decisions = strat.generate_orders(current_price);

// 5. EXECUTE decisions
    for decision in decisions {
    execute_order(&client, &api_key, account_id, &decision).await?;
}

    Ok(()) // program completed successfully
}


// current layout: 

// Grid Strategy

// TradeDecision Vec
   
// execute_order()
   
// OANDA API