use ink::prelude::vec::Vec;
use ink::storage::Mapping;

#[ink::contract]
mod voting {
    #[ink(storage)]
    pub struct Voting {
        candidates: Vec<String>,
        votes: Mapping<String, u32>,
        has_voted: Mapping<AccountId, bool>,
    }

    impl Voting {
        #[ink(constructor)]
        pub fn new(candidates: Vec<String>) -> Self {
            let votes = Mapping::new();
            let has_voted = Mapping::new();
            Self {
                candidates,
                votes,
                has_voted,
            }
        }

        #[ink(message)]
        pub fn vote(&mut self, candidate: String) -> Result<(), String> {
            let caller = self.env().caller();
            if self.has_voted.get(&caller).unwrap_or(false) {
                return Err("You have already voted.".into());
            }

            if !self.candidates.contains(&candidate) {
                return Err("Candidate not found.".into());
            }

            let current_votes = self.votes.get(&candidate).unwrap_or(0);
            self.votes.insert(&candidate, &(current_votes + 1));
            self.has_voted.insert(&caller, &true);

            Ok(())
        }

        #[ink(message)]
        pub fn get_votes(&self, candidate: String) -> u32 {
            self.votes.get(&candidate).unwrap_or(0)
        }

        #[ink(message)]
        pub fn get_candidates(&self) -> Vec<String> {
            self.candidates.clone()
        }
    }
}
