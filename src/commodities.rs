use crate::COM_HASH;
use std::collections::HashMap;

pub struct ESList {
    avg: f32,
    eslist: Vec<f32>,
}
impl ESList {
    pub fn new() -> ESList {
        ESList {
            avg: 0.0,
            eslist: Vec::new(),
        }
    }
    pub fn last_average(&mut self, history: i32) -> f32 {
        if self.eslist.len() == 0 {
            return 0.0;
        }
        let skip =
            ((self.eslist.len() as i32) - 1).min(0.max((self.eslist.len() as i32) - history));
        let end = ((self.eslist.len() as i32) - 1).min(history);
        if skip < end {
            let mut sum_val = 0.0;
            for x in skip..end {
                sum_val += self.eslist[x as usize];
            }
            self.avg = sum_val / ((end - skip) as f32);
        }
        self.avg
    }
    pub fn add(&mut self, val: f32) {
        self.eslist.push(val);
    }
    pub fn count(&mut self) -> i32 {
        self.eslist.len() as i32
    }
}

pub struct Commodity {
    bids: ESList,
    asks: ESList,
    prices: ESList,
    trades: ESList,
    profits: ESList,
    avg_price: f32,
    first_avg_price: bool,
    debug: i32,
    name: String,
    price: f32,
    demand: f32,
    production: f32,
    dep: Dependency,
}
impl Commodity {
    pub fn new(n: String, p: f32, d: Dependency) -> Commodity {
        let mut _bids = ESList::new();
        let mut _asks = ESList::new();
        let mut _prices = ESList::new();
        let mut _trades = ESList::new();
        let mut _profits = ESList::new();

        _bids.add(1.0);
        _asks.add(1.0);
        _prices.add(1.0);
        _trades.add(1.0);
        _profits.add(1.0);

        Commodity {
            bids: _bids,
            asks: _asks,
            prices: _prices,
            trades: _trades,
            profits: _profits,
            avg_price: 1.0,
            first_avg_price: true,
            debug: 0,
            name: n,
            price: 1.0,
            demand: 1.0,
            production: p,
            dep: d,
        }
    }
    pub fn get_avg_price(&mut self, history: i32) -> f32 {
        if self.first_avg_price {
            self.first_avg_price = false;
            let skip = (self.prices.count() - history).max(0);
            let end = self.prices.count();
            let prices_after_skip: Vec<f32> = self.prices.eslist[(skip as usize)..].to_vec();
            let prices_after_skip_len = prices_after_skip.len() as i32;
            let mut sum_val = 0.0;
            for x in skip..end {
                sum_val += self.prices.eslist[x as usize];
            }
            self.avg_price = sum_val / ((end - skip) as f32);
        }
        self.avg_price
    }
    pub fn update(&mut self, p: f32, dem: f32) {
        self.first_avg_price = true;
        self.price = p;
        self.demand = dem;
    }
    pub fn get_price(&mut self) -> f32 {
        self.price
    }
    pub fn add_bid(&mut self, new_val: f32) {
        self.bids.add(new_val);
    }
    pub fn add_ask(&mut self, new_val: f32) {
        self.asks.add(new_val);
    }
}

pub struct Dependency {
    dependency_map: HashMap<String, f32>,
}

impl Dependency {
    pub fn new() -> Dependency {
        Dependency {
            dependency_map: HashMap::new(),
        }
    }
    pub fn add(&mut self, name: String, quantity: f32) {
        self.dependency_map.insert(name, quantity);
    }
}

pub struct Commodities {
    com: HashMap<String, Commodity>,
}
impl Commodities {
    pub fn new() -> Commodities {
        Commodities {
            com: HashMap::new(),
        }
    }
    fn awake(&mut self) {
        //instance = self;
        self.com = HashMap::new();
        //self.init();
    }
    pub fn add(&mut self, name: String, production: f32, dep: Dependency) -> bool {
        if self.com.contains_key(&name) {
            return false;
        }
        self.com
            .insert(name.clone(), Commodity::new(name.clone(), production, dep));
        true
    }
}
