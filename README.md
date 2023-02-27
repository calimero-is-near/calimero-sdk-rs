# Usage

To specify which cross shard call contract connector your contract will use. Inside your NEAR contract .rs file add at top:
```
calimero_cross_shard_connector!("xsc_connector.lal89.dev.calimero.testnet");
```

To trigger a cross call execute call this macro:
```
calimero_cross_call_execute!(
    DESTINATION_CONTRACT_ID,
    DESTINATION_CONTRACT_METHOD,
    args,
    DESTINATION_GAS,
    DESTINATION_DEPOSIT,
    "game_started",
    CROSS_CALL_GAS
);
```

To specify a method that receives response from other chain use the ``#[calimero_receive_response]`` proc_macro_attribute
Also, the ``#[calimero_expand]`` needs to be called before ``#[near_bindgen]`` in order to expand ``calimero_receive_response`` first
for proper functioning on NEAR contracts

E.g.
```
#[calimero_expand]
#[near_bindgen]
impl MyContract {

    #[calimero_receive_response]
    pub fn foo(&mut self, arg1: Option<u64>) {
        if arg1.is_none() {
            // handle cross shard contract None response
            // e.g. panic!("Failed extracting response");
        } else {
            // arg1.unwrap() has the deserialized cross shard call execution result
            // code ...
        }
    }

    #[calimero_receive_response]
    pub fn bar(&mut self, bar_arg: usize) {
        // either panics or cross shard contract execution deserialized into bar_arg
        // code ...
    }
}
```
