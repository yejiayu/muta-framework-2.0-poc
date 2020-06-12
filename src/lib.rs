mod components;

use std::collections::HashMap;

use crate::components::erc20::{ERC20Balance, ERC20, ERC20ID};
use crate::components::primitive::{AccountID, Coin};

pub struct Repository {
    pub registered_ERC20: HashMap<ERC20ID, ERC20>,
    pub accounts: HashMap<AccountID, Account>,
}

impl Repository {
    pub fn new() -> Self {
        Self {
            registered_ERC20: HashMap::new(),
            accounts: HashMap::new(),
        }
    }
}

pub struct Account {
    id: AccountID,
    assets: HashMap<ERC20ID, ERC20Balance>,
}

impl Account {
    pub fn new(id: AccountID) -> Self {
        Self {
            id,
            assets: HashMap::new(),
        }
    }

    pub fn deposit(&mut self, erc20: &ERC20, value: Coin) {
        let balance = self
            .assets
            .entry(erc20.get_id().to_owned())
            .or_insert(ERC20Balance::new(erc20.clone(), Coin::new(0)));

        balance.deposit(value);
    }

    pub fn withdraw(&mut self, erc20: &ERC20, value: Coin) {
        let balance = self.assets.get_mut(erc20.get_id()).expect("account withdraw");
        balance.withdraw(value);
    }

    pub fn transfer(&mut self, to: &mut Account, erc20: &ERC20, value: Coin) {
        to.deposit(erc20, value);
        self.withdraw(erc20, value);
    }

    pub fn get_id(&self) -> &AccountID {
        &self.id
    }

    pub fn get_balance(&self, erc20: &ERC20) -> Coin {
        match self.assets.get(erc20.get_id()) {
            Some(balance) => balance.get_amount(),
            None => Coin::new(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::components::erc20::ERC20;
    use crate::components::primitive::{AccountID, Coin};
    use crate::{Account, Repository};

    const ADMIN_ACCOUNT_ID: &str = "admin";
    const USER_FOO_ACCOUNT_ID: &str = "user_foo";

    #[test]
    fn erc20_test() {
        // create global state
        let mut global_repo = Repository::new();

        // create account
        let mut admin_account = Account::new(ADMIN_ACCOUNT_ID.to_owned());
        let mut user_account = Account::new(USER_FOO_ACCOUNT_ID.to_owned());

        // create muta token of erc20
        let muta_token = ERC20::create(
            "muta-token".to_owned(),
            Coin::new(1024),
            admin_account.get_id().to_owned(),
        );
        admin_account.deposit(&muta_token, muta_token.get_supply());
        let muta_token_id = muta_token.get_id().clone();
        global_repo
            .registered_ERC20
            .insert(muta_token.get_id().to_owned(), muta_token);
        let muta_token = global_repo.registered_ERC20.get(&muta_token_id).unwrap();

        // transfer
        admin_account.transfer(&mut user_account, &muta_token, Coin::new(1024 / 2));

        println!("admin token {:?} user token {:?}", admin_account.get_balance(&muta_token), user_account.get_balance(&muta_token));
    }
}
