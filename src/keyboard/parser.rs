use crate::keyboard::kb_constants::keys::{KbPosition, KeyAction};
use crate::keyboard::kb_constants::tokens::{KeyToken, MASTER_DICTIONARY};
use crate::keyboard::layout::{Keyboard, Layer};
// use regex::Regex;

enum LineType {
    LayerLine,
    RemapLine,
    MacroLine,
    SkipLine,
}

fn match_line_type(line_str: &str) -> LineType {
    match line_str.chars().next() {
        Some('<') => LineType::LayerLine,
        Some('[') => LineType::RemapLine,
        Some('{') => LineType::MacroLine,
        _ => LineType::SkipLine,
    }
}

pub fn parse_layout_file(raw_file: &str, current_kb: &mut Keyboard) {
    let mut active_layer = Layer::Base;
    for line in raw_file.lines() {
        let trimmed_line = line.trim();

        // let first_char = trimmed_line.chars().next();
        match trimmed_line.chars().next() {
            None | Some('*') => continue,

            Some('<') => {
                active_layer =
                    Layer::from_string(&trimmed_line[1..trimmed_line.len() - 1]).unwrap();
                continue;
            }

            Some('[') => {
                if let Some((trigger, action)) = trimmed_line.split_once('>') {
                    let trigger_pos_str: &str = trigger.trim_matches(|c| c == '[' || c == ']');

                    let trigger_pos = match KbPosition::get_position(trigger_pos_str) {
                        Some(found_pos) => found_pos,
                        None => {
                            println!("Did not find position.");
                            continue;
                        }
                    };

                    let remap_actions_vector: Vec<&str> = action
                        .split("][")
                        .map(|s| s.trim_matches(|c| c == '[' || c == ']'))
                        .filter(|s| !s.is_empty())
                        .collect();

                    // let (tap_key, hold_data) = match remap_actions_vector.as_slice() {

                    let (tap_key, hold_data) = match remap_actions_vector.as_slice() {
                        [tap] => (*tap, None),
                        [tap, duration, hold] => {
                            let delay_ms: u16 =
                                (*duration).trim_start_matches("t&h").parse().unwrap_or(200);
                            let hold_keytoken = match MASTER_DICTIONARY.get(*hold) {
                                // phf::Map returns a reference, so found_token is already &'static KeyToken
                                Some(found_token) => found_token,

                                None => {
                                    println!("Warning: Invalid hold token '{}'", hold);
                                    continue; // 2. Safely skip this broken line in the config file
                                }
                            };

                            (*tap, Some((delay_ms, hold_keytoken)))
                        }

                        _ => {
                            println!("Invalid configuration.");
                            continue;
                        }
                    };

                    let tap_key_token = match MASTER_DICTIONARY.get(tap_key) {
                        Some(tap_token) => {
                            let new_action: KeyAction = KeyAction::Remap(tap_token, hold_data);

                            current_kb.set_override(active_layer, trigger_pos, new_action);
                        }
                        None => {
                            println!(
                                "Warning: Invalid tap token '{}' on line: {}",
                                tap_key, trimmed_line
                            );
                        }
                    };
                }
            }

            Some('{') => {
                // TODO: implement assignment

                if let Some((trigger, action)) = trimmed_line.split_once('>') {
                    // let trigger_pos_str: &str = trigger.trim_matches(|c| c == '{' || c == '}');
                    let macro_triggers_vector: Vec<&str> = trigger
                        .split("}{")
                        .map(|s| s.trim_matches(|c| c == '{' || c == '}'))
                        .filter(|s| !s.is_empty())
                        .collect();

                    let macro_actions_vector: Vec<&str> = action
                        .split("}{")
                        .map(|s| s.trim_matches(|c| c == '{' || c == '}'))
                        .filter(|s| !s.is_empty())
                        .collect();
                }
            }

            Some(unrecognized_char) => {
                println!(
                    "Warning: invalid syntax starting with '{}' on line {}",
                    unrecognized_char, trimmed_line
                );
            } // _ => {}
        }
    }
}
