// parser.rs

use crate::keyboard::kb_constants::keys::{KbPosition, KeyAction};
use crate::keyboard::kb_constants::tokens::MASTER_DICTIONARY;
use crate::keyboard::layout_2::{Keyboard, Layer};
use regex::Regex;

// pub enum Layer {
// Base,
// Keypad,
// Fn1,
// Fn2,
// Fn3,
// }

pub fn parse_layout_file(raw_file: &str, mut new_kb: Keyboard) {
    let mut active_layer = Layer::Base;
    for line in raw_file.lines() {
        let trimmed_line = line.trim();

        // let first_char = trimmed_line.chars().next();
        match trimmed_line.chars().next() {
            None | Some('*') => continue,

            Some('<') => {
                active_layer = Layer::from_string_2(&trimmed_line[1..trimmed_line.len() - 1])
                    .unwrap_or(active_layer);
                continue;
            }

            Some('[') => {
                // if let Some((trigger, action)) = trimmed_line.split_once('>') {
                let re = Regex::new(r"\[([a-z0-9\.\+\-\=\*\/]+)\]").unwrap();
                if let Some(re_captures) = re.captures(trimmed_line) {
                    // let overwrite_kb_position = KbPosition::get_position(&re_captures[0]);
                    if let new_action = KeyAction::Remap(&MASTER_DICTIONARY[&re_captures[1]])
                        && let overwrite_kb_position = KbPosition::get_position(&re_captures[0])
                    {
                        new_kb.set_override(
                            active_layer,
                            overwrite_kb_position.unwrap(),
                            new_action,
                        );
                    }
                }

                // TODO: implement assignment

                // }
            }

            Some('{') => {
                // TODO: implement assignment
            }

            Some(unrecognized_char) => {
                println!(
                    "Warning: invalid syntax starting with '{}' on line {}",
                    unrecognized_char, trimmed_line
                );
            }
            _ => {}
        }
    }
}
