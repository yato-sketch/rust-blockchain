#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod betting {
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PackedLayout, SpreadLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum BetOption {
        Option1,
        Option2,
    }

    #[derive(Debug, Default, Clone, PackedLayout, SpreadLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Bet {
        amount: Balance,
        option: BetOption,
    }

    #[ink(storage)]
    pub struct Betting {
        owner: AccountId,
        bets: StorageHashMap<AccountId, Bet>,
        total_amount: Balance,
        option1_amount: Balance,
        option2_amount: Balance,
        betting_open: bool,
        winner: Option<BetOption>,
    }

    impl Betting {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                bets: StorageHashMap::new(),
                total_amount: 0,
                option1_amount: 0,
                option2_amount: 0,
                betting_open: true,
                winner: None,
            }
        }

        #[ink(message)]
        pub fn place_bet(&mut self, option: BetOption) {
            let caller = self.env().caller();
            let value = self.env().transferred_balance();

            assert!(self.betting_open, "Betting is closed");
            assert!(value > 0, "Bet amount must be greater than zero");

            let bet = Bet { amount: value, option };

            self.total_amount += value;
            match option {
                BetOption::Option1 => self.option1_amount += value,
                BetOption::Option2 => self.option2_amount += value,
            }

            self.bets.insert(caller, bet);
        }

        #[ink(message)]
        pub fn close_betting(&mut self) {
            assert_eq!(self.env().caller(), self.owner, "Only the owner can close betting");
            self.betting_open = false;
        }

        #[ink(message)]
        pub fn select_winner(&mut self, option: BetOption) {
            assert_eq!(self.env().caller(), self.owner, "Only the owner can select the winner");
            assert!(!self.betting_open, "Betting must be closed before selecting a winner");

            self.winner = Some(option);
            self.distribute_rewards();
        }

        #[ink(message)]
        pub fn withdraw(&mut self) {
            let caller = self.env().caller();
            assert!(self.winner.is_some(), "Winner not selected yet");

            if let Some(bet) = self.bets.get(&caller) {
                if bet.option == self.winner.unwrap() {
                    let payout_ratio = self.total_amount / match self.winner.unwrap() {
                        BetOption::Option1 => self.option1_amount,
                        BetOption::Option2 => self.option2_amount,
                    };

                    let payout = bet.amount * payout_ratio;
                    self.env().transfer(caller, payout).expect("Transfer failed");
                    self.bets.take(&caller); // Remove the bet after payout
                }
            }
        }

        fn distribute_rewards(&mut self) {
            if let Some(winning_option) = self.winner {
                let payout_ratio = match winning_option {
                    BetOption::Option1 => self.total_amount / self.option1_amount,
                    BetOption::Option2 => self.total_amount / self.option2_amount,
                };
        
                for (account, bet) in self.bets.iter() {
                    if bet.option == winning_option {
                        let payout = bet.amount * payout_ratio;
                        self.env().transfer(*account, payout).expect("Transfer failed");
                    }
                }
            }
        }
    }
}

