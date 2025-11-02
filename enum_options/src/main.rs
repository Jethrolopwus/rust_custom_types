pub mod options;

fn main() {
  

 let result =  options::options_types();
 println!("Option value: {:?}", result.unwrap());

 // String option type //
 let result_str =  options::options_types_string();
 println!(" The option types on strings are: {:?}", result_str.unwrap());

}
