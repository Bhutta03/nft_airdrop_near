use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Promise};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct NftAirdrop {
    pub owner_id: AccountId,
    pub contract_id: AccountId,
    pub token_ids: Vec<String>,
}

#[near_bindgen]
impl NftAirdrop {
    #[init]
    pub fn new(owner_id: AccountId, contract_id: AccountId) -> Self {
        Self {
            owner_id,
            contract_id,
            token_ids: vec![],
        }
    }

    pub fn mint_tokens(&mut self, token_ids: Vec<String>) {
        self.token_ids = token_ids;
    }

    pub fn airdrop(&self, recipient_ids: Vec<AccountId>) {
        for recipient_id in &recipient_ids {
            for token_id in &self.token_ids {
                let promise = Promise::new(self.contract_id.clone())
                    .function_call(
                        b"nft_transfer".to_vec(),
                        format!(
                            r#"{{"receiver_id": "{}", "token_id": "{}", "memo": "Airdrop"}}"#,
                            recipient_id, token_id
                        )
                        .into_bytes(),
                        0,
                        300_000_000_000_000,
                    );
                promise.and(Promise::new(env::current_account_id()));
            }
        }
    }
}
