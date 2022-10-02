#![allow(unused)]
extern crate strum;
extern crate strum_macros;

mod asset_name;
mod trader;
mod deserialize;
mod order;

use crate::asset_name::*;
use std::fs::File;
use crate::deserialize::*;

#[cfg(test)]
mod tests {
    use super::*;
}
