mod keyboard;
use crate::keyboard::kb_constants::keys::Keyboard;
fn main() {
    // Regex for getting the first 2 columns: ^(\S+)\t([\S+]+)
    // Override format = [key1]>[key2]
    // Matching group 1 = kinesis token (what gets written in the file)
    // Matching group 2 = form to use in GUI key label (can add custom symbols for future macros)
    println!("Hello, world!");
    let kb = Keyboard::new();
    println!(
        "{}",
        keyboard::kb_constants::tokens::MASTER_DICTIONARY["a"].token_id
    );
    println!(
        "{:?}",
        keyboard::kb_constants::tokens::MASTER_DICTIONARY["a"]
            .regular_value
            .unwrap()
    );
    println!(
        "{:?}",
        keyboard::kb_constants::tokens::MASTER_DICTIONARY["a"]
            .shifted_value
            .unwrap()
    );
    println!(
        "{}",
        keyboard::kb_constants::tokens::MASTER_DICTIONARY["a"].ui_representation
    );
}
