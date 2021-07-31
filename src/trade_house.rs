use crate::agent::EconAgent;
use std::collections::HashMap;

pub struct Trade<'a> {
    commodity: String,
    price: f32,
    quantity: f32,
    agent: &'a EconAgent,
}
impl<'a> Trade<'a> {
    pub fn new(c: String, p: f32, q: f32, a: &'a EconAgent) -> Trade {
        Trade {
            commodity: c,
            price: p,
            quantity: q,
            agent: a,
        }
    }
}

pub struct TradeSubmission<'a> {
    trade_submission_hash: HashMap<String, Trade<'a>>,
}
impl<'a> TradeSubmission<'a> {
    pub fn new() -> TradeSubmission<'a> {
        TradeSubmission {
            trade_submission_hash: HashMap::new(),
        }
    }
    pub fn add(&mut self, s: String, t: Trade) {
        //self.trade_submission_hash.insert(s,t);
    }
}
