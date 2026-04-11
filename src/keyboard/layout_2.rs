use crate::keyboard::kb_constants::keys::{KbKey, KbPosition, KeyAction};
use crate::keyboard::kb_constants::tokens;
use crate::keyboard::parser_attempt_2;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Layer {
    Base,
    Keypad,
    Fn1,
    Fn2,
    Fn3,
}

impl Layer {
    pub fn from_string_2(tag: &str) -> Option<Layer> {
        match tag {
            "base" => Some(Layer::Base),
            "keypad" => Some(Layer::Keypad),
            "function1" => Some(Layer::Fn1),
            "function2" => Some(Layer::Fn2),
            "function3" => Some(Layer::Fn3),
            _ => None, // If it reads a layer that doesn't exist, it returns nothing
        }
    }
}
pub struct Keyboard {
    pub default_keys: [KbKey; 76],
    layer_overrides: HashMap<Layer, HashMap<KbPosition, KeyAction>>,
}

#[allow(dead_code)]
impl Keyboard {
    pub fn new() -> Self {
        Self {
            default_keys: generate_default_keys(),

            layer_overrides: HashMap::new(),
        }
    }

    pub fn load_new_profile(&mut self, new_layout_text: &str) {
        self.layer_overrides.clear();
        parser_attempt_2::parse_layout_file(new_layout_text, self);
    }

    pub fn set_override(&mut self, layer: Layer, position: KbPosition, new_action: KeyAction) {
        self.layer_overrides
            .entry(layer)
            .or_default()
            .insert(position, new_action);
    }

    pub fn clear_override(&mut self, layer: &Layer, position: &KbPosition) {
        if let Some(layer_map) = self.layer_overrides.get_mut(layer) {
            layer_map.remove(position);
        }
    }
    pub fn get_active_action(&self, layer: &Layer, position: KbPosition) -> KeyAction {
        self.layer_overrides
            .get(layer)
            .and_then(|layer_map| layer_map.get(&position))
            .cloned()
            .unwrap_or_else(|| {
                let array_slot = position as usize;
                KeyAction::Remap(self.default_keys[array_slot].default_token)
            })
    }
}

fn generate_default_keys() -> [KbKey; 76] {
    [
        // Left hand - outside (pinky) in, top to bottom
        make_key(KbPosition::L_C0_R0, "eql"),
        make_key(KbPosition::L_C0_R1, "tab"),
        make_key(KbPosition::L_C0_R2, "esc"),
        make_key(KbPosition::L_C0_R3, "lshf"),
        make_key(KbPosition::L_C0_R4, "fn1s"),
        make_key(KbPosition::L_C1_R0, "1"),
        make_key(KbPosition::L_C1_R1, "q"),
        make_key(KbPosition::L_C1_R2, "a"),
        make_key(KbPosition::L_C1_R3, "z"),
        make_key(KbPosition::L_C1_R4, "grav"),
        make_key(KbPosition::L_C2_R0, "2"),
        make_key(KbPosition::L_C2_R1, "w"),
        make_key(KbPosition::L_C2_R2, "s"),
        make_key(KbPosition::L_C2_R3, "x"),
        make_key(KbPosition::L_C2_R4, "caps"),
        make_key(KbPosition::L_C3_R0, "3"),
        make_key(KbPosition::L_C3_R1, "e"),
        make_key(KbPosition::L_C3_R2, "d"),
        make_key(KbPosition::L_C3_R3, "c"),
        make_key(KbPosition::L_C3_R4, "left"),
        make_key(KbPosition::L_C4_R0, "4"),
        make_key(KbPosition::L_C4_R1, "r"),
        make_key(KbPosition::L_C4_R2, "f"),
        make_key(KbPosition::L_C4_R3, "v"),
        make_key(KbPosition::L_C4_R4, "rght"),
        make_key(KbPosition::L_C5_R0, "5"),
        make_key(KbPosition::L_C5_R1, "t"),
        make_key(KbPosition::L_C5_R2, "g"),
        make_key(KbPosition::L_C5_R3, "b"),
        make_key(KbPosition::L_C6_R0, "keyt"),
        make_key(KbPosition::L_C6_R1, "hk1"),
        make_key(KbPosition::L_C6_R2, "hk2"),
        // Left Thumb Cluster
        // The two large vertical keys (2U)
        make_key(KbPosition::L_Thumb_2U_Outer, "khBB"),
        make_key(KbPosition::L_Thumb_2U_Inner, "bspc"),
        // The four regular keys (1U) forming the '7' around the inner large key
        make_key(KbPosition::L_Thumb_1U_TopOuter, "lctr"),
        make_key(KbPosition::L_Thumb_1U_TopInner, "lalt"),
        make_key(KbPosition::L_Thumb_1U_MidInner, "home"),
        make_key(KbPosition::L_Thumb_1U_BottomInner, "end"),
        // Right hand
        make_key(KbPosition::R_C0_R0, "hyph"),
        make_key(KbPosition::R_C0_R1, "bsls"),
        make_key(KbPosition::R_C0_R2, "apos"),
        make_key(KbPosition::R_C0_R3, "rshf"),
        make_key(KbPosition::R_C0_R4, "fn1s"),
        make_key(KbPosition::R_C1_R0, "0"),
        make_key(KbPosition::R_C1_R1, "p"),
        make_key(KbPosition::R_C1_R2, "scol"),
        make_key(KbPosition::R_C1_R3, "fsls"),
        make_key(KbPosition::R_C1_R4, "cbrk"),
        make_key(KbPosition::R_C2_R0, "9"),
        make_key(KbPosition::R_C2_R1, "o"),
        make_key(KbPosition::R_C2_R2, "l"),
        make_key(KbPosition::R_C2_R3, "perd"),
        make_key(KbPosition::R_C2_R4, "obrk"),
        make_key(KbPosition::R_C3_R0, "8"),
        make_key(KbPosition::R_C3_R1, "i"),
        make_key(KbPosition::R_C3_R2, "k"),
        make_key(KbPosition::R_C3_R3, "comm"),
        make_key(KbPosition::R_C3_R4, "down"),
        make_key(KbPosition::R_C4_R0, "7"),
        make_key(KbPosition::R_C4_R1, "u"),
        make_key(KbPosition::R_C4_R2, "j"),
        make_key(KbPosition::R_C4_R3, "m"),
        make_key(KbPosition::R_C4_R4, "up"),
        make_key(KbPosition::R_C5_R0, "6"),
        make_key(KbPosition::R_C5_R1, "y"),
        make_key(KbPosition::R_C5_R2, "h"),
        make_key(KbPosition::R_C5_R3, "n"),
        make_key(KbPosition::R_C6_R0, "ss"),
        make_key(KbPosition::R_C6_R1, "hk3"),
        make_key(KbPosition::R_C6_R2, "hk4"),
        // Right Thumb Cluster
        // The two large vertical keys (2U)
        make_key(KbPosition::R_Thumb_2U_Outer, "spc"),
        make_key(KbPosition::R_Thumb_2U_Inner, "ent"),
        // The four regular keys (1U) forming the 'backwards 7' around the inner large key
        make_key(KbPosition::R_Thumb_1U_TopOuter, "rctr"),
        make_key(KbPosition::R_Thumb_1U_TopInner, "rwin"),
        make_key(KbPosition::R_Thumb_1U_MidInner, "pgup"),
        make_key(KbPosition::R_Thumb_1U_BottomInner, "pgdn"),
    ]
}

fn make_key(position: KbPosition, default_token_id: &'static str) -> KbKey {
    let default_token = tokens::MASTER_DICTIONARY
        .get(default_token_id)
        .unwrap_or_else(|| {
            panic!(
                "CRITICAL DEV ERROR: Token '{}' does not exist in MASTER_DICTIONARY",
                default_token_id
            )
        });
    KbKey {
        position,
        default_token,
    }
}
