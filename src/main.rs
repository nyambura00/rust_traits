// TASK: create a basic banking system using Traits in Rust. 
// The program will allow users to create accounts, deposit and withdraw money, and view their account balance.

fn main() {
    let mut alice_account = BankAccount {
        account_number: 12345,
        holder_name: "Alice".to_string(),
        balance: 0.22
    };

    let mut bob_account = BankAccount {
        account_number: 23312,
        holder_name: "Bob".to_string(),
        balance: 123400.00
    };

    println!("Alice's balance before deposit: {}.", alice_account.balance);
    println!("Withdraw 400 from Bob: {}", bob_account.balance);

    alice_account.deposit(200.12);

    bob_account.withdraw(400.0);

    println!("Alice's balance after deposit: {:#?}", alice_account.balance);
    println!("Bob's balance after withdrawal: {:#?}", bob_account.balance);
}

struct BankAccount {
    account_number: u16, // a fixed-width integer data type
    holder_name: String,
    balance: f64
}
trait Account {
   fn deposit(&mut self, amount: f64);
   fn withdraw(&mut self, amount: f64);
   fn balance(&mut self);
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance = self.balance + amount
    }

    fn withdraw(&mut self, amount: f64) {
        self.balance = self.balance - amount
    }

    fn balance(&mut self) {
        self.balance;
    }
}
