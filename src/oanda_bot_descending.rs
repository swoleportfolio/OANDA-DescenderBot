use std::env;

let api_key = env::var("OANDA_API_KEY")
    .expect("API key not set");

struct TradeInputs {
    price: u32,
    buy: String,
    sell: String,
    stoploss: String,
    cancelorder: String,

}


fn main () {




}

