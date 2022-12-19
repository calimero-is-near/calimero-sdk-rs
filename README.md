# Usage

To specify which cross shard call contract connector your contract will use. Inside your NEAR contract .rs file add at top:
```
use_calimero!("xsc_connector.rs1.dev.calimero.testnet");
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

E.g.
```
#[calimero_receive_response]
pub fn game_started(&mut self, game_id: Option<usize>) {
  // code ...
}
```
