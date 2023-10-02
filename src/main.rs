// TASK 1: Create a basic banking system using Traits in Rust. 
// The program will allow users to create accounts, deposit and withdraw money, and view their account balance.

// TASK 2: Enforce error handling

fn main() {
    let mut alice_account = BankAccount {
        account_number: 12345,
        holder_name: "Alice".to_string(),
        balance: 20.22
    };

    let mut bob_account = BankAccount {
        account_number: 23312,
        holder_name: "Bob".to_string(),
        balance: 123400.00
    };

    println!("Alice's balance before deposit: {}.", alice_account.balance);
    println!("Bob's balance before withdrawal: {}", bob_account.balance);

    match alice_account.deposit(200.12) {
        Ok(_) => println!("Deposit Status: OK"),
        Err(err) => println!("Deposit Status: FAILED, Err: {:?}", err)
    };

    match bob_account.withdraw(400.0) {
        Ok(_) => println!("Withdrawal Status: OK"),
        Err(err) => println!("Withdrawal Status: FAILED, Err: {:?}", err)
    };

    println!("Alice's balance after deposit: {:#?}", alice_account.balance);
    println!("Bob's balance after withdrawal: {:#?}", bob_account.balance);
}

struct BankAccount {
    account_number: u16, // a fixed-width integer data type
    holder_name: String,
    balance: f64
}

#[derive(Debug)]
enum BankTransferErrors {
    InvalidAmount(String),
    ThresholdLimitReached(String),
    InvalidDataType,
    CustomError(String)
}

trait Account {
   fn deposit(&mut self, amount: f64) -> Result<(), String>;
   fn withdraw(&mut self, amount: f64) -> Result<(), String>;
   fn balance(&mut self);
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount < 100.00 {
            return Err(format!("{:#?}", BankTransferErrors::InvalidAmount("Minimum Deposit Amount = 100.00".to_string())));
        } else if !amount.is_finite() {
            return Err(format!("{:#?}", BankTransferErrors::InvalidDataType));
        } else {
            self.balance += amount;
        }

        Ok(())

    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if self.balance > 10.00 {
            if self.balance - amount < 10.00 {
                return Err(format!("{:#?}", BankTransferErrors::ThresholdLimitReached("Minimum amount needed in the account = 10.0".to_string())));
            } else if amount < 100.00 {
                return Err(format!("{:#?}", BankTransferErrors::InvalidAmount("Minimum Withdrawal Amount = 100.00".to_string())));
            }
        } else {
            return Err(format!("{:#?}", BankTransferErrors::CustomError("Account is running low. Cannot withdraw, deposit now".to_string())));
        }
        
        Ok(())

    }

    fn balance(&mut self) {
        self.balance;
    }
}
