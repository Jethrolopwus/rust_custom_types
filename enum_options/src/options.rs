


pub fn options_types () -> Option<u8> {
    let  mut opt1: Option<u8> = None;
    opt1 = Some(10);
    return opt1;
}

pub fn options_types_string () -> Option<String> {
    let  mut opt1: Option<String> = None;
    opt1 = Some("This is the string options type".to_string());
    return opt1;
}