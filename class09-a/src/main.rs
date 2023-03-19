use std::collections::HashMap;


fn main() {
    
    let mut user_hash_map:HashMap<String, String> = HashMap::new();
    
    // let name = String::from("name");
    let email = String::from("email");
    let password = String::from("password");
    user_hash_map.insert(email.clone(), String::from("user@email.com"));
    user_hash_map.insert(password.clone(), String::from("12345678"));
    println!("User Email {:?}",user_hash_map.get(& email));


    user_hash_map.remove(&email);
    println!("User Email {:?}",user_hash_map.get(& email));

    // user_hash_map.entry(email.clone()).or_insert(String::from("demo@email.com"));
    // println!("User Email {:?}",user_hash_map.get(& email));
    match user_hash_map.get(& email){
        Some(string) =>  println!("Email is {}",string),
        None=> panic!("Email Not Found")
    }
    println!("Program continue");
    // println!(name)
}
