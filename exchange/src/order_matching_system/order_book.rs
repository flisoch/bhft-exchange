use crate::order::*;
use crate::order_matching_system::limit_tree::*;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::deserialize::Deserialize;
use crate::trader::Trader;

pub struct OrderBook {
    pub buy_limits: LimitTree,
    pub sell_limits: LimitTree,
    pub orders: BTreeMap<usize, Rc<RefCell<Order>>>,
    pub users: BTreeMap<String, Rc<RefCell<Trader>>>,
}

impl OrderBook {
    pub fn from_files() -> Self {
        Self {
            buy_limits: LimitTree::new(Direction::Buy),
            sell_limits: LimitTree::new(Direction::Sell),
            orders: Order::deserialize_all(),
            users: Trader::deserialize_all(),
        }
    }

    pub fn limit(&mut self, order: &Rc<RefCell<Order>>) {
        self.orders.insert(order.borrow().id, order.clone());
        self.users[&order.borrow().trader_name]
            .as_ref()
            .borrow_mut()
            .block_funds(order.clone());
        if (order.borrow().direction == Direction::Buy) {
            self.limit_buy(order.clone());
        } else {
            self.limit_sell(order.clone());
        }
    }

    fn limit_sell(&mut self, order: Rc<RefCell<Order>>) {
        if (!self.buy_limits.empty()) {
            let buys_best_price = *self.buy_limits.borrow_mut().limits.iter().next().unwrap().0;

            if (order.borrow().price <= buys_best_price) {
                //todo make closure or smth to not pass users&orders but pass function on_fill with scope captured
                self.buy_limits
                    .market(order.clone(), &mut self.users, &mut self.orders);
                if (self.orders.contains_key(&order.borrow().id) && order.borrow().amount > 0) {
                    self.sell_limits.new_limit(order.clone());
                }
            }
        } else {
            self.sell_limits.new_limit(order.clone());
        }
    }

    fn limit_buy(&mut self, order: Rc<RefCell<Order>>) {
        if (!self.sell_limits.empty()) {
            let sell_best_price = *self.sell_limits.borrow_mut().limits.iter().next().unwrap().0;

            if (order.borrow().price >= sell_best_price) {
                self.sell_limits
                    .market(order.clone(), &mut self.users, &mut self.orders);
                if (self.orders.contains_key(&order.borrow().id) && order.borrow().amount > 0) {
                    self.buy_limits.new_limit(order.clone());
                }
            }
        } else {
            self.buy_limits.new_limit(order.clone());
        }
    }
}

impl Default for OrderBook {
    fn default() -> Self {
        Self {
            buy_limits: LimitTree::new(Direction::Buy),
            sell_limits: LimitTree::new(Direction::Sell),
            orders: Default::default(),
            users: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{asset_name::AssetName, deserialize::Deserialize, order, trader::Trader};

    use super::*;

    #[test]
    fn orderbook_orders_len_increased_after_new_order_inserted() {
        let mut order_book = OrderBook {
            users: Trader::deserialize_all(),
            ..Default::default()
        };
        let orders = Order::deserialize_all();
        order_book.limit(&orders.iter().next().unwrap().1.clone());
        assert_eq!(order_book.orders.len(), 1);
    }

    #[test]
    fn trader_balance_changed_after_new_buy_order_inserted() {
        let mut orderbook = OrderBook::from_files();
        let balance_before = orderbook.users["C1"].borrow().usd_balance;
        orderbook.limit(&orderbook.orders[&usize::MIN].clone());

        let order: &Rc<RefCell<Order>> = &orderbook.orders[&usize::MIN];
        assert_eq!(
            orderbook.users["C1"].borrow().usd_balance,
            balance_before - order.as_ref().borrow().price * order.as_ref().borrow().amount
        );
    }
}
