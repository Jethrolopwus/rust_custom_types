pub mod result_enum;
use result_enum::Student; 

fn main() {
    let student_db = vec![
        Student {
            name: String::from("Alice"),
            grade: Some(85),
        },
        Student {
            name: String::from("Charlie"),
            grade: Some(95),
        },
        Student {
            name: String::from("Bob"),
            grade: None,
        },
    ];
    
    let student_name = String::from("Charlie");
    let check_result = result_enum::check_student_get_grade(&student_name, &student_db);
    
    match check_result {
        Ok(option_grade) => {
            match option_grade {  
                Some(grade) => println!("{}'s grade is {}", student_name, grade),
                None => println!("{} has no grade recorded", student_name),
            }
        }
        Err(error_message) => println!("Error: {}", error_message),
    }
}