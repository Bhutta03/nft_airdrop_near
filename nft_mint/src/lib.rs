use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    total_supply: Balance,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            total_supply: 0,
        }
    }

    pub fn mint(&mut self) {
        assert_eq!(env::predecessor_account_id(), self.owner_id);
        self.total_supply += 1;
    }

    pub fn get_total_supply(&self) -> Balance {
        self.total_supply
    }
}