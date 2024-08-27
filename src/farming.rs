#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod farming {
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };

    #[derive(Debug, Default, Clone, PartialEq, Eq, PackedLayout, SpreadLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct StakeInfo {
        amount: Balance,
        reward_debt: Balance,
        last_staked: u64,
    }

    #[ink(storage)]
    pub struct Farming {
        total_staked: Balance,
        reward_rate: Balance,
        stakers: StorageHashMap<AccountId, StakeInfo>,
    }

    impl Farming {
        #[ink(constructor)]
        pub fn new(reward_rate: Balance) -> Self {
            Self {
                total_staked: 0,
                reward_rate,
                stakers: StorageHashMap::new(),
            }
        }

        #[ink(message)]
        pub fn stake(&mut self, amount: Balance) {
            let caller = self.env().caller();
            let block_number = self.env().block_number();

            let mut stake_info = self.stakers.get(&caller).cloned().unwrap_or_default();

            let pending = self.pending_reward(&caller);

            stake_info.amount += amount;
            stake_info.reward_debt += pending;
            stake_info.last_staked = block_number;

            self.total_staked += amount;
            self.stakers.insert(caller, stake_info);
        }

        #[ink(message)]
        pub fn withdraw(&mut self, amount: Balance) {
            let caller = self.env().caller();
            let block_number = self.env().block_number();

            let mut stake_info = self.stakers.get(&caller).cloned().expect("No stake found");

            assert!(stake_info.amount >= amount, "Insufficient staked balance");

            let pending = self.pending_reward(&caller);

            stake_info.amount -= amount;
            stake_info.reward_debt += pending;
            stake_info.last_staked = block_number;

            self.total_staked -= amount;
            self.stakers.insert(caller, stake_info);
        }

        #[ink(message)]
        pub fn claim(&mut self) {
            let caller = self.env().caller();
            let pending = self.pending_reward(&caller);

            let mut stake_info = self.stakers.get(&caller).cloned().expect("No stake found");

            self.env()
                .transfer(caller, pending)
                .expect("Transfer failed");

            stake_info.reward_debt = 0;
            self.stakers.insert(caller, stake_info);
        }

        #[ink(message)]
        pub fn pending_reward(&self, staker: &AccountId) -> Balance {
            if let Some(stake_info) = self.stakers.get(staker) {
                let block_number = self.env().block_number();
                let staked_time = block_number - stake_info.last_staked;
                let pending = stake_info.amount * self.reward_rate * staked_time as Balance;

                return pending - stake_info.reward_debt;
            }
            0
        }

        #[ink(message)]
        pub fn get_staked_amount(&self, staker: AccountId) -> Balance {
            if let Some(stake_info) = self.stakers.get(&staker) {
                return stake_info.amount;
            }
            0
        }

        #[ink(message)]
        pub fn get_total_staked(&self) -> Balance {
            self.total_staked
        }
    }
}
