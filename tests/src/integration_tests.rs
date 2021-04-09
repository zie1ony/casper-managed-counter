#[cfg(test)]
mod tests {
    use casper_engine_test_support::{Code, Hash, SessionBuilder, TestContextBuilder};
    use casper_types::{
        account::AccountHash, runtime_args, PublicKey, RuntimeArgs, SecretKey, U512,
    };

    #[test]
    fn should_store_hello_world() {
        // Prepare the account.
        let public_key: PublicKey = SecretKey::ed25519([7u8; 32]).into();
        let account_addr = AccountHash::from(&public_key);

        let mut context = TestContextBuilder::new()
            .with_public_key(public_key, U512::from(500_000_000_000_000_000u64))
            .build();

        // Deploy the main contract.
        let session_code = Code::from("managed_counter.wasm");
        let session = SessionBuilder::new(session_code, RuntimeArgs::new())
            .with_address(account_addr)
            .with_authorization_keys(&[account_addr])
            .build();
        context.run(session);

        // Call the manager contract to create a new counter contract.
        let session_code = Code::NamedKey(String::from("manager"), String::from("new_counter"));
        let session_args = runtime_args! {
            "name" => String::from("counter_one")
        };
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(account_addr)
            .with_authorization_keys(&[account_addr])
            .build();
        context.run(session);

        // Ready counter_one's hash.
        let counter_one_hash: Hash = context
            .query(
                account_addr,
                &[String::from("manager"), String::from("counter_one_hash")],
            )
            .unwrap()
            .into_t()
            .unwrap();

        // Increment counter 3 times.
        for _ in 0..3 {
            let session_code = Code::Hash(counter_one_hash, String::from("increment"));
            let session = SessionBuilder::new(session_code, RuntimeArgs::new())
                .with_address(account_addr)
                .with_authorization_keys(&[account_addr])
                .build();
            context.run(session);
        }

        // Read counter_one's value.
        let counter_one_value: u32 = context
            .query(
                account_addr,
                &[
                    String::from("manager"),
                    String::from("counter_one"),
                    String::from("counter_value"),
                ],
            )
            .unwrap()
            .into_t()
            .unwrap();

        
        // Expect the counter to be incremented.
        assert_eq!(counter_one_value, 3);
    }
}

fn main() {
    panic!("The main should not be used here");
}