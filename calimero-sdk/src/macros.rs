#[macro_export]
macro_rules! calimero_cross_call_execute {
    ($destination_contract_id: ident,
     $destination_contract_method: ident,
     $json_args: ident,
     $destination_gas: ident,
     $destination_deposit: ident,
     $callback_method_name: literal,
     $cross_call_gas: ident) => {
        env::promise_return(env::promise_create(
                AccountId::new_unchecked(CROSS_SHARD_CALL_CONTRACT_ID.to_string()
            ),
                "cross_call",
                &serde_json::to_vec(&(
                    $destination_contract_id,
                    $destination_contract_method,
                    $json_args.to_string(),
                    $destination_gas,
                    $destination_deposit,
                    $callback_method_name)).unwrap(),
                0,
                $cross_call_gas,
            ));
    }
}
