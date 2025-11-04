
pub struct BankAccount {
    balance: i32,
    owner: String,
}

impl BankAccount {
    pub fn new(owner: String, initial_balance: i32) -> Self {
        println!("Creating account successfully for {}", owner);
        Self { 
            balance: initial_balance,
            owner,
        }   
    }
    
    pub fn change_owner(mut self, new_owner: String) -> Self {
        self.owner = new_owner;
        self
    }
    
    pub fn check_balance(&self) {
        println!("The balance for {} is ${}", self.owner, self.balance);
    }
    
    pub fn deposit(&mut self, amount: i32) -> &mut Self {
        self.balance += amount;
        println!("Deposited ${} to {}", amount, self.owner);
        self
    }
    
    pub fn withdraw(&mut self, amount: i32) -> &mut Self {
        if self.balance < amount {  
            println!("Insufficient funds for {}", self.owner);
        } else {
            self.balance -= amount;
            println!("Withdrew ${} from {}", amount, self.owner);
        }
        self
    }
    
    pub fn view_account(&self) -> &Self {  
        println!("Account Owner: {}, Balance: ${}", self.owner, self.balance);
        self  
    }
}