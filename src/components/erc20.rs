use std::collections::HashMap;

use crate::components::primitive::{AccountID, Coin};

pub type ERC20ID = String;

#[derive(Clone)]
pub struct ERC20 {
    id: ERC20ID,
    symbol: String,
    supply: Coin,
    owner: AccountID,
    approved_coin: HashMap<AccountID, Coin>,
}

impl ERC20 {
    pub fn new(id: ERC20ID, symbol: String, supply: Coin, owner: AccountID) -> Self {
        Self {
            id,
            symbol,
            supply,
            owner,
            approved_coin: HashMap::new(),
        }
    }

    pub fn create(symbol: String, supply: Coin, owner: AccountID) -> Self {
        Self {
            id: symbol.clone() + &owner,
            symbol,
            supply,
            owner,
            approved_coin: HashMap::new(),
        }
    }

    pub fn mint(&mut self, coin: Coin) {
        self.supply.deposit(coin);
    }

    pub fn burn(&mut self, coin: Coin) {
        self.supply.withdraw(coin);
    }

    pub fn get_id(&self) -> &ERC20ID {
        &self.id
    }

    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }

    pub fn get_supply(&self) -> Coin {
        self.supply
    }

    pub fn get_owner(&self) -> &AccountID {
        &self.owner
    }
}

#[derive(Clone)]
pub struct ERC20Balance {
    erc20: ERC20,
    amount: Coin,
}

impl ERC20Balance {
    pub fn new(erc20: ERC20, amount: Coin) -> Self {
        Self { erc20, amount }
    }

    pub fn transfer(&mut self, to: &mut ERC20Balance, value: Coin) {
        to.deposit(value);
        self.withdraw(value);
    }

    pub fn withdraw(&mut self, value: Coin) {
        self.amount.withdraw(value);
    }

    pub fn deposit(&mut self, value: Coin) {
        self.amount.deposit(value);
    }

    pub fn get_erc20(&self) -> &ERC20 {
        &self.erc20
    }

    pub fn get_amount(&self) -> Coin {
        self.amount
    }
}
