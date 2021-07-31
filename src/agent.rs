//TODO change borrow references

use crate::trade_house::{Trade, TradeSubmission};
use crate::commodities::Commodity;
use rand::Rng;
use std::cmp::{max, min};
use std::collections::HashMap;

pub struct History {
    items: Vec<f32>,
    min: f32,
    max: f32,
}

impl History {
    pub fn new() -> History {
        History {
            items: Vec::new(),
            min: 0.0,
            max: 0.0,
        }
    }
    pub fn add(&mut self, new_val: f32) {
        self.items.push(new_val);
    }
    pub fn get_min(&self) -> f32 {
        self.min
    }
    pub fn get_max(&self) -> f32 {
        self.max
    }
}

pub struct CommodityStock {
    commodity_name: String,
    quantity: f32,
    max_quantity: f32,
    mean_cost: f32,
    production: f32,
    min_price_belief: f32,
    max_price_belief: f32,
    price_history: History,
    significant: f32,
    sig_imbalance: f32,
    low_inventory: f32,
    high_inventory: f32,
    cost: f32,
    wobble: f32,
    production_rate: f32,
}

impl CommodityStock {
    pub fn new(
        _name: String,
        _quantity: f32,
        _max_quantity: f32,
        _mean_price: f32,
        _production: f32,
    ) -> CommodityStock {
        let mut price_history: History = History::new();
        price_history.add(_mean_price);

        CommodityStock {
            commodity_name: _name,
            quantity: _quantity,
            max_quantity: _max_quantity,
            mean_cost: _mean_price,
            production: _production,
            min_price_belief: _mean_price / 2.0,
            max_price_belief: _mean_price * 2.0,
            price_history: price_history,

            significant: 0.25,
            sig_imbalance: 0.33,
            low_inventory: 0.1,
            high_inventory: 2.0,
            cost: 1.0,
            wobble: 0.02,
            production_rate: 1.0,
        }
    }

    pub fn tick(&self) {
        println!("TODO");
    }

    pub fn increase_quantity(&mut self, val: f32) {
        self.quantity += val;
    }

    pub fn buy(&mut self, quant: f32, price: f32) -> f32 {
        let total_cost = self.mean_cost * self.quantity + price * quant;
        self.mean_cost = total_cost / self.quantity;
        let mut left_over = quant - self.deficit();
        self.quantity += quant;
        if left_over > 0.0 {
            println!(
                "Bought too much! Max: {} {} \nleftover:{}",
                self.quantity, quant, left_over
            );
        } else {
            left_over = 0.0;
        }
        self.update_price_belief(false, price, true);
        quant
    }
    pub fn sell(&mut self, quant: f32, price: f32) {
        self.quantity = quant;
        self.update_price_belief(true, price, true);
    }
    pub fn get_price(&mut self) -> f32 {
        self.sane_price_beliefs();
        let mut rng = rand::thread_rng();
        //let p: f32 = (rng.gen() as f32)*(self.max_price_belief-self.min_price_belief) + self.min_price_belief;
        let p = rng.gen_range(self.min_price_belief..self.max_price_belief) as f32;
        if p > 1000.0 {
            println!(
                "price beliefs {}, {}",
                self.min_price_belief, self.max_price_belief
            )
        }
        p
    }
    fn sane_price_beliefs(&mut self) {
        //self.min_price_belief = max(self.cost,self.min_price_belief);
        //self.max_price_belief = max(self.min_price_belief*1.1,self.max_price_belief);
        self.min_price_belief = self.min_price_belief.max(self.cost);
        self.max_price_belief = self.max_price_belief.max(self.min_price_belief * 1.1);
        self.min_price_belief = self.min_price_belief.clamp(0.1, 900.0);
        self.max_price_belief = self.max_price_belief.clamp(0.1, 900.0);
    }
    pub fn update_price_belief(&mut self, is_sell: bool, price: f32, success: bool) {
        self.sane_price_beliefs();
        if self.min_price_belief > self.max_price_belief {
            println!(
                "{} ERROR {} > {}",
                self.commodity_name, self.min_price_belief, self.max_price_belief
            );
        }
        self.price_history.add(price);

        let buy = !is_sell;
        let mean = (self.min_price_belief + self.max_price_belief) / 2.0;
        let delta_mean = mean - price;

        if success {
            if (is_sell && delta_mean < -self.significant * mean)
                || (buy && delta_mean > self.significant * mean)
            {
                self.min_price_belief -= delta_mean / 4.0;
                self.max_price_belief -= delta_mean / 4.0;
            }
            self.min_price_belief += self.wobble * mean;
            self.max_price_belief -= self.wobble * mean;

            if self.min_price_belief > self.max_price_belief {
                let avg = (self.min_price_belief + self.max_price_belief) / 2.0;
                self.min_price_belief = avg * (1.0 - self.wobble);
                self.max_price_belief = avg * (1.0 + self.wobble);
            }
            self.wobble = self.wobble / 2.0;
        } else {
            self.min_price_belief -= delta_mean / 4.0;
            self.max_price_belief -= delta_mean / 4.0;
            if (buy && self.quantity < self.max_quantity * self.low_inventory)
                || (is_sell && self.quantity > self.max_quantity * self.low_inventory)
            {
                println!("TODO");
            } else {
                println!("TODO");
            }
            self.min_price_belief -= self.wobble * mean;
            self.max_price_belief += self.wobble * mean
        }

        if self.min_price_belief < self.max_price_belief {
            self.min_price_belief = self.max_price_belief / 2.0;
        }

        self.sane_price_beliefs();
    }
    pub fn deficit(&self) -> f32 {
        (self.max_quantity - self.quantity).max(0.0)
    }
    pub fn surplus(&self) -> f32 {
        self.quantity
    }
}

pub struct EconAgent {
    debug: i32,
    cash: f32,
    prev_cash: f32,
    max_stock: f32,
    profits: Vec<f32>, //TODO ESList?
    stock_pile: HashMap<String, CommodityStock>,
    stock_pile_cost: HashMap<String, f32>,
    buildables: Vec<String>,
    com: HashMap<String, Commodity>,
    bankruptcy_threshold: f32,
    history_count: i32,
}

impl EconAgent {
    pub fn new() -> EconAgent {
        EconAgent {
            debug: 0,
            cash: 0.0,
            prev_cash: 0.0,
            max_stock: 1.0,
            profits: Vec::new(),
            stock_pile: HashMap::new(),
            stock_pile_cost: HashMap::new(),
            buildables: Vec::new(),
            com: HashMap::new(),
            bankruptcy_threshold: -200.0,
            history_count: 10,
        }
    }

    pub fn start(&mut self) {
        self.cash = 0.0
    }

    pub fn add_to_stock_pile(
        &mut self,
        name: String,
        num: f32,
        max: f32,
        price: f32,
        production: f32,
    ) {
        if !self.stock_pile.contains_key(&name) {
            self.stock_pile.insert(
                name.clone(),
                CommodityStock::new(name.clone(), num, max, price, production),
            );
        }
        self.stock_pile_cost.insert(
            name.clone(),
            self.com.get_mut(&name).unwrap().get_price() * num,
        );
    }

    //TODO
    pub fn init_agent(&mut self, init_cash: f32, b: Vec<String>, init_num: f32, _max_stock: f32) {
        /*
        if self.com.len() == 0{
            self.buildables = b;
            self.cash = init_cash;
            self.prev_cash = self.cash;
            self.max_stock = _max_stock;

            for_each!(buildable in buildables{
                if(!com.contains_key(buildable)){
                    println!("Commodity not recognized {}",buildable);
                }

            });
        }
        */
    }

    pub fn tax_profit(&mut self, tax_rate: f32) -> f32 {
        let profit = self.get_profit();
        if profit <= 0.0 {
            return profit;
        }
        let tax_amount = profit * tax_rate;
        self.cash -= tax_amount;
        profit - tax_amount
    }

    pub fn get_profit(&mut self) -> f32 {
        let profit = self.cash - self.prev_cash;
        self.prev_cash = self.cash;
        profit
    }

    pub fn is_bankrupt(&mut self) -> bool {
        self.cash < self.bankruptcy_threshold
    }
    //TODO
    pub fn tick(&self) {}
    pub fn buy(&mut self, commodity: String, quantity: f32, price: f32) -> f32 {
        let bought_quantity = self
            .stock_pile
            .get_mut(&commodity)
            .unwrap()
            .buy(quantity, price);
        self.cash -= price * bought_quantity;
        bought_quantity
    }
    pub fn sell(&mut self, commodity: String, quantity: f32, price: f32) {
        self.stock_pile
            .get_mut(&commodity)
            .unwrap()
            .sell(-quantity, price);
        self.cash += price * quantity;
    }
    pub fn reject_ask(&mut self, commodity: String, price: f32) {
        self.stock_pile
            .get_mut(&commodity)
            .unwrap()
            .update_price_belief(true, price, false);
    }
    pub fn reject_bid(&mut self, commodity: String, price: f32) {
        self.stock_pile
            .get_mut(&commodity)
            .unwrap()
            .update_price_belief(false, price, false);
    }
    fn find_sell_count(&mut self, c: String) -> f32 {
        let avg_price = self
            .com
            .get_mut(&c)
            .unwrap()
            .get_avg_price(self.history_count);
        let lowest_price = self.stock_pile.get_mut(&c).unwrap().price_history.get_min();
        let highest_price = self.stock_pile.get_mut(&c).unwrap().price_history.get_max();
        let mut favorability: f32 = 0.5; // TODO LERP 1.0/avg_price.lerp(lowest_price,highest_price);
        favorability = favorability.clamp(0.0, 1.0);
        let num_asks = favorability * self.stock_pile.get_mut(&c).unwrap().surplus();
        num_asks.max(1.0)
    }
    fn find_buy_count(&mut self, c: String) -> f32 {
        let avg_price = self
            .com
            .get_mut(&c)
            .unwrap()
            .get_avg_price(self.history_count);
        let lowest_price = self.stock_pile.get_mut(&c).unwrap().price_history.get_min();
        let highest_price = self.stock_pile.get_mut(&c).unwrap().price_history.get_max();
        let mut favorability: f32 = 0.5; // TODO LERP 1.0/avg_price.lerp(lowest_price,highest_price);
        favorability = favorability.clamp(0.0, 1.0);
        let num_bids = (1.0 - favorability) * self.stock_pile.get_mut(&c).unwrap().deficit();
        num_bids.max(1.0)
    }
    pub fn consume(&mut self, com: HashMap<String, Commodity>) -> TradeSubmission {
        let mut bids = TradeSubmission::new();
        let keys = self.stock_pile.keys().cloned().collect::<Vec<_>>();
        for key in keys {
            if self.buildables.contains(&key) {
                continue;
            }
            let num_bids = self.find_buy_count(key.clone());
            let value = self.stock_pile.get_mut(&key).unwrap();

            if num_bids > 0.0 {
                let buy_price = value.get_price();
                if buy_price > 1000.0 {
                    println!(
                        "{} buy price: {} : {}",
                        key.clone(),
                        buy_price,
                        value.min_price_belief
                    );
                }
                if num_bids < 0.0 {
                    println!("{} buying negative {} for {}", key, num_bids, buy_price);
                }
                bids.add(
                    key.clone(),
                    Trade::new(value.commodity_name.clone(), buy_price, num_bids, &*self),
                );
            }
        }
        /*
        for (key,value) in self.stock_pile.iter(){
            if self.buildables.contains(&key){
                continue;
            }
            let num_bids = self.find_buy_count(key.clone());
            if num_bids > 0.0{
                let buy_price = value.get_price();
                if buy_price > 1000.0{
                    println!("{} buy price: {} : {}",key.clone(),buy_price,value.min_price_belief);
                }
                if num_bids < 0.0{
                    println!("{} buying negative {} for {}",key,num_bids,buy_price);
                }
                bids.add(key.clone(), Trade::new(value.commodity_name.clone(),buy_price,num_bids, &*self));
            }
        }
        */
        bids
    }
    //TODO
    pub fn produce(
        &mut self,
        mut com: HashMap<String, Commodity>,
        mut idle_tax: f32,
    ) -> TradeSubmission {
        let mut asks = TradeSubmission::new();

        /*
        for buildable in self.buildables.iter_mut(){
            let mut num_produced = f32::MAX;

            if(!self.com.contains_key(buildable)){
                println!("not a commodity {}", buildable);
            }
            for (key,value) in com.get_mut(buildable).unwrap().dep.dependency_map.iter(){
                let num_needed = value;
                let num_avail = self.stock_pile.get(&*key).unwrap().quantity;
                num_produced = num_produced.min(num_avail/num_needed);
            }

            let upper_bound = self.
                stock_pile.get(buildable).unwrap().production_rate.min(self.stock_pile.get(buildable).unwrap().deficit());
            num_produced = num_produced.clamp(0.0,upper_bound);
            for (key,value) in com.get(buildable).unwrap().dep.dependency_map.iter(){
                let stock = self.stock_pile.get(&*key).unwrap().quantity;
                let mut num_used = value * num_produced;
                num_used = num_used.clamp(0.0,stock);
                self.stock_pile.get_mut(&*key).unwrap().increase_quantity(-num_used);
            }
            num_produced *= self.stock_pile.get(buildable).unwrap().production;
            num_produced = num_produced.max(0.0);
            self.stock_pile.get_mut(buildable).unwrap().increase_quantity(num_produced);

            let build_stock = self.stock_pile.get_mut(buildable).unwrap();
            //TODO look at
            let sell_price = self.find_sell_count(buildable.to_string().clone());
            build_stock.get_price();

            if num_produced > 0.0 && sell_price > 0.0{
                asks.add(buildable.to_string(), Trade::new(buildable.to_string(),sell_price,build_stock.quantity, &*self));
            }
            else{
                idle_tax = self.cash.abs() * 0.05;
                self.cash -= idle_tax;
            }
        }
        */
        asks
    }
    //TODO
    fn get_cost_of(commodity: String) -> f32 {
        0.0
    }
}
