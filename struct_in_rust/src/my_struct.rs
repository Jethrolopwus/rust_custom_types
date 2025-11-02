 use std::cell::Cell;
 
 struct Person<'p> {
     first_name: Cell<&'p str>,
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


 fn new_person() -> Person <'static> {
    // mut keyword added to make the struct mutable
   let p1 = Person{ 
    // using Cell here from the standard library //
       first_name: Cell::from ("Jethro"),
       last_name: String::from("Brown"),
       birth_year: 1995,
       birth_month: 8,
       is_married: false,

      
   };
// mutating struct fields
    p1.first_name.set( "Micheal");
   return p1;

}

pub fn create_person() {
   let my_person = new_person();
    println!("First Name: {0}, Last Name: {1}, Birth Year: {2}, Birth Month: {3}, is Married: {4} ", my_person.first_name.get(), my_person.last_name, my_person.birth_year, my_person.birth_month, my_person.is_married);

    
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
    // to acces each field 
    let my_vehicle_tupple = new_vehicle_tupple();
   println!("{:?}", new_vehicle_tupple());
//    to access individual field index here //

println!("Manufacturer: {0}, Model: {1}, Year: {2}, Color: {3:?}, Is Electric: {4}", my_vehicle_tupple.0, my_vehicle_tupple.1, my_vehicle_tupple.2, my_vehicle_tupple.3, my_vehicle_tupple.4 );
}