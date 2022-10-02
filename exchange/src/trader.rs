use asset_name::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::rc::Rc;

use crate::deserialize::Deserialize;
use crate::order::{Direction, Order};

#[derive(Default)]
pub struct Trader {
    pub id: usize,
    pub name: String,
    pub usd_balance: u64,
    pub assets_count: HashMap<AssetName, u64>,
}

impl Trader {
    pub fn block_funds(&mut self, order: Rc<RefCell<Order>>) {
        // todo: if order funds greater than user's, update order to user max available resources
        // or throw exceptions
        if (order.borrow().direction == Direction::Sell) {
            *self.assets_count.get_mut(&order.borrow().asset).unwrap() -= order.borrow().amount;
        }
        else {
            self.usd_balance -= order.borrow().amount * order.borrow().price;
        }
    }
}

impl Deserialize<String, Rc<RefCell<Trader>>> for Trader {
    fn deserialize(serialized_str: String) -> Rc<RefCell<Trader>> {
        let parts: Vec<&str> = serialized_str.split(' ').collect();

        let trader_name: String = parts[0].to_string();
        let usd_balance = parts[1].parse::<u64>().expect("Can't parse to u64");
        let mut assets_count: HashMap<AssetName, u64> = HashMap::new();

        let mut asset_name = AssetName::A;
        while asset_name != AssetName::Unknown {
            let asset_count = parts[asset_name.index() + 2]
                .parse::<u64>()
                .expect("Can't parse to u64");
            assets_count.insert(asset_name, asset_count);
            asset_name = asset_name.next();
        }

        let trader = Trader {
            name: trader_name,
            usd_balance: usd_balance,
            assets_count: assets_count,
            ..Default::default()
        };
        Rc::new(RefCell::new(trader))
    }

    fn deserialize_all() -> HashMap<String, Rc<RefCell<Trader>>> {
        let mut traders = HashMap::new();
        let lines = Self::read_lines(Path::new("./resources/clients.txt"));
        for line in lines {
            if let Ok(serialized_trader) = line {
                let mut trader = Self::deserialize(serialized_trader);
                trader.borrow_mut().id = traders.len();
                traders.insert(trader.borrow().name.clone(), trader.clone());
            }
        }
        traders
    }
}

mod tests {
    use super::*;

    #[test]
    fn read_two_traders() {
        let traders = Trader::deserialize_all();
        assert_eq!(traders.len(), 2);
    }

    #[test]
    fn first_trader_has_all_fields_filled() {
        let traders = Trader::deserialize_all();
        let trader = &traders["C1"].borrow();

        assert_eq!(trader.name, "C1");
        assert_eq!(trader.usd_balance, 1000);
        assert_eq!(trader.assets_count.len(), 4);
        assert_eq!(trader.assets_count[&AssetName::A], 10);
    }

    #[test]
    fn deserialized_assets_len_equals_serialized() {
        let serialized_str = "C1 1000 10 5 15 0".to_string();
        
        let trader = Trader::deserialize(serialized_str);
        let assets = &trader.borrow().assets_count;

        assert_eq!(trader.borrow().name, "C1");
        assert_eq!(trader.borrow().usd_balance, 1000u64);
        assert_eq!(assets.len(), 4);
    }

    #[test]
    fn usd_balance_changes_after_buy() {
        let order = Order {direction: Direction::Buy, amount: 12, price: 7, ..Default::default()};
        let mut trader = Trader {usd_balance: 1000, ..Default::default()};
        
        trader.block_funds(Rc::new(RefCell::new(order)));

        assert_eq!(trader.usd_balance, 1000 - 12*7);
    }

    #[test]
    fn asset_amount_changes_after_sell() {
        let order = Order {direction: Direction::Sell, asset:AssetName::A, amount: 10, ..Default::default()};
        let assets: HashMap<AssetName, u64> = [(AssetName::A, 10)].iter().cloned().collect();
        let mut trader = Trader {assets_count: assets, ..Default::default()};
        
        trader.block_funds(Rc::new(RefCell::new(order)));

        assert_eq!(trader.assets_count.iter().next().unwrap().1.clone(), 0);
    }
}
