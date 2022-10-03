use exchange::{deserialize::Deserialize, order::Order, order_matching_system::order_book::*, trader::Trader};

fn main() {
    let mut order_book = OrderBook {
        users: Trader::deserialize_all(),
        ..Default::default()
    };
    let orders = Order::deserialize_all();

    Trader::serialize_all(&order_book.users);

    for (_key, order) in orders {
        order_book.limit(&order.clone());
    }
    
    Trader::serialize_all(&order_book.users);
}
