#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod market_maker {
    use ink_storage::collections::HashMap;

    #[ink(storage)]
    pub struct MarketMaker {
        owner: AccountId,
        eth_reserve: Balance,
        token_reserve: Balance,
        token_balances: HashMap<AccountId, Balance>,
    }

    #[ink(event)]
    pub struct LiquidityAdded {
        #[ink(topic)]
        provider: AccountId,
        eth_amount: Balance,
        token_amount: Balance,
    }

    #[ink(event)]
    pub struct TokensSwapped {
        #[ink(topic)]
        swapper: AccountId,
        eth_in: Balance,
        token_out: Balance,
    }

    #[ink(event)]
    pub struct ETHSwapped {
        #[ink(topic)]
        swapper: AccountId,
        token_in: Balance,
        eth_out: Balance,
    }

    impl MarketMaker {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                eth_reserve: 0,
                token_reserve: 0,
                token_balances: HashMap::new(),
            }
        }

        #[ink(message)]
        pub fn add_liquidity(&mut self, token_amount: Balance) -> bool {
            let caller = self.env().caller();
            let eth_amount = self.env().transferred_balance();

            assert!(token_amount > 0 && eth_amount > 0, "Invalid amounts");

            self.eth_reserve += eth_amount;
            self.token_reserve += token_amount;

            self.token_balances.insert(caller, token_amount);

            self.env().emit_event(LiquidityAdded {
                provider: caller,
                eth_amount,
                token_amount,
            });

            true
        }

        #[ink(message)]
        pub fn swap_eth_for_tokens(&mut self) -> Balance {
            let caller = self.env().caller();
            let eth_in = self.env().transferred_balance();
            assert!(eth_in > 0, "Invalid ETH amount");

            let token_out = self.get_token_price(eth_in);
            assert!(token_out <= self.token_reserve, "Not enough liquidity");

            self.eth_reserve += eth_in;
            self.token_reserve -= token_out;

            let caller_balance = self.token_balances.get(&caller).unwrap_or(&0);
            self.token_balances
                .insert(caller, caller_balance + token_out);

            self.env().emit_event(TokensSwapped {
                swapper: caller,
                eth_in,
                token_out,
            });

            token_out
        }

        #[ink(message)]
        pub fn swap_tokens_for_eth(&mut self, token_in: Balance) -> Balance {
            let caller = self.env().caller();
            assert!(token_in > 0, "Invalid token amount");

            let eth_out = self.get_eth_price(token_in);
            assert!(eth_out <= self.eth_reserve, "Not enough liquidity");

            self.eth_reserve -= eth_out;
            self.token_reserve += token_in;

            let caller_balance = self.token_balances.get(&caller).unwrap_or(&0);
            self.token_balances
                .insert(caller, caller_balance - token_in);

            self.env()
                .transfer(caller, eth_out)
                .unwrap_or_else(|_| panic!("Transfer failed"));

            self.env().emit_event(ETHSwapped {
                swapper: caller,
                token_in,
                eth_out,
            });

            eth_out
        }

        fn get_eth_price(&self, token_amount: Balance) -> Balance {
            (self.eth_reserve * token_amount) / self.token_reserve
        }

        fn get_token_price(&self, eth_amount: Balance) -> Balance {
            (self.token_reserve * eth_amount) / self.eth_reserve
        }

        #[ink(message)]
        pub fn withdraw_liquidity(&mut self, eth_amount: Balance, token_amount: Balance) -> bool {
            let caller = self.env().caller();
            assert!(
                eth_amount <= self.eth_reserve && token_amount <= self.token_reserve,
                "Not enough liquidity"
            );

            self.eth_reserve -= eth_amount;
            self.token_reserve -= token_amount;

            let caller_balance = self.token_balances.get(&caller).unwrap_or(&0);
            self.token_balances
                .insert(caller, caller_balance - token_amount);

            self.env()
                .transfer(caller, eth_amount)
                .unwrap_or_else(|_| panic!("Transfer failed"));

            self.env().emit_event(LiquidityAdded {
                provider: caller,
                eth_amount,
                token_amount,
            });

            true
        }
    }
}
