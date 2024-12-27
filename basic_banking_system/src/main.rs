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

    account1.deposit(5000.0);
    account2.withdraw(2500.0);
    let acc1 = account1.balance();
    let acc2 = account2.balance();

    println!("{} owns the account with the account number {}, which has a balance of {}", account1.holder_name, account1.account_number, acc1);
    println!("{} owns the account with the account number {}, which has a balance of {}", account2.holder_name, account2.account_number, acc2);
}

trait Account {
    fn deposit(&mut self, amount:f64);
    fn withdraw(&mut self, amount:f64);
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: String,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount:f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount:f64) {
        self.balance -= amount;
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}
