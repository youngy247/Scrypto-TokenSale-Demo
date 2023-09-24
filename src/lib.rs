use scrypto::prelude::*;

blueprint! {
    struct TokenSale {
        useful_tokens_value: Vault
    }

    impl TokenSale {
        pub fn new() -> ComponentAddress {
            let bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Useful Tokens")
                .metadata("symbol", "UT")
                .inital_supply(1000);

            Self { 
                useful_tokens_vault: Vault::with_bucket(bucket)
            }
            .instantiate()
            .globalize()
        }
    }
}
