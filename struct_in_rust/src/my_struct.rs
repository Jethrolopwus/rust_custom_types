 struct Person {

     fist_name: String,
     last_name: String,
     birth_year: u16,
     birth_month: u8,
      is_married: bool,
}


#[derive(Debug)]
#[allow(dead_code)]
enum VehicleColors {
    Red,
    Blue,
    Green,
    Black,
    White,
    Silver,
    Yellow,
    
}
#[derive(Debug)]
#[allow(dead_code)]
struct Vehicle {
    manufacturer: String,
    model: String,
    year: u16,
    color: VehicleColors,
    is_electric: bool,
}

// representing a struct fiels as a tupple struct
#[derive(Debug)]
#[allow(dead_code)]
struct VehicleTupple(String, String, u16, VehicleColors, bool);


 fn new_person() -> Person {
   let p1 = Person{ 
       fist_name: "Jethro".to_string(),
       last_name: String::from("Brown"),
       birth_year: 1995,
       birth_month: 8,
       is_married: false,
      
   };
   return p1;

}

pub fn create_person() {
   let my_person = new_person();
    println!("First Name: {0}, Last Name: {1}, Birth Year: {2}, Birth Month: {3}, is Married: {4} ", my_person.fist_name, my_person.last_name, my_person.birth_year, my_person.birth_month, my_person.is_married);
}



fn new_vehicle() -> Vehicle {
   let v1 = Vehicle{
       manufacturer: "Tesla".to_string(),
       model: "Model 3".to_string(),
       year: 2020,
       color: VehicleColors::Silver,
       is_electric: true,
   };
   return v1;
}

 pub fn create_new_vehicle() {
   let my_vehicle = new_vehicle();
   println!("{:?}", my_vehicle);
}


// vehicle tuple //
fn new_vehicle_tupple() -> VehicleTupple {
   return VehicleTupple("Benz".to_string(), "GLK 350".to_string (), 2020, VehicleColors::Blue, true);
}   

pub fn create_new_vehicletupple () {
   println!("{:?}", new_vehicle_tupple());
}