use crate::agent::EconAgent;
use crate::commodities::CommodityKind;

extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

pub struct Trade {
    commodity: CommodityKind,
    price: f32,
    quantity: f32,
    agent_idx: usize,
}
impl Trade {
    pub fn new(c: CommodityKind, p: f32, q: f32, agent_idx: usize) -> Trade {
        Trade {
            commodity: c,
            price: p,
            quantity: q,
            agent_idx,
        }
    }
    pub fn reduce(&mut self, q: f32) {
        self.quantity -= q;
    }
}

pub struct TradeSubmission {
    submission: Vec<Option<Trade>>,
}
impl TradeSubmission {
    pub fn new() -> TradeSubmission {
        let mut submission = Vec::with_capacity(CommodityKind::SIZE);
        for _i in 0..CommodityKind::SIZE {
            submission.push(None);
        }
        TradeSubmission { submission }
    }
    pub fn add(&mut self, s: CommodityKind, t: Trade) {
        self.submission[s as usize] = Some(t);
    }
}

pub struct TradeTable {
    trades: Vec<Vec<Trade>>,
}

impl TradeTable {
    pub fn new() -> TradeTable {
        let mut trades = Vec::with_capacity(CommodityKind::SIZE);
        for _i in 0..CommodityKind::SIZE {
            trades.push(Vec::new());
        }
        TradeTable { trades }
    }

    pub fn shuffle(&mut self) {
        for t in self.trades.iter_mut() {
            t.shuffle(&mut thread_rng());
        }
    }

    pub fn add(&mut self, trade_submission: TradeSubmission) {
        for i in 0..CommodityKind::SIZE {
            if let Some(t) = &trade_submission.submission[i] {
                let t = Trade::new(t.commodity, t.price, t.quantity, t.agent_idx);
                self.trades[i].push(t);
            }
        }
    }
}

pub struct MarketHouse {
    ask_table: TradeTable,
    bid_table: TradeTable,
}

impl MarketHouse {
    pub fn new() -> MarketHouse {
        MarketHouse {
            ask_table: TradeTable::new(),
            bid_table: TradeTable::new(),
        }
    }

    pub fn resolve(&mut self) {}
}
