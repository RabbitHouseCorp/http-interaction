use std::collections::HashMap;


fn main() {
    let mut interactions: HashMap<&str, &str> = HashMap::new();
    let mut connection: HashMap<&str, &str> = HashMap::new();
    let mut bot_connection_state: HashMap<&str, &str> = HashMap::new();
    interactions.insert("test", "a");
    println!("{:?}", interactions);
}
