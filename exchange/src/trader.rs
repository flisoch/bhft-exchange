use asset_name::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::deserialize::Deserialize;

#[derive(Default)]
pub struct Trader {
    id: usize,
    name: String,
    usd_balance: u64,
    assets_count: HashMap<AssetName, u64>,
}

impl Deserialize<String, Trader> for Trader {
    fn deserialize(serialized_str: String) -> Trader {
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

        Trader {
            name: trader_name,
            usd_balance: usd_balance,
            assets_count: assets_count,
            ..Default::default()
        }
    }

    fn deserialize_all() -> HashMap<String, Trader> {
        let mut traders = HashMap::new();
        let lines = Self::read_lines(Path::new("./resources/clients.txt"));
        for line in lines {
            if let Ok(serialized_trader) = line {
                let mut trader = Self::deserialize(serialized_trader);
                trader.id = traders.len();
                traders.insert(trader.name.clone(), trader);
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
        let trader = &traders["C1"];

        assert_eq!(trader.name, "C1");
        assert_eq!(trader.usd_balance, 1000);
        assert_eq!(trader.assets_count.len(), 4);
        assert_eq!(trader.assets_count[&AssetName::A], 10);
    }

    #[test]
    fn deserialized_assets_len_equals_serialized() {
        let serialized_str = "C1 1000 10 5 15 0".to_string();
        let trader = Trader::deserialize(serialized_str);
        let assets = trader.assets_count;

        assert_eq!(trader.name, "C1");
        assert_eq!(trader.usd_balance, 1000u64);
        assert_eq!(assets.len(), 4);
    }
}
