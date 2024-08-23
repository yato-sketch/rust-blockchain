use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
struct Event {
    event_type: String,
    from: String,
    to: String,
    amount: u64,
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Event {{ type: {}, from: {}, to: {}, amount: {} }}",
            self.event_type, self.from, self.to, self.amount
        )
    }
}

struct TokenContract {
    name: String,
    symbol: String,
    total_supply: u64,
    balances: HashMap<String, u64>,
    allowances: HashMap<String, HashMap<String, u64>>,
    events: Vec<Event>,
    owner: String,
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
            allowances: HashMap::new(),
            events: Vec::new(),
            owner: "owner".to_string(),
        }
    }

    fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), String> {
        self.check_balance(from, amount)?;
        self.update_balances(from, to, amount)?;
        self.emit_event("Transfer", from, to, amount);
        Ok(())
    }

    fn transfer_from(
        &mut self,
        from: &str,
        to: &str,
        spender: &str,
        amount: u64,
    ) -> Result<(), String> {
        self.check_balance(from, amount)?;
        self.check_allowance(from, spender, amount)?;
        self.update_balances(from, to, amount)?;
        self.update_allowance(from, spender, amount)?;
        self.emit_event("TransferFrom", from, to, amount);
        Ok(())
    }

    fn approve(&mut self, owner: &str, spender: &str, amount: u64) -> Result<(), String> {
        self.allowances
            .entry(owner.to_string())
            .or_insert_with(HashMap::new)
            .insert(spender.to_string(), amount);
        self.emit_event("Approval", owner, spender, amount);
        Ok(())
    }

    fn mint(&mut self, to: &str, amount: u64) -> Result<(), String> {
        if to != self.owner {
            return Err("Only owner can mint tokens".to_string());
        }
        *self.balances.entry(to.to_string()).or_insert(0) += amount;
        self.total_supply += amount;
        self.emit_event("Mint", "0x0", to, amount);
        Ok(())
    }

    fn burn(&mut self, from: &str, amount: u64) -> Result<(), String> {
        self.check_balance(from, amount)?;
        *self.balances.get_mut(from).unwrap() -= amount;
        self.total_supply -= amount;
        self.emit_event("Burn", from, "0x0", amount);
        Ok(())
    }

    fn balance_of(&self, account: &str) -> u64 {
        *self.balances.get(account).unwrap_or(&0)
    }

    fn allowance(&self, owner: &str, spender: &str) -> u64 {
        self.allowances
            .get(owner)
            .and_then(|spenders| spenders.get(spender))
            .cloned()
            .unwrap_or(0)
    }

    fn get_token_info(&self) -> (String, String, u64) {
        (self.name.clone(), self.symbol.clone(), self.total_supply)
    }

    fn check_balance(&self, account: &str, amount: u64) -> Result<(), String> {
        let balance = self.balance_of(account);
        if balance < amount {
            Err(format!("Insufficient balance for {}", account))
        } else {
            Ok(())
        }
    }

    fn check_allowance(&self, owner: &str, spender: &str, amount: u64) -> Result<(), String> {
        let allowed = self.allowance(owner, spender);
        if allowed < amount {
            Err(format!(
                "Insufficient allowance for {} from {}",
                spender, owner
            ))
        } else {
            Ok(())
        }
    }

    fn update_balances(&mut self, from: &str, to: &str, amount: u64) -> Result<(), String> {
        *self.balances.entry(from.to_string()).or_insert(0) -= amount;
        *self.balances.entry(to.to_string()).or_insert(0) += amount;
        Ok(())
    }

    fn update_allowance(&mut self, owner: &str, spender: &str, amount: u64) -> Result<(), String> {
        let allowance = self
            .allowances
            .get_mut(owner)
            .and_then(|spenders| spenders.get_mut(spender))
            .unwrap();
        *allowance -= amount;
        Ok(())
    }

    fn emit_event(&mut self, event_type: &str, from: &str, to: &str, amount: u64) {
        self.events.push(Event {
            event_type: event_type.to_string(),
            from: from.to_string(),
            to: to.to_string(),
            amount,
        });
    }

    fn get_events(&self) -> &[Event] {
        &self.events
    }

    fn print_event_summary(&self) {
        let mut transfer_volume = 0;
        let mut mint_volume = 0;
        let mut burn_volume = 0;

        for event in &self.events {
            match event.event_type.as_str() {
                "Transfer" | "TransferFrom" => transfer_volume += event.amount,
                "Mint" => mint_volume += event.amount,
                "Burn" => burn_volume += event.amount,
                _ => {}
            }
        }

        println!("Event Summary:");
        println!("Total Transfer Volume: {}", transfer_volume);
        println!("Total Minted: {}", mint_volume);
        println!("Total Burned: {}", burn_volume);
    }
}

impl fmt::Debug for TokenContract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TokenContract")
            .field("name", &self.name)
            .field("symbol", &self.symbol)
            .field("total_supply", &self.total_supply)
            .field("balances", &self.balances)
            .field("allowances", &self.allowances)
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

    // Perform some operations
    token.transfer("owner", "alice", 1000).unwrap();
    token.approve("alice", "bob", 500).unwrap();
    token.transfer_from("alice", "bob", "bob", 300).unwrap();
    token.mint("owner", 5000).unwrap();
    token.burn("owner", 2000).unwrap();

    println!("Final state: {:?}", token);

    // Print all events
    println!("Events:");
    for event in token.get_events() {
        println!("{:?}", event);
    }

    // Print event summary
    token.print_event_summary();
}
