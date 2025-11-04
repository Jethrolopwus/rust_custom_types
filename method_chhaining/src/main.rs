

pub mod chaining_method;
use chaining_method::BankAccount;


fn main() {

    // catsing between references
    let x: u32 = 10;
    let y = x as f32;
    println!("x: {}, y: {}", x, y);

    // casting mutable refernces to 

    let mut data = 45;  
    let mutable_ref = &mut data;
    let immutable_ref = mutable_ref as *const i32; // ( allowed: casting mutable reference to immutable reference)

    println!("{:?}, {:?}", mutable_ref, immutable_ref);

    //Assignment of references
    let mut str = String::from("Hello");
    let ref_str_1 = &mut str;
    let ref_str_2 = &mut *ref_str_1; // ( allowed: assigning mutable reference to another mutable reference)
    println!("ref_str_1: {ref_str_1}");


    // method chaining depends on how each methods receives and returns self

    //1.  method  that does not return anything (i.e., returns ())
    // methods returning nothing cannotbe chanined further to grow the chain
    let mut account = BankAccount::new(String::from("Jethro"), 1000);

    account.check_balance();

    //2. method that returns &mut Self 
    //&mut Self returning methods can be chained requiring &mut self or &self

    account.deposit(500)
           .withdraw(200).view_account();

    //3. method that returns &Self
    // &Self returning methods can be chained requiring &self
    account.view_account().check_balance();

    //4. method that returns  owned form of Self
    //self -> chained with methods accepting any of the three forms of self: self, &mut self, &self

    account.change_owner(String::from("new_owner")).change_owner(String::from("Another_owner")).deposit(300).view_account();


   
}
