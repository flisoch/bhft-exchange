use crate::order::*;
use crate::trader::Trader;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Limit {
    price: u64,
    volume: u64,
    orders: VecDeque<Rc<RefCell<Order>>>,
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
        if (self.limits.contains_key(&order.borrow().price)) {
            if let Some(limit) = self.limits.get_mut(&order.borrow().price) {
                limit.borrow_mut().volume += order.borrow().amount;
                limit.borrow_mut().orders.push_back(order.clone());
            }
        } else {
            let price = order.borrow().price;
            let mut limit = Limit {
                price: price,
                volume: order.borrow().amount,
                orders: VecDeque::new(),
            };
            limit.orders.push_back(order.clone());
            self.limits.insert(price, Rc::new(RefCell::new(limit)));
        }
    }
    pub fn market(
        &mut self,
        mut order: Rc<RefCell<Order>>,
        users: &mut BTreeMap<String, Rc<RefCell<Trader>>>,
        orders: &mut BTreeMap<usize, Rc<RefCell<Order>>>,
    ) {
        let limit_order_id: usize;
        let market_order_id: usize;

        while (!self.limits.is_empty()
            && Self::matched(
                order.borrow().price,
                self.limits.iter().next().unwrap().0.clone(),
                order.borrow().direction,
            ))
        {
            let mut order_ref = order.borrow_mut();

            let matched_limit = self.limits.iter().next().as_ref().unwrap().1.clone();
            let matched_limit_ref = matched_limit.borrow_mut();
            let mut matched_order = matched_limit_ref.orders.front().unwrap().clone();
            let mut matched_order_ref = matched_order.borrow_mut();

            if (matched_order_ref.amount >= order_ref.amount) {
                market_order_id = matched_order_ref.id;
                limit_order_id = order_ref.id;

                if (matched_order_ref.amount == order_ref.amount) {
                    drop(matched_limit_ref);
                    drop(order_ref);
                    drop(matched_order_ref);

                    self.finish(matched_limit);
                    self.on_fill(market_order_id, limit_order_id, users, orders);
                } else {
                    matched_order_ref.amount -= order_ref.amount;
                    drop(order_ref);
                    drop(matched_order_ref);
                    self.on_fill(limit_order_id, market_order_id, users, orders);
                }
                return;
            }
            drop(order_ref);
            drop(matched_order_ref);
            drop(matched_limit_ref);
            self.finish(matched_limit);
            self.on_fill(matched_order.borrow().id, order.borrow().id, users, orders);
            order.borrow_mut().amount -= matched_order.borrow().amount;
        }
    }

    fn matched(limit: u64, market: u64, direction: Direction) -> bool {
        match direction {
            Direction::Sell => limit <= market,
            Direction::Buy => limit >= market,
        }
    }

    fn finish(&mut self, limit: Rc<RefCell<Limit>>) {
        if (limit.borrow().orders.len() == 1) {
            self.limits.remove(&limit.borrow().price);
        } else {
            limit.borrow_mut().orders.pop_front();
        }
    }

    fn on_fill(
        &mut self,
        filled_order_id: usize,
        donor_order_id: usize,
        traders: &mut BTreeMap<String, Rc<RefCell<Trader>>>,
        orders: &mut BTreeMap<usize, Rc<RefCell<Order>>>,
    ) {
        if (filled_order_id != donor_order_id) {
            let filled_order = orders[&filled_order_id].clone();
            let filled_order_ref = filled_order.borrow();
            let donor_order = orders[&donor_order_id].clone();
            let donor_order_ref = donor_order.borrow();

            let mut filled_trader = traders[&filled_order_ref.trader_name].borrow_mut();
            let mut donor_trader = traders[&donor_order_ref.trader_name].borrow_mut();

            if (filled_order_ref.direction == Direction::Buy) {
                *filled_trader
                    .assets_count
                    .get_mut(&filled_order_ref.asset)
                    .unwrap() += filled_order_ref.amount;
                donor_trader.usd_balance += filled_order_ref.price * filled_order_ref.amount;
            } else {
                filled_trader.usd_balance += filled_order_ref.amount * filled_order_ref.price;
                *donor_trader
                    .assets_count
                    .get_mut(&filled_order_ref.asset)
                    .unwrap() += filled_order_ref.amount;
                donor_trader.usd_balance +=
                    (donor_order_ref.price - filled_order_ref.price) * filled_order_ref.amount;
            }
        }
        orders.remove(&filled_order_id);
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
