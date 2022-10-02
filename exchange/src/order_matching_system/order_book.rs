use order::*;
use order_matching_system::limit_tree::*;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::rc::{Rc, Weak};

use crate::trader::Trader;

struct OrderBook<'a> {
    buy_limits: LimitTree,
    sell_limits: LimitTree,
    orders: BTreeMap<usize, Rc<RefCell<Order>>>,
    users: Option<&'a HashMap<String, Trader>>,
}

impl OrderBook<'_> {
    pub fn limit(&mut self, order: Rc<RefCell<Order>>) {
        self.orders.insert(order.borrow().id, order.clone());

    }
    fn limit_sell(&self, order: Rc<RefCell<Order>>) {}
    fn limit_buy(&self, order: Rc<RefCell<Order>>) {}
}

impl Default for OrderBook<'_> {
    fn default() -> Self {
        Self {
            buy_limits: LimitTree::new(Direction::Buy),
            sell_limits: LimitTree::new(Direction::Sell),
            orders: Default::default(),
            users: None,
        }
    }
}
#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use crate::{asset_name::AssetName, deserialize::Deserialize, trader::Trader};

    use super::*;

    #[test]
    fn orderbook_orders_len_increased_after_new_order_inserted() {
        let mut orderbook = OrderBook::default();
        let order = Order::default();
        orderbook.limit(Rc::new(RefCell::new(order)));
        assert_eq!(orderbook.orders.len(), 1);
    }

    #[test]
    fn trader_balance_changed_after_new_order_inserted() {
        let mut orderbook = OrderBook::default();
        let traders = Trader::deserialize_all();
        let trader = traders.iter().next().unwrap().1;

        let order = Rc::new(RefCell::new(Order::default()));
        let mut order_ref = order.borrow_mut();
        order_ref.trader_name = trader.name.clone();
        order_ref.asset = AssetName::A;
        order_ref.price = 7;
        order_ref.amount = 12;
        orderbook.limit(order.clone());
        assert_eq!(trader.usd_balance, 1000 - order_ref.price * order_ref.amount);
    }
}
