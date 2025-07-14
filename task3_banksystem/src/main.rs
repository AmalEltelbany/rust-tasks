use std::fmt;

trait Account {
    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
    fn account_info(&self) -> String;
}

#[derive(Debug)]
struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl BankAccount {
    fn new(account_number: u32, holder_name: String, initial_balance: f64) -> Self {
        BankAccount {
            account_number,
            holder_name,
            balance: initial_balance,
        }
    }
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            Err("❌ Deposit amount must be greater than 0.".to_string())
        } else {
            self.balance += amount;
            Ok(())
        }
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            Err("❌ Withdrawal amount must be greater than 0.".to_string())
        } else if amount > self.balance {
            Err("❌ Insufficient funds.".to_string())
        } else {
            self.balance -= amount;
            Ok(())
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }

    fn account_info(&self) -> String {
        format!(
            "Account #{}: {} | Balance: ${:.2}",
            self.account_number, self.holder_name, self.balance
        )
    }
}

impl fmt::Display for BankAccount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "🏦 Account #{}: {} | Balance: ${:.2}",
            self.account_number, self.holder_name, self.balance
        )
    }
}

fn main() {
    println!("🏦 Welcome to Rust Bank System!");
    println!("================================");

    let mut user1 = BankAccount::new(1001, "Amal".to_string(), 1000.0);
    let mut user2 = BankAccount::new(1002, "Yasmeen".to_string(), 500.0);

    println!("\n📊 Initial Account Status:");
    println!("{}", user1);
    println!("{}", user2);

    println!("\n💰 Processing Transactions:");
    println!("----------------------------");

    // Deposit to user1
    print!("Depositing $500.00 to {}'s account... ", user1.holder_name);
    match user1.deposit(500.0) {
        Ok(_) => println!("✅ Deposit successful!"),
        Err(e) => println!("❌ Deposit failed: {}", e),
    }

    // Attempt withdrawal from user2
    print!(
        "Attempting to withdraw $1000.00 from {}'s account... ",
        user2.holder_name
    );
    match user2.withdraw(1000.0) {
        Ok(_) => println!("✅ Withdrawal successful!"),
        Err(e) => println!("❌ Withdrawal failed: {}", e),
    }

    // Valid withdrawal from user2
    print!(
        "Attempting to withdraw $200.00 from {}'s account... ",
        user2.holder_name
    );
    match user2.withdraw(200.0) {
        Ok(_) => println!("✅ Withdrawal successful!"),
        Err(e) => println!("❌ Withdrawal failed: {}", e),
    }

    println!("\n📊 Final Account Status:");
    println!("------------------------");
    println!("{}", user1);
    println!("{}", user2);

    println!("\n📋 Account Information:");
    println!("----------------------");
    println!("🔍 {}", user1.account_info());
    println!("🔍 {}", user2.account_info());

    // Transfer functionality
    println!("\n💸 Transfer Example:");
    println!("-------------------");
    transfer_funds(&mut user1, &mut user2, 300.0);

    println!("\n📊 After Transfer:");
    println!("------------------");
    println!("{}", user1);
    println!("{}", user2);
}

fn transfer_funds(from_account: &mut BankAccount, to_account: &mut BankAccount, amount: f64) {
    println!(
        "Transferring ${:.2} from Account #{} to Account #{}...",
        amount, from_account.account_number, to_account.account_number
    );

    match from_account.withdraw(amount) {
        Ok(_) => {
            match to_account.deposit(amount) {
                Ok(_) => println!("✅ Transfer successful!"),
                Err(e) => {
                    // Rollback the withdrawal if deposit fails
                    from_account.deposit(amount).unwrap();
                    println!("❌ Transfer failed: {}", e);
                }
            }
        }
        Err(e) => println!("❌ Transfer failed: {}", e),
    }
}
