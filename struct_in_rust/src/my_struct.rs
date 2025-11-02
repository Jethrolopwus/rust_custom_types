pub struct Person {

     fist_name: String,
     last_name: String,
     birth_year: u16,
     birth_month: u8,
}


 fn new_person() -> Person {
   let p1 = Person{ 
       fist_name: "Jethro".to_string(),
       last_name: String::from("Brown"),
       birth_year: 1995,
       birth_month: 8,
   };
   return p1;

}

pub fn create_person() {
   let my_person = new_person();
    println!("First Name: {0}, Last Name: {1}, Birth Year: {2}, Birth Month: {3}", my_person.fist_name, my_person.last_name, my_person.birth_year, my_person.birth_month);
}