use crate::asset_name::AssetName;
use crate::deserialize::Deserialize;
use crate::order_matching_system::*;
use std::collections::{HashMap, BTreeMap};
use std::default;
use std::path::Path;
use std::str::FromStr;
use strum_macros::EnumString;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug, Default, Copy, Clone, EnumString, PartialEq)]
pub enum Direction {
    #[strum(serialize = "b")]
    #[default]
    Buy,
    #[strum(serialize = "s")]
    Sell
}

#[derive(Default)]
pub struct Order {
    pub id: usize,
    pub trader_name: String,
    pub direction: Direction,
    pub asset: AssetName,
    pub price: u64,
    pub amount: u64,
    pub limit: Option<Rc<RefCell<Limit>>>,
}

impl Deserialize<usize, Rc<RefCell<Order>>> for Order {

    fn deserialize(serialized_str: String) -> Rc<RefCell<Order>> {
        let parts: Vec<&str> = serialized_str.split(' ').collect();

        let trader_name: String = parts[0].to_string();
        let direction = Direction::from_str(parts[1]).unwrap();
        let asset = AssetName::from_str(parts[2]).unwrap();
        let price = parts[3].parse::<u64>().expect("Can't parse to u64");
        let amount = parts[4].parse::<u64>().expect("Can't parse to u64");

        let order = Order {
            id: usize::MAX,
            trader_name: trader_name,
            direction: direction,
            asset: asset,
            price: price,
            amount: amount,
            limit: None,
        };
        Rc::new(RefCell::new(order))
    }

    fn deserialize_all() -> BTreeMap<usize, Rc<RefCell<Order>>> {
        let mut orders = BTreeMap::new();
        let lines = Self::read_lines(Path::new("./resources/orders.txt"));
        for line in lines {
            if let Ok(serialized_order) = line {
                let mut order = Self::deserialize(serialized_order);
                order.borrow_mut().id = orders.len();
                orders.insert(order.borrow().id, order.clone());
            }
        }
        orders
    }
}

mod tests {
    use super::*;

    #[test]
    fn read_two_orders() {
        let orders = Order::deserialize_all();
        assert_eq!(orders.len(), 2);
    }

    #[test]
    fn first_order_has_all_fields_filled() {
        let orders = Order::deserialize_all();
        let order = &orders[&usize::MIN].borrow();
        assert_eq!(order.id, 0);
        assert_eq!(order.trader_name, "C1");
        assert_eq!(order.direction, Direction::Buy);
        assert_eq!(order.asset, AssetName::A);
        assert_eq!(order.price, 7);
        assert_eq!(order.amount, 12);
    }
}
