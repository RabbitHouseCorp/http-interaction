use std::collections::HashMap;


pub mod structures;

fn main() {
    let mut interactions: HashMap<&str, structures::interaction::InteractionData> = HashMap::new();
    let mut connection: HashMap<&str, structures::connection_state::ConnectionStateKraken> = HashMap::new();
    let mut bot_connection_state: HashMap<&str, structures::bot_interaction::BotInteraction> = HashMap::new();

}
