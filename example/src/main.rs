use near_sdk::{AccountId, Gas, Balance};
use near_sdk::serde_json::{json, self};
use calimero_sdk::{flip_arguments, use_calimero, calimero_response_test, calimero_receive_response1, calimero_receive_response};
use calimero_sdk::calimero_cross_call_execute;
use near_sdk::{env, require};


use_calimero!("bob.near");


#[flip_arguments]
fn my_function2(_lala: String, _mama: String) {
}

#[calimero_response_test]
fn my_function3() {
  println!("Code after injected code");
}

#[calimero_receive_response1]
fn start_game1(game_id: Option<usize>) {
  println!("GAME ID {:?}", game_id);
}

#[calimero_receive_response]
fn start_game(_: bool,  game_id: Option<usize>) {
  println!("GAME ID {:?}", game_id);
}

fn main() {
  my_function2("a".to_string(), "b".to_string());
  my_function3();

  const DESTINATION_GAS: Gas = Gas(20_000_000_000_000);
  const DESTINATION_DEPOSIT: Balance = 0;
  const CROSS_CALL_GAS: Gas = Gas(20_000_000_000_000);

  let sample_args = json!({"argument_x":"77"});

  let lala: &str = "a";
  calimero_cross_call_execute!(
    lala,
    lala,
    sample_args,
    DESTINATION_GAS,
    DESTINATION_DEPOSIT,
    "lala",
    CROSS_CALL_GAS);
}
