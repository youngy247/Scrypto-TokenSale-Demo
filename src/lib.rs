use scrypto::prelude::*;

blueprint! {
    struct TokenSale {
        useful_tokens_value: Vault,
        xrd_tokens_vault: Vault,
        price_per_token: Decimal,
    }

    impl TokenSale {
        pub fn new(price_per_token: Decimal) -> ComponentAddress {
            let bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Useful Tokens")
                .metadata("symbol", "UT")
                .inital_supply(1000);

            Self { 
                useful_tokens_vault: Vault::with_bucket(bucket),
                xrd_tokens_vault: Vault::new(RADIX_TOKEN),
                price_per_token: price_per_token,
            }
            .instantiate()
            .globalize()
        }

        pub fn buy(&mut self, funds: Bucket) -> Bucket {
            let purchase_amount: Decimal = funds.amount() / self.price_per_token;
            self.xrd_tokens_vault.put(funds);
            self.useful_tokens_vault.take(purchase_amount);
        }
    }
}
