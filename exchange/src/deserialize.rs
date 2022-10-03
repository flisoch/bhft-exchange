use std::collections::{HashMap, BTreeMap};
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

pub trait Deserialize<IdType, T> {
    fn deserialize(serialized_str: String) -> T;
    fn deserialize_all() -> BTreeMap<IdType, T>;

    fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename).expect("Unable to open file");
        io::BufReader::new(file).lines()
    }
}
