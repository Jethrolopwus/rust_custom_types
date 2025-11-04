use std::collections::HashSet;

pub fn hashset_type () {
    let mut planet_list = HashSet::from(["Mercury", "Venus", "Earth"]);

    let planet_list_more = HashSet::from(["Earth", "Mars", "Jupiter"]);

    // using the Difference method here to  subtract any value from the second hashSet with the other parameter  which implement the iterator method//

     let planet_diff =  planet_list.difference(&planet_list_more);

     for planet in planet_diff {
          println!("Thanks for adding {} ", planet);
     }


     //symetric difference object in HashSet which merged the hashsets togethter and subtract the duplicates values in both
     let planet_symdiff = planet_list.symmetric_difference(&planet_list_more);


     for planet in planet_symdiff {
          println!("Thanks for adding both {} ", planet);
     }

    // inserting/adding items into a HashSet using insert method
     planet_list.insert("Saturn");
     planet_list.insert("Uranus");
     planet_list.insert("Neptune");
     planet_list.insert("Pluto");

    // iterating over the list of the hashset planet list
    for planet in planet_list {
        println!("Adding all Plannets {} ", planet);
    }

}