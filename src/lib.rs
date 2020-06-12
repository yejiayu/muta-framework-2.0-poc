mod erc20;
mod primitive;

use std::collections::HashMap;

use crate::primitive::AccountID;
use crate::erc20::{ERC20, ERC20Balance, ERC20ID};

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

    pub fn (self, ) -> Self {
        self.assets.insert(balance.get_erc20().get_id().to_owned(), balance);
        Self {
            assets: self.assets,
            ..self
        }
    }

    pub fn get_id(&self) -> &AccountID {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{Repository, Account};
    use crate::erc20::{ERC20};
    use crate::primitive::{AccountID, Coin};

    const ADMIN_ACCOUNT_ID: AccountID  = "admin".to_owned();
    const USER_FOO_ACCOUNT_ID: AccountID  = "user_foo".to_owned();

    #[test]
    fn erc20_test() {
        // create global state
        let global_repo = Repository::new();
    
        // create muta token of erc20
        let admin_account = Account::new(ADMIN_ACCOUNT_ID.to_owned());
        let muta_token = ERC20::create("muta-token".to_owned(), Coin::new(1024), admin_account.get_id().to_owned());
        admin_account.deposit_erc20()
        
        global_repo.registered_ERC20.insert(muta_token.get_id().to_owned(), muta_token);

        // transfer
        ERC20::new(, symbol: String, supply: Coin, owner: AccountID)
        assert_eq!(2 + 2, 4);
    }
}
