use crate::agent::EconAgent;
use crate::commodities::CommodityKind;

extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

pub struct Trade {
    commodity: CommodityKind,
    price: f32,
    quantity: i32,
    agent_idx: usize,
}
impl Trade {
    pub fn new(c: CommodityKind, p: f32, q: i32, agent_idx: usize) -> Trade {
        Trade {
            commodity: c,
            price: p,
            quantity: q,
            agent_idx,
        }
    }
    pub fn reduce(&mut self, q: i32) {
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

    pub fn shuffle(&mut self, c: CommodityKind) {
        let t = &mut self.trades[c as usize];
        t.shuffle(&mut thread_rng());
    }

    pub fn sort_asc(&mut self, c: CommodityKind) {
        let t = &mut self.trades[c as usize];
        t.sort_unstable_by(|t1, t2| t1.price.partial_cmp(&t2.price).unwrap())
    }

    pub fn sort_desc(&mut self, c: CommodityKind) {
        let t = &mut self.trades[c as usize];
        t.sort_unstable_by(|t1, t2| t2.price.partial_cmp(&t1.price).unwrap())
    }

    pub fn is_empty(&self, c: CommodityKind) -> bool {
        self.trades[c as usize].is_empty()
    }

    pub fn add(&mut self, trade_submission: TradeSubmission) {
        for i in 0..CommodityKind::SIZE {
            if let Some(t) = &trade_submission.submission[i] {
                let t = Trade::new(t.commodity, t.price, t.quantity, t.agent_idx);
                self.trades[i].push(t);
            }
        }
    }

    pub fn pop(&mut self, c: CommodityKind) -> Trade {
        if let Some(t) = self.trades[c as usize].pop() {
            return t;
        }
        panic!("pop from empty trade table")
    }

    pub fn push(&mut self, c: CommodityKind, trade: Trade) {
        self.trades[c as usize].push(trade);
    }

    pub fn clear(&mut self, c: CommodityKind) {
        self.trades[c as usize].clear();
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

    fn resolve(&mut self, c: CommodityKind, agents: &mut Vec<EconAgent>) {
        // Explicitly shuffle so that agents don't have an unfair advantage for
        // being born earlier.
        self.bid_table.shuffle(c);
        self.ask_table.shuffle(c);
        // The paper sorts these in the opposite respective orders and removes
        // the first trade on each iteration. We instead take the last item, so
        // that removing isn't quadratic time.
        self.bid_table.sort_asc(c);
        self.ask_table.sort_desc(c);
        while !self.bid_table.is_empty(c) && !self.ask_table.is_empty(c) {
            let mut buy = self.bid_table.pop(c);
            let mut sell = self.ask_table.pop(c);
            let q = std::cmp::min(sell.quantity, buy.quantity);
            let p = (sell.price + buy.price) / 2.0;
            if q > 0 {
                buy.reduce(q);
                let buyer = &mut agents[buy.agent_idx];
                buyer.buy("this is not correct".into(), q as f32, p);
                // TODO: buy doesn't update price beliefs

                sell.reduce(q);
                let seller = &mut agents[sell.agent_idx];
                seller.sell("this is not correct".into(), q as f32, p);
                // TODO: sell doesn't update price beliefs
            }
            // The paper only removes trades when they're done. We remove
            // trades every time and push them back on if they aren't clear.
            // No difference, just maybe convenience wrt borrow checker.
            if buy.quantity > 0 {
                self.ask_table.push(c, buy);
            }
            if sell.quantity > 0 {
                self.bid_table.push(c, sell);
            }
        }
        // TODO: remaining offers are rejected
        self.bid_table.clear(c);
        self.ask_table.clear(c);
    }

    pub fn resolve_all(&mut self, agents: &mut Vec<EconAgent>) {
        let cs = [
            CommodityKind::CarrotSeed,
            CommodityKind::CarrotCrop,
            CommodityKind::CornSeed,
            CommodityKind::CornCrop,
            CommodityKind::PotatoSeed,
            CommodityKind::PotatoCrop,
            CommodityKind::LettuceSeed,
            CommodityKind::LettuceCrop,
        ];
        for c in cs {
            self.resolve(c, agents);
        }
    }
}
