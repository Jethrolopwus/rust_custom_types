use std::collections::HashMap;

pub fn hashmap_basic() {

    let mut stock_list = HashMap::<String, f32>::new(); 

    println!(" The lenght is: {} ", stock_list.len());
    println!(" The empty stocklist is: {} ", stock_list.is_empty());


    // adding a value into the stocklist //
    stock_list.insert("NVDA".to_string(),  478.52);
    stock_list.insert("AAPL".to_string(),  232.92);
    stock_list.insert("AMSC".to_string(),  50.78);

    // updating a value of a particular stocklist key say AAPL by overriding//
    stock_list.insert("AAPL".to_string(),  235.50);

    // adding a key and value if a key is not present //
    stock_list.entry("META".to_string()).or_insert(346.34);


    println!("{:#?} ", stock_list);
    // removing a value of a stocklist //

    stock_list.remove(&("AAPL".to_string()));


    println!("{:#?} ", stock_list);


    // iterating over the stocklist using iterator method //

    for (ticker, current_value) in stock_list{
        println!("{} is trading at: {} ", ticker, current_value);
    }
}