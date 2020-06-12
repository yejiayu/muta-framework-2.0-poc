use crate::primitive::{AccountID, Coin};

pub type ERC20ID = String;

pub struct ERC20 {
    id: ERC20ID,
    symbol: String,
    supply: Coin,
    owner: AccountID,
}

impl ERC20 {
    pub fn new(id: ERC20ID, symbol: String, supply: Coin, owner: AccountID) -> Self {
        Self {
            id,
            symbol,
            supply,
            owner,
        }
    }

    pub fn create(symbol: String, supply: Coin, owner: AccountID) -> Self {
        Self {
            id: symbol + &owner,
            symbol,
            supply,
            owner,
        }
    }

    pub fn mint(self, coin: Coin) -> Self {
        Self {
            supply: self.supply.deposit(coin),
            ..self
        }
    }

    pub fn burn(self, coin: Coin) -> Self {
        Self {
            supply: self.supply.withdraw(coin),
            ..self
        }
    }

    pub fn get_id(&self) -> &ERC20ID {
        &self.id
    }

    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }

    pub fn get_supply(&self) -> &Coin {
        &self.supply
    }

    pub fn get_owner(&self) -> &AccountID {
        &self.owner
    }
}

pub struct ERC20Balance {
    erc20: ERC20,
    amount: Coin,
}

impl ERC20Balance {
    pub fn new(erc20: ERC20, amount: Coin) -> Self {
        Self { erc20, amount }
    }

    pub fn transfer(self, from: ERC20Balance, value: Coin) -> (ERC20Balance, ERC20Balance) {
        let to = self;

        let from = from.withdraw(value.clone());
        let to = to.deposit(value);

        (to, from)
    }

    pub fn withdraw(self, value: Coin) -> Self {
        Self { 
            amount: self.amount.withdraw(value),
            ..self
        }
    }

    pub fn deposit(self, value: Coin) -> Self {
        Self { 
            amount: self.amount.deposit(value),
            ..self
        }
    }

    pub fn get_erc20(&self) -> &ERC20 {
        &self.erc20
    }

    pub fn get_amount(&self) -> &Coin {
        &self.amount
    }
}
