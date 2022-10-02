#![allow(unused)]
extern crate strum;
extern crate strum_macros;

mod asset_name;
mod trader;
mod deserialize;
mod order;
mod order_matching_system;

use std::fs::File;

#[cfg(test)]
mod tests {
    use super::*;
}
