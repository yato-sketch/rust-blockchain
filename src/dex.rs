#![cfg_attr(not(feature = "std"), no_std)]

use ink::prelude::vec::Vec;
use ink::storage::Mapping;
use ink::env::AccountId;

#[ink::contract]
mod simple_dex {
    use super::*;

    #[ink(storage)]
    pub struct SimpleDex {
        // Simulated token balances within the DEX
        token_a_balance: Balance,
        token_b_balance: Balance,
        // Total liquidity provided
        total_liquidity: Balance,
        // Mapping of user address to their liquidity shares
        liquidity_providers: Mapping<AccountId, Balance>,
    }

    impl SimpleDex {
        #[ink(constructor)]
        pub fn new(initial_a: Balance, initial_b: Balance) -> Self {
            let caller = Self::env().caller();
            let total_liquidity = Self::calculate_liquidity(initial_a, initial_b);
            let mut liquidity_providers = Mapping::new();
            liquidity_providers.insert(&caller, &total_liquidity);
            Self {
                token_a_balance: initial_a,
                token_b_balance: initial_b,
                total_liquidity,
                liquidity_providers,
            }
        }

        /// Adds liquidity to the pool and returns the amount of liquidity tokens minted
        #[ink(message)]
        pub fn add_liquidity(&mut self, amount_a: Balance, amount_b: Balance) -> Balance {
            let caller = self.env().caller();
            let liquidity_minted = Self::calculate_liquidity(amount_a, amount_b);
            self.token_a_balance += amount_a;
            self.token_b_balance += amount_b;
            self.total_liquidity += liquidity_minted;
            let user_liquidity = self.liquidity_providers.get(&caller).unwrap_or(0);
            self.liquidity_providers.insert(&caller, &(user_liquidity + liquidity_minted));
            liquidity_minted
        }

        /// Removes liquidity from the pool and returns the amounts of tokens withdrawn
        #[ink(message)]
        pub fn remove_liquidity(&mut self, liquidity: Balance) -> (Balance, Balance) {
            let caller = self.env().caller();
            let user_liquidity = self.liquidity_providers.get(&caller).unwrap_or(0);
            assert!(user_liquidity >= liquidity, "Insufficient liquidity");
            let amount_a = liquidity * self.token_a_balance / self.total_liquidity;
            let amount_b = liquidity * self.token_b_balance / self.total_liquidity;
            self.token_a_balance -= amount_a;
            self.token_b_balance -= amount_b;
            self.total_liquidity -= liquidity;
            self.liquidity_providers.insert(&caller, &(user_liquidity - liquidity));
            (amount_a, amount_b)
        }

        /// Swaps `amount_a` of TokenA for TokenB
        #[ink(message)]
        pub fn swap_a_for_b(&mut self, amount_a: Balance) -> Balance {
            let amount_b = self.get_amount_out(amount_a, self.token_a_balance, self.token_b_balance);
            self.token_a_balance += amount_a;
            self.token_b_balance -= amount_b;
            amount_b
        }

        /// Swaps `amount_b` of TokenB for TokenA
        #[ink(message)]
        pub fn swap_b_for_a(&mut self, amount_b: Balance) -> Balance {
            let amount_a = self.get_amount_out(amount_b, self.token_b_balance, self.token_a_balance);
            self.token_b_balance += amount_b;
            self.token_a_balance -= amount_a;
            amount_a
        }

        /// Helper function to calculate output amount based on input amount and reserves
        fn get_amount_out(&self, amount_in: Balance, reserve_in: Balance, reserve_out: Balance) -> Balance {
            // Simple constant product formula: (x + Δx) * (y - Δy) = k
            // Δy = (Δx * y) / (x + Δx)
            (amount_in * reserve_out) / (reserve_in + amount_in)
        }

        /// Helper function to calculate liquidity minted based on amounts added
        fn calculate_liquidity(amount_a: Balance, amount_b: Balance) -> Balance {
            // In a real DEX, this would be more sophisticated
            // For simplicity, we'll take the minimum of the ratios
            if amount_a == 0 || amount_b == 0 {
                0
            } else {
                amount_a.min(amount_b)
            }
        }

        // Getter functions
        #[ink(message)]
        pub fn get_reserves(&self) -> (Balance, Balance) {
            (self.token_a_balance, self.token_b_balance)
        }

        #[ink(message)]
        pub fn get_total_liquidity(&self) -> Balance {
            self.total_liquidity
        }

        #[ink(message)]
        pub fn get_user_liquidity(&self, user: AccountId) -> Balance {
            self.liquidity_providers.get(&user).unwrap_or(0)
        }
    }
}
