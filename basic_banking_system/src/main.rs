fn main() {
    println!("Hello, world!");
    let mut account1 = BankAccount {
        account_number: "0285566375".to_string(),
        holder_name: String::from("Adetola"),
        balance: 3000.0
    };

    let mut account2 = BankAccount {
        
        account_number: "0564317135".to_string(),
        holder_name: String::from("Simeon"),
        balance: 10000.0
    };


    if let Err(e) = account1.deposit(5000.0) {
        println!("Failed to deposit into {}'s account: {}", account1.holder_name, e);
    }

    if let Err(e) = account2.withdraw(2500.0) {
        println!("Failed to withdraw from {}'s account: {}", account2.holder_name, e)
    }
    let acc1 = account1.balance();
    let acc2 = account2.balance();

    println!("{} owns the account with the account number {}, which has a balance of {}", account1.holder_name, account1.account_number, acc1);
    println!("{} owns the account with the account number {}, which has a balance of {}", account2.holder_name, account2.account_number, acc2);
}

trait Account {
    fn deposit(&mut self, amount:f64) -> Result<(), String>;
    fn withdraw(&mut self, amount:f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: String,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount:f64) -> Result<(), String> {
        if amount <= 0.0 {
            Err("Cannot deposit a non-positive amount.".to_string())
        } else {
            self.balance += amount;
            Ok(())

        }
    }

    fn withdraw(&mut self, amount:f64) -> Result<(), String> {
        if amount <= 0.0 {
            Err("Cannot withdraw a non-positive amount.".to_string())
        }else if self.balance < amount {
            Err("Insufficient balance.".to_string())
        }else{
            self.balance -= amount;
            Ok(())
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}
