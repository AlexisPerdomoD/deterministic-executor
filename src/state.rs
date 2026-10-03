use std::collections::HashMap;

type AccountCode = String;

#[derive(Clone)]
pub struct Account {
    pub bal: u128,
    pub nonce: u64,
}

#[derive(Clone, Default)]
pub struct State {
    owner_code: Option<String>,
    accounts: HashMap<AccountCode, Account>,
}

impl State {
    pub fn owner_code(&self) -> Option<&str> {
        self.owner_code.as_deref()
    }

    pub fn account(&self, code: &str) -> Option<&Account> {
        self.accounts.get(code)
    }

    pub(crate) fn account_mut(&mut self, code: &str) -> Option<&mut Account> {
        self.accounts.get_mut(code)
    }

    pub fn builder() -> StateBuilder {
        StateBuilder::default()
    }
}

#[derive(Default)]
pub struct StateBuilder {
    owner_code: Option<String>,
    accounts: Option<HashMap<AccountCode, Account>>,
}

impl StateBuilder {
    pub fn owner_code(mut self, owner_code: String) -> Self {
        self.owner_code = Some(owner_code);
        self
    }

    pub fn accounts(mut self, accounts: HashMap<AccountCode, Account>) -> Self {
        self.accounts = Some(accounts);
        self
    }

    pub fn build(self) -> State {
        State {
            owner_code: self.owner_code,
            accounts: self.accounts.unwrap_or_default(),
        }
    }
}
