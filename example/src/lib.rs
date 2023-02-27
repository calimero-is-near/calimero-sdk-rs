use near_sdk::near_bindgen;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use calimero_sdk::{calimero_cross_shard_connector, calimero_cross_call_execute, calimero_expand};
use near_sdk::serde_json::{json, self};
use near_sdk::{AccountId, env, require, PanicOnDefault, Gas, Balance};

const DESTINATION_CONTRACT_ID: &str = "tictactoe.lal89.calimero.testnet"; // tictactoe on calimero
const DESTINATION_CONTRACT_METHOD: &str = "start_game";
const DESTINATION_GAS: Gas = Gas(50_000_000_000_000);
const DESTINATION_DEPOSIT: Balance = 0;
const CROSS_CALL_GAS: Gas = Gas(100_000_000_000_000);

calimero_cross_shard_connector!("xsc_connector.lal89.dev.calimero.testnet");

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct MyStruct {
    field1: u64,
    field2: String,
}

#[calimero_expand]
#[near_bindgen]
impl MyStruct {
    #[init]
    pub fn new() -> Self {
        Self {
            field1: 1,
            field2: "test".to_string(),
        }
    }

    #[calimero_receive_response]
    pub fn foo(&mut self, number: Option<u64>) {
        if number.is_none() {
            panic!("Failed extracting response");
        } else {
            self.field1 = number.unwrap();
        }
    }

    #[calimero_receive_response]
    pub fn bar(&mut self, number: u64) {
        self.field1 = number;
    }

    pub fn baz(&mut self, number: u64) {
        self.field1 = number;

        let args = json!({
            "player_a" : "player_a.testnet",
            "player_b" : env::predecessor_account_id()
        });

        calimero_cross_call_execute!(
            DESTINATION_CONTRACT_ID,
            DESTINATION_CONTRACT_METHOD,
            args,
            DESTINATION_GAS,
            DESTINATION_DEPOSIT,
            "game_started",
            CROSS_CALL_GAS
        );
    }
}
