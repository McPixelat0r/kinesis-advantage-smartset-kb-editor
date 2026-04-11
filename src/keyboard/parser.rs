// parser.rs

use crate::keyboard::kb_constants::keys::{KbPosition, KeyAction};
use crate::keyboard::layout::{Keyboard, Layer};

// pub enum Layer {
// Base,
// Keypad,
// Fn1,
// Fn2,
// Fn3,
// }
impl Layer {
    pub fn from_string(tag: &str) -> Option<Layer> {
        match tag {
            "base" => Some(Layer::Base),
            "keypad" => Some(Layer::Keypad), // The Kinesis tag for keypad is usually <kp>
            "function1" => Some(Layer::Fn1),
            "function2" => Some(Layer::Fn2),
            "function3" => Some(Layer::Fn3),
            _ => None, // If it reads a layer that doesn't exist, it returns nothing
        }
    }
}

pub fn parse_layout_file(raw_file: &str) {
    let mut active_layer = Layer::Base;
    for line in raw_file.lines() {
        let trimmed_line = line.trim();

        let first_char = trimmed_line.chars().next();
        match first_char {
            None | Some('*') => continue,

            Some('<') if trimmed_line.ends_with('>') => {
                active_layer = Layer::from_string(&trimmed_line[1..trimmed_line.len() - 1])
                    .unwrap_or(active_layer);
                // TODO: implement layer update
            }

            Some('[') | Some('{') => {
                // if let Some((trigger, action)) = trimmed_line.split_once('>') {
                // TODO: implement assignment

                // }
            }

            Some(unrecognized_char) => {
                println!(
                    "Warning: invalid syntax starting with '{}' on line {}",
                    unrecognized_char, trimmed_line
                );
            }
        }
    }
}
