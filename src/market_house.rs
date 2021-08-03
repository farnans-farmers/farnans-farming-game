use crate::agent::EconAgent;
use crate::COM_HASH;
use std::collections::HashMap;

extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

pub struct Trade {
    commodity: String,
    price: f32,
    quantity: f32,
    agent: EconAgent,
}
impl Trade {
    pub fn new(c: String, p: f32, q: f32, a: EconAgent) -> Trade {
        Trade {
            commodity: c,
            price: p,
            quantity: q,
            agent: a,
        }
    }
    pub fn reduce(&mut self, q: f32) {
        self.quantity -= q;
    }
}

pub struct TradeSubmission {
    trade_submission_hash: HashMap<String, Trade>,
}
impl TradeSubmission {
    pub fn new() -> TradeSubmission {
        TradeSubmission {
            trade_submission_hash: HashMap::new(),
        }
    }
    pub fn add(&mut self, s: String, t: Trade) {
        self.trade_submission_hash.insert(s, t);
    }
}

pub struct Trades {
    trades_hash: Vec<Trade>,
}
impl Trades {
    pub fn new() -> Trades {
        Trades {
            trades_hash: Vec::new(),
        }
    }
    pub fn add(&mut self, new_val: Trade) {
        self.trades_hash.push(new_val);
    }
    pub fn shuffle(&mut self) {
        let count = self.trades_hash.len();
        let last = count - 1;
        self.trades_hash.shuffle(&mut thread_rng());
    }
    pub fn len(&self) -> i32 {
        self.trades_hash.len() as i32
    }
}

pub struct TradeTable {
    trade_table_hash: HashMap<String, Trades>,
}
impl TradeTable {
    pub fn new() -> TradeTable {
        let mut trade_table_hash = HashMap::new();
        for (key, val) in COM_HASH.iter() {
            trade_table_hash.insert(key.clone(), Trades::new());
        }
        TradeTable {
            trade_table_hash: trade_table_hash,
        }
    }
    pub fn add(&mut self, trade_submission: TradeSubmission) {
        for (key, val) in trade_submission.trade_submission_hash.iter() {
            let mut temp_trades = Trades::new();
            //temp_trades.add(val.clone()); TODO
            self.trade_table_hash.insert(key.clone(), temp_trades);
        }
    }
}

pub struct MarketHouse {
    tick_interval: f32,
    num_agents: i32,
    init_cash: f32,
    init_stock: f32,
    max_stock: f32,
    agents: Vec<EconAgent>,
    ask_table: TradeTable,
    bid_table: TradeTable,
    track_bids: HashMap<String, HashMap<String, f32>>,
    last_tick: f32,
}

impl MarketHouse {
    pub fn new() -> MarketHouse {
        let mut track_bids: HashMap<String, HashMap<String, f32>> = HashMap::new();

        for (key, val) in COM_HASH.iter() {
            let mut hash_entry = HashMap::new();
            hash_entry.insert(key.clone(), 0.0);
            track_bids.insert(key.clone(), hash_entry);
        }

        MarketHouse {
            tick_interval: 0.1,
            num_agents: 100,
            init_cash: 100.0,
            init_stock: 15.0,
            max_stock: 20.0,
            agents: Vec::new(),
            ask_table: TradeTable::new(),
            bid_table: TradeTable::new(),
            track_bids: HashMap::new(),
            last_tick: 0.0,
        }
    }

    pub fn init_agent(&mut self, mut agent: EconAgent, agent_type: String) {
        let mut rng = rand::thread_rng();
        let mut buildables = Vec::new();
        buildables.push(agent_type);
        let _init_stock = rng.gen_range(self.init_stock..self.max_stock);
        let _max_stock = _init_stock.max(self.max_stock);

        agent.init_agent(self.init_cash, &buildables, _init_stock, _max_stock);
    }
    /*
    pub fn tick(&mut self){
        for agent in self.agents.iter_mut(){
            let (idle_tax,ts) = agent.produce_com_hash(0.0);
            self.ask_table.add(ts);
            self.bid_table.add(agent.consume_com_hash())
        }

        for (key,val) in COM_HASH.iter(){
            let (mut money_exchanged, mut goods_exchanged) = (0.0,0.0);
            let asks = self.ask_table.get(key.to_string()`);
            let bids = self.bid_table.get(key.to_string()`);
            let demand = bids.len() / (asks.len() as f32).max(0.01);

            let mut num_bids = 0.0;
            for bid in bids.iter(){
                num_bids += bid.quantity;
            }
            let mut num_asks = 0.0;
            for ask in asks.iter(){
                num_asks += ask.quantity;
            }

            val.add_bid(num_bids);
            val.add_ask(num_asks);
        }
    }
    */
}
