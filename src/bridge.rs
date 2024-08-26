#![cfg_attr(not(feature = "std"), no_std)]

use ink::storage::Mapping;

#[ink::contract]
mod bridge {
    use super::*;

    #[ink(storage)]
    pub struct Bridge {
        pub locked_tokens: Mapping<AccountId, Balance>,
        pub admins: Mapping<AccountId, bool>,
    }

    #[ink(event)]
    pub struct Locked {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        amount: Balance,
        #[ink(topic)]
        target_chain: u32,
        #[ink(topic)]
        target_address: [u8; 32],
    }

    #[ink(event)]
    pub struct Unlocked {
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        amount: Balance,
    }

    impl Bridge {
        #[ink(constructor)]
        pub fn new(admin: AccountId) -> Self {
            let mut admins = Mapping::new();
            admins.insert(&admin, &true);
            Self {
                locked_tokens: Mapping::new(),
                admins,
            }
        }

        #[ink(message)]
        pub fn lock(&mut self, target_chain: u32, target_address: [u8; 32], amount: Balance) -> Result<(), &'static str> {
            let caller = self.env().caller();

            self.transfer_from(caller, self.env().account_id(), amount)?;

            let current_locked = self.locked_tokens.get(&caller).unwrap_or(0);
            self.locked_tokens.insert(&caller, &(current_locked + amount));

            self.env().emit_event(Locked {
                from: caller,
                amount,
                target_chain,
                target_address,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn unlock(&mut self, to: AccountId, amount: Balance) -> Result<(), &'static str> {
            let caller = self.env().caller();
            let is_admin = self.admins.get(&caller).unwrap_or(false);

            if !is_admin {
                return Err("Only admin can unlock tokens");
            }

            self.transfer_from(self.env().account_id(), to, amount)?;

            self.env().emit_event(Unlocked { to, amount });

            Ok(())
        }

        fn transfer_from(&self, from: AccountId, to: AccountId, amount: Balance) -> Result<(), &'static str> {
            Ok(())
        }

        #[ink(message)]
        pub fn add_admin(&mut self, new_admin: AccountId) -> Result<(), &'static str> {
            let caller = self.env().caller();
            let is_admin = self.admins.get(&caller).unwrap_or(false);

            if !is_admin {
                return Err("Only an existing admin can add a new admin");
            }

            self.admins.insert(&new_admin, &true);
            Ok(())
        }

        #[ink(message)]
        pub fn remove_admin(&mut self, admin: AccountId) -> Result<(), &'static str> {
            let caller = self.env().caller();
            let is_admin = self.admins.get(&caller).unwrap_or(false);

            if !is_admin {
                return Err("Only an existing admin can remove an admin");
            }

            self.admins.insert(&admin, &false);
            Ok(())
        }
    }
}
