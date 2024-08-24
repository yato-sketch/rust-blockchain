#![cfg_attr(not(feature = "std"), no_std)]

use ink::prelude::string::String;
use ink::storage::Mapping;

#[ink::contract]
mod escrow {
    use super::*;

    #[ink(storage)]
    pub struct Escrow {
        pub buyer: AccountId,
        pub seller: AccountId,
        pub arbiter: AccountId,
        pub amount: Balance,
        pub is_funded: bool,
        pub is_released: bool,
    }

    #[ink(event)]
    pub struct Funded {
        #[ink(topic)]
        buyer: AccountId,
        #[ink(topic)]
        amount: Balance,
    }

    #[ink(event)]
    pub struct Released {
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        amount: Balance,
    }

    #[ink(event)]
    pub struct Refunded {
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        amount: Balance,
    }

    impl Escrow {
        #[ink(constructor)]
        pub fn new(seller: AccountId, arbiter: AccountId) -> Self {
            Self {
                buyer: Default::default(),
                seller,
                arbiter,
                amount: 0,
                is_funded: false,
                is_released: false,
            }
        }

        #[ink(message, payable)]
        pub fn fund(&mut self) -> Result<(), &'static str> {
            let caller = self.env().caller();
            let transferred_amount = self.env().transferred_value();

            if self.is_funded {
                return Err("Escrow is already funded.");
            }

            self.buyer = caller;
            self.amount = transferred_amount;
            self.is_funded = true;

            self.env().emit_event(Funded {
                buyer: caller,
                amount: transferred_amount,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn release(&mut self) -> Result<(), &'static str> {
            let caller = self.env().caller();

            if caller != self.arbiter {
                return Err("Only the arbiter can release the funds.");
            }

            if !self.is_funded {
                return Err("Escrow is not funded.");
            }

            if self.is_released {
                return Err("Funds have already been released.");
            }

            self.is_released = true;

            self.env().transfer(self.seller, self.amount)?;

            self.env().emit_event(Released {
                to: self.seller,
                amount: self.amount,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn refund(&mut self) -> Result<(), &'static str> {
            let caller = self.env().caller();

            if caller != self.arbiter {
                return Err("Only the arbiter can refund the funds.");
            }

            if !self.is_funded {
                return Err("Escrow is not funded.");
            }

            if self.is_released {
                return Err("Funds have already been released.");
            }

            self.is_funded = false;

            self.env().transfer(self.buyer, self.amount)?;

            self.env().emit_event(Refunded {
                to: self.buyer,
                amount: self.amount,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn get_status(&self) -> (AccountId, AccountId, AccountId, Balance, bool, bool) {
            (
                self.buyer,
                self.seller,
                self.arbiter,
                self.amount,
                self.is_funded,
                self.is_released,
            )
        }
    }
}
