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

#[expect(dead_code)]
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

#[expect(dead_code)]
#[derive(Debug, Clone)]
pub struct KbKey {
    pub position: KbPosition,
    pub default_token: &'static KeyToken,
}
