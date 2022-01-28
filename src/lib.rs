use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, PanicOnDefault};

use std::collections::VecDeque;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    prices: VecDeque<u32>,
}


#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
    ) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");

        Self {
            prices: VecDeque::new(),
        }
    }

    pub fn add_price(&mut self, price: u32) -> u8 {
        let price_len = self.prices.len();
        if price_len > 4 {
            self.prices.pop_front();
        }
        self.prices.push_back(price);
        1
    }

    pub fn get_avg(&self) -> u32 {
        let price_len = self.prices.len() as u32;
        let sum :u32 = self.prices.iter().sum();
        sum/price_len
    }
}
