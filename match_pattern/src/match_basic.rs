

pub fn match_int_types () {
    let myage: u16 = 35;

    match myage{
        // match arms //
        35 =>{
            println!("Your age is 35 ");
        }
        _ => {
            println!("Your age is NOT 35 ");

        }
    }

}