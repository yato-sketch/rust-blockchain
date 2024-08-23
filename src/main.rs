use std::collections::HashMap;
use std::fmt;

struct TokenContract {
    name: String,
    symbol: String,
    total_supply: u64,
    balances: HashMap<String, u64>,
}

impl TokenContract {
    fn new(name: String, symbol: String, initial_supply: u64) -> Self {
        let mut balances = HashMap::new();
        balances.insert("owner".to_string(), initial_supply);

        TokenContract {
            name,
            symbol,
            total_supply: initial_supply,
            balances,
        }
    }

    fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), String> {
        let sender_balance = self.balances.get(from).unwrap_or(&0);
        if *sender_balance < amount {
            return Err(format!("Insufficient balance for {}", from));
        }

        *self.balances.entry(from.to_string()).or_insert(0) -= amount;
        *self.balances.entry(to.to_string()).or_insert(0) += amount;

        Ok(())
    }

    fn balance_of(&self, account: &str) -> u64 {
        *self.balances.get(account).unwrap_or(&0)
    }

    fn get_token_info(&self) -> (String, String, u64) {
        (self.name.clone(), self.symbol.clone(), self.total_supply)
    }
}

impl fmt::Debug for TokenContract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TokenContract")
            .field("name", &self.name)
            .field("symbol", &self.symbol)
            .field("total_supply", &self.total_supply)
            .field("balances", &self.balances)
            .finish()
    }
}

fn main() {
    let mut token = TokenContract::new("Yato".to_string(), "YTO".to_string(), 1_000_000);

    let (name, symbol, total_supply) = token.get_token_info();
    println!(
        "Token Info: {} ({}) - Total Supply: {}",
        name, symbol, total_supply
    );

    println!("Initial state: {:?}", token);

    match token.transfer("owner", "alice", 1000) {
        Ok(_) => println!("Transfer successful"),
        Err(e) => println!("Transfer failed: {}", e),
    }

    println!("Alice's balance: {}", token.balance_of("alice"));
    println!("Owner's balance: {}", token.balance_of("owner"));
}
