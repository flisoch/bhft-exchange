use crate::order::*;
use crate::trader::Trader;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};

pub struct Limit {
    price: u64,
    volume: u64,
    //orders live longer than limits, so there's a weak ref
    orders: VecDeque<Weak<RefCell<Order>>>,
}

pub struct LimitTree {
    pub limits: BTreeMap<u64, Rc<RefCell<Limit>>>,
    direction: Direction,
}

impl LimitTree {
    pub fn empty(&self) -> bool {
        self.limits.is_empty()
    }

    pub fn new(direction: Direction) -> Self {
        Self {
            limits: BTreeMap::new(),
            direction: direction,
        }
    }
    pub fn new_limit(&mut self, mut order: Rc<RefCell<Order>>) {
        let mut order_ref = order.borrow_mut();
        if (self.limits.contains_key(&order_ref.price)) {
            if let Some(limit) = self.limits.get_mut(&order_ref.price) {
                limit.borrow_mut().volume += order_ref.amount;
                limit.borrow_mut().orders.push_back(Rc::downgrade(&order));
                order_ref.limit = Some(limit.clone());
            }
        } else {
            let price = order_ref.price;
            let mut limit = Limit {
                price: price,
                volume: order_ref.amount,
                orders: VecDeque::new(),
            };
            limit.orders.push_back(Rc::downgrade(&order));
            self.limits.insert(price, Rc::new(RefCell::new(limit)));
        }
    }
    pub fn market(
        &mut self,
        mut order: Rc<RefCell<Order>>,
        users: &mut HashMap<String, Rc<RefCell<Trader>>>,
        orders: &mut HashMap<usize, Rc<RefCell<Order>>>,
    ) {
        let mut order_ref = order.borrow_mut();
        while (Self::matched(
            order_ref.price,
            self.limits.iter().next().unwrap().0.clone(),
            order_ref.direction,
        )) {
            let matched_limit = self.limits.iter().next().unwrap().1;
            let matched_order = matched_limit
                .borrow_mut()
                .orders
                .front()
                .unwrap()
                .upgrade()
                .unwrap();
            if (matched_order.borrow().amount >= order_ref.amount) {
                if (matched_order.borrow().amount == order_ref.amount) {
                    self.finish(&matched_order);
                    self.on_fill(matched_order.borrow().id, order_ref.id, users, orders);
                } else {
                    matched_order.borrow_mut().amount -= order_ref.amount;
                }
                self.on_fill(order_ref.id, order_ref.id, users, orders);
                return;
            }

            self.finish(&matched_order);
            self.on_fill(matched_order.borrow().id, order_ref.id, users, orders);
            order_ref.amount -= matched_order.borrow().amount;
        }
    }

    fn matched(limit: u64, market: u64, direction: Direction) -> bool {
        match direction {
            Direction::Sell => limit <= market,
            Direction::Buy => limit >= market,
        }
    }

    fn finish(&mut self, order: &Rc<RefCell<Order>>) {
        let order_ref = order.borrow();
        let mut limit = order_ref.limit.as_ref().unwrap();
        if (limit.borrow().orders.len() == 1) {
            self.limits.remove(&limit.borrow().price);
        } else {
            limit.borrow_mut().orders.pop_front();
        }
    }

    fn on_fill(&mut self,
        market_order_id: usize,
        limit_order_id: usize,
        traders: &mut HashMap<String, Rc<RefCell<Trader>>>,
        orders: &mut HashMap<usize, Rc<RefCell<Order>>>,
    ) {
        if (market_order_id != limit_order_id) {
            let mut market_order = orders[&market_order_id].clone();
            let mut new_order = orders[&limit_order_id].clone();
            let mut market_trader = traders[&market_order.borrow().trader_name].borrow_mut();
            let mut new_trader = traders[&new_order.borrow().trader_name].borrow_mut();

            if (market_order.borrow().direction == Direction::Buy) {
                *market_trader.assets_count.get_mut(&market_order.borrow().asset).unwrap() += market_order.borrow().amount;
                new_trader.usd_balance += market_order.borrow().price * market_order.borrow().amount;
            }
            else {
                market_trader.usd_balance += market_order.borrow().amount * market_order.borrow().price;
                *new_trader.assets_count.get_mut(&market_order.borrow().asset).unwrap() += market_order.borrow().amount;
                new_trader.usd_balance += (new_order.borrow().price - market_order.borrow().price) * market_order.borrow().amount;
            }
            
            orders.remove(&market_order_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use super::*;

    #[test]
    fn new_order_inserter_to_empty_limit_tree() {
        let order = Order::default();
        let mut limit_tree = LimitTree {
            limits: BTreeMap::new(),
            direction: Direction::Buy,
        };
        limit_tree.new_limit(Rc::new(RefCell::new(order)));
        assert_eq!(limit_tree.limits.len(), 1);
    }

    #[test]
    fn order_with_same_price_inserted_to_one_limit() {
        let mut order = Order {
            id: 0,
            price: 10,
            amount: 1,
            ..Default::default()
        };
        let mut order1 = Order {
            id: 1,
            price: 10,
            amount: 2,
            ..Default::default()
        };
        let mut limit_tree = LimitTree {
            limits: BTreeMap::new(),
            direction: Direction::Buy,
        };
        limit_tree.new_limit(Rc::new(RefCell::new(order)));
        limit_tree.new_limit(Rc::new(RefCell::new(order1)));

        assert_eq!(limit_tree.limits.len(), 1);
    }

    #[test]
    fn matched_fn_returns_correct_vlues() {
        let market = 10;
        let limit = 20;
        assert_eq!(LimitTree::matched(limit, market, Direction::Sell), false);
        assert_eq!(LimitTree::matched(limit, market, Direction::Buy), true);
        assert_eq!(LimitTree::matched(market, limit, Direction::Buy), false);
        assert_eq!(LimitTree::matched(market, limit, Direction::Sell), true);
    }
}
