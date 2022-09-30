use asset_name::*;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Default)]
pub struct Trader {
    name: String,
    usd_balance: u64,
    assets_count: HashMap<AssetName, u64>,
}

pub struct FileDao {
    file: File,
    traders: HashMap<String, Trader>,
}

impl FileDao {

    fn deserialize(serialized_str: String) -> Trader {
        let parts: Vec<&str> = serialized_str.split(' ').collect();
        
        let trader_name: String = parts[0].to_string();
        let usd_balance = parts[1].parse::<u64>().expect("Can't parse to u64");
        let mut assets_count: HashMap<AssetName, u64> = HashMap::new();

        let mut asset_name = AssetName::A;
        while asset_name != AssetName::Unknown {
            let asset_count = parts[asset_name.index() + 2].parse::<u64>().expect("Can't parse to u64");
            assets_count.insert(asset_name, asset_count);
            asset_name = asset_name.next();            
        }

        Trader {
            name: trader_name,
            usd_balance: usd_balance,
            assets_count: assets_count,
        }
    }

    fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename).expect("Unable to open file");
        io::BufReader::new(file).lines()
    }

    fn read_traders() -> HashMap<String, Trader> {
        let mut traders = HashMap::new();

        let lines = Self::read_lines(Path::new("./clients.txt"));
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(serialized_trader) = line {
                let trader = Self::deserialize(serialized_trader);
                traders.insert(trader.name.clone(), trader);
            }
        }

        traders
    }
}

impl Default for FileDao {
    fn default() -> Self {
        let path = Path::new("./clients.txt");
        let p = env::current_dir().expect("errr");
        println!("The current directory is {}", p.display());
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(), why),
            Ok(file) => file,
        };
        FileDao {
            file: file,
            traders: HashMap::new(),
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn read_two_traders() {
        let file_dao = FileDao::default();
        let traders = FileDao::read_traders();
        assert_eq!(traders.len(), 2);
    }

    #[test]
    fn first_trader_has_all_fields_filled() {
        let file_dao = FileDao::default();
        let traders = FileDao::read_traders();
        let trader = &traders["C1"];
        
        assert_eq!(trader.name, "C1");
        assert_eq!(trader.usd_balance, 1000);
        assert_eq!(trader.assets_count.len(), 4);
        assert_eq!(trader.assets_count[&AssetName::A], 10);
    }

    #[test]
    fn deserialized_assets_len_equals_serialized() {
        let serialized_str = "C1 1000 10 5 15 0".to_string();
        let trader = FileDao::deserialize(serialized_str);
        let assets = trader.assets_count;

        assert_eq!(trader.name, "C1");
        assert_eq!(trader.usd_balance, 1000u64);
        assert_eq!(assets.len(), 4);
    }
}
