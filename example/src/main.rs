use near_sdk::{AccountId, Gas, Balance};
use near_sdk::serde_json::{json, self};
use near_sdk::near_bindgen;
use calimero_sdk::{flip_arguments, use_calimero, calimero_response_test, calimero_receive_response1, calimero_receive_response};
use calimero_sdk::calimero_cross_call_execute;
use calimero_sdk::{list_methods, list_methods_impl, list_methods_impl2};
use near_sdk::{env, require};


use_calimero!("bob.near");

#[near_bindgen]
struct MyStruct {
    field1: u64,
    field2: String,
}

//#[list_methods_impl2]
#[near_bindgen]
impl MyStruct {
    fn foo(&self) {}
    //#[calimero_response_test]
    fn bar(&mut self, broj: u64) {
        self.field1 = broj;
    }
}

// fn main() {
//     let s = MyStruct {
//         field1: 42,
//         field2: "hello".into(),
//     };
//     //println!("All methods: {:?}", s.list_all_methods());
// }

/*
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
}*/
