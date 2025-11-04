

pub struct Student {
   pub name: String,
   pub grade: Option<u32>
}


pub  fn check_student_get_grade (student_name: &String, student_db: &Vec<Student>) -> Result<Option<u32>, String> {
   for student  in student_db {
        if student.name == *student_name {
            return Ok(student.grade);
        }
    }
   Err(String::from("Student not found"))

}

// pub fn get_grade (student_name: &String, student_db: &Vec<Student>) -> Option<u32> {
//     for student  in student_db {
//         if student.name == *student_name {
//             return student.grade;
//         }
//     }
//     None 
// }
