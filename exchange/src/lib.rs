#![allow(unused)]
extern crate strum;
extern crate strum_macros;

mod asset_name;
mod trader;
use crate::asset_name::*;
use std::fs::File;

#[cfg(test)]
mod tests {
    use super::*;
}
