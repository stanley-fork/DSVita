use ini::{Properties, SectionSetter};
use std::ffi::CString;

pub const NUM_KEYS: usize = 12;

/// DS button names, indexed by `crate::core::input::Keycode` value. Used as the
/// ini keys and the editor row labels.
pub const DS_KEY_NAMES: [&str; NUM_KEYS] = ["A", "B", "X", "Y", "Right", "Left", "Up", "Down", "R", "L", "Select", "Start"];

/// A named custom controls profile: for each DS key, the host (Vita) button bit
/// that triggers it. Vita-specific in meaning (the values are `SCE_CTRL_*` bits)
/// but stored as plain `u32`, so this module stays platform-agnostic.
#[derive(Clone)]
pub struct KeyBinding {
    pub name: String,
    pub buttons: [u32; NUM_KEYS],
}

impl KeyBinding {
    pub fn new(name: String, buttons: [u32; NUM_KEYS]) -> Self {
        KeyBinding { name, buttons }
    }

    pub fn from_ini(name: &str, props: &Properties) -> Self {
        let mut buttons = [0u32; NUM_KEYS];
        for (i, key_name) in DS_KEY_NAMES.iter().enumerate() {
            buttons[i] = props.get(*key_name).and_then(|v| v.parse::<u32>().ok()).unwrap_or(0);
        }
        KeyBinding { name: name.to_string(), buttons }
    }

    pub fn to_ini(&self, section_setter: &mut SectionSetter) {
        for (i, key_name) in DS_KEY_NAMES.iter().enumerate() {
            section_setter.set(*key_name, self.buttons[i].to_string());
        }
    }

    pub fn name_c_str(&self) -> CString {
        CString::new(self.name.clone()).unwrap_or_default()
    }
}

impl Default for KeyBinding {
    fn default() -> Self {
        KeyBinding {
            name: String::new(),
            buttons: [0; NUM_KEYS],
        }
    }
}
