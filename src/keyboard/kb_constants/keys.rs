use crate::keyboard::kb_constants::tokens;

use super::tokens::KeyToken;

#[allow(non_camel_case_types)] // This silences the warning
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KbPosition {
    // Left hand - outside (pinky) in, top to bottom
    L_C0_R0,
    L_C0_R1,
    L_C0_R2,
    L_C0_R3,
    L_C0_R4,
    L_C1_R0,
    L_C1_R1,
    L_C1_R2,
    L_C1_R3,
    L_C1_R4,
    L_C2_R0,
    L_C2_R1,
    L_C2_R2,
    L_C2_R3,
    L_C2_R4,
    L_C3_R0,
    L_C3_R1,
    L_C3_R2,
    L_C3_R3,
    L_C3_R4,
    L_C4_R0,
    L_C4_R1,
    L_C4_R2,
    L_C4_R3,
    L_C4_R4,
    L_C5_R0,
    L_C5_R1,
    L_C5_R2,
    L_C5_R3,
    L_C6_R0,
    L_C6_R1,
    L_C6_R2,
    // Left Thumb Cluster
    // The two large vertical keys (2U)
    L_Thumb_2U_Outer,
    L_Thumb_2U_Inner,
    // The four regular keys (1U) forming the '7' around the inner large key
    L_Thumb_1U_TopOuter,
    L_Thumb_1U_TopInner,
    L_Thumb_1U_MidInner,
    L_Thumb_1U_BottomInner,

    // Right hand
    R_C0_R0,
    R_C0_R1,
    R_C0_R2,
    R_C0_R3,
    R_C0_R4,
    R_C1_R0,
    R_C1_R1,
    R_C1_R2,
    R_C1_R3,
    R_C1_R4,
    R_C2_R0,
    R_C2_R1,
    R_C2_R2,
    R_C2_R3,
    R_C2_R4,
    R_C3_R0,
    R_C3_R1,
    R_C3_R2,
    R_C3_R3,
    R_C3_R4,
    R_C4_R0,
    R_C4_R1,
    R_C4_R2,
    R_C4_R3,
    R_C4_R4,
    R_C5_R0,
    R_C5_R1,
    R_C5_R2,
    R_C5_R3,
    R_C6_R0,
    R_C6_R1,
    R_C6_R2,
    // Right Thumb Cluster
    // The two large vertical keys (2U)
    R_Thumb_2U_Outer,
    R_Thumb_2U_Inner,
    // The four regular keys (1U) forming the 'backwards 7' around the inner large key
    R_Thumb_1U_TopOuter,
    R_Thumb_1U_TopInner,
    R_Thumb_1U_MidInner,
    R_Thumb_1U_BottomInner,
}

#[derive(Debug, Clone)]
pub enum KeyAction {
    /// basic remap
    Remap(&'static KeyToken),

    /// macro: {trigger}>{action1}{action2}...
    /// using vec to hold ordered list due to variable macro length
    Macro(Vec<&'static KeyToken>),

    /// tap and hold: [position]>[tap][t&hXXX][hold]
    TapAndHold {
        tap: &'static KeyToken,
        delay_ms: u16,
        hold: &'static KeyToken,
    },
}

#[derive(Debug, Clone)]
pub struct KbKey {
    pub position: KbPosition,
    pub default_token: &'static KeyToken,
    pub overwrite_token: Option<KeyAction>,
}

pub struct Keyboard {
    pub kb_keys: [KbKey; 76],
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            kb_keys: [
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
            ],
        }
    }

    pub fn set_overwrite(&mut self, position: KbPosition, new_action: KeyAction) {
        let array_slot = position as usize;

        self.kb_keys[array_slot].overwrite_token = Some(new_action);
    }

    pub fn clear_overwrite(&mut self, position: KbPosition) {
        let array_slot = position as usize;

        self.kb_keys[array_slot].overwrite_token = None;
    }
}

fn make_key(position: KbPosition, default_token_id: &'static str) -> KbKey {
    let default_token = tokens::MASTER_DICTIONARY
        .get(default_token_id)
        .expect(&format!(
            "CRITICAL DEV ERROR: Token '{}' does not exist in MASTER_DICTIONARY",
            default_token_id
        ));
    KbKey {
        position,
        default_token,
        overwrite_token: None,
    }
}
