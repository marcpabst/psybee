use winit::event::VirtualKeyCode;

use crate::visual::Window;

#[derive(Debug, Hash, Ord, PartialOrd, Clone, Copy)]
#[repr(u32)]
pub enum Key {
    /// The '1' key over the letters.
    Key1,
    /// The '2' key over the letters.
    Key2,
    /// The '3' key over the letters.
    Key3,
    /// The '4' key over the letters.
    Key4,
    /// The '5' key over the letters.
    Key5,
    /// The '6' key over the letters.
    Key6,
    /// The '7' key over the letters.
    Key7,
    /// The '8' key over the letters.
    Key8,
    /// The '9' key over the letters.
    Key9,
    /// The '0' key over the 'O' and 'P' keys.
    Key0,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    /// The Escape key, next to F1.
    Escape,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    /// Print Screen/SysRq.
    Snapshot,
    /// Scroll Lock.
    Scroll,
    /// Pause/Break key, next to Scroll lock.
    Pause,

    /// `Insert`, next to Backspace.
    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,

    Left,
    Up,
    Right,
    Down,

    /// The Backspace key, right over Enter.
    // TODO: rename
    Back,
    /// The Enter key.
    Return,
    /// The space bar.
    Space,

    /// The "Compose" key on Linux.
    Compose,

    Caret,

    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadDivide,
    NumpadDecimal,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    NumpadMultiply,
    NumpadSubtract,

    AbntC1,
    AbntC2,
    Apostrophe,
    Apps,
    Asterisk,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    Equals,
    Grave,
    Kana,
    Kanji,
    LAlt,
    LBracket,
    LControl,
    LShift,
    LWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    Mute,
    MyComputer,
    // also called "Next"
    NavigateForward,
    // also called "Prior"
    NavigateBackward,
    NextTrack,
    NoConvert,
    OEM102,
    Period,
    PlayPause,
    Plus,
    Power,
    PrevTrack,
    RAlt,
    RBracket,
    RControl,
    RShift,
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
    Copy,
    Paste,
    Cut,
    Any, // special key that matches any key
}

impl From<VirtualKeyCode> for Key {
    fn from(key: VirtualKeyCode) -> Self {
        match key {
            VirtualKeyCode::Key1 => Key::Key1,
            VirtualKeyCode::Key2 => Key::Key2,
            VirtualKeyCode::Key3 => Key::Key3,
            VirtualKeyCode::Key4 => Key::Key4,
            VirtualKeyCode::Key5 => Key::Key5,
            VirtualKeyCode::Key6 => Key::Key6,
            VirtualKeyCode::Key7 => Key::Key7,
            VirtualKeyCode::Key8 => Key::Key8,
            VirtualKeyCode::Key9 => Key::Key9,
            VirtualKeyCode::Key0 => Key::Key0,
            VirtualKeyCode::A => Key::A,
            VirtualKeyCode::B => Key::B,
            VirtualKeyCode::C => Key::C,
            VirtualKeyCode::D => Key::D,
            VirtualKeyCode::E => Key::E,
            VirtualKeyCode::F => Key::F,
            VirtualKeyCode::G => Key::G,
            VirtualKeyCode::H => Key::H,
            VirtualKeyCode::I => Key::I,
            VirtualKeyCode::J => Key::J,
            VirtualKeyCode::K => Key::K,
            VirtualKeyCode::L => Key::L,
            VirtualKeyCode::M => Key::M,
            VirtualKeyCode::N => Key::N,
            VirtualKeyCode::O => Key::O,
            VirtualKeyCode::P => Key::P,
            VirtualKeyCode::Q => Key::Q,
            VirtualKeyCode::R => Key::R,
            VirtualKeyCode::S => Key::S,
            VirtualKeyCode::T => Key::T,
            VirtualKeyCode::U => Key::U,
            VirtualKeyCode::V => Key::V,
            VirtualKeyCode::W => Key::W,
            VirtualKeyCode::X => Key::X,
            VirtualKeyCode::Y => Key::Y,
            VirtualKeyCode::Z => Key::Z,
            VirtualKeyCode::Escape => Key::Escape,
            VirtualKeyCode::F1 => Key::F1,
            VirtualKeyCode::F2 => Key::F2,
            VirtualKeyCode::F3 => Key::F3,
            VirtualKeyCode::F4 => Key::F4,
            VirtualKeyCode::F5 => Key::F5,
            VirtualKeyCode::F6 => Key::F6,
            VirtualKeyCode::F7 => Key::F7,
            VirtualKeyCode::F8 => Key::F8,
            VirtualKeyCode::F9 => Key::F9,
            VirtualKeyCode::F10 => Key::F10,
            VirtualKeyCode::F11 => Key::F11,
            VirtualKeyCode::F12 => Key::F12,
            VirtualKeyCode::F13 => Key::F13,
            VirtualKeyCode::F14 => Key::F14,
            VirtualKeyCode::F15 => Key::F15,
            VirtualKeyCode::F16 => Key::F16,
            VirtualKeyCode::F17 => Key::F17,
            VirtualKeyCode::F18 => Key::F18,
            VirtualKeyCode::F19 => Key::F19,
            VirtualKeyCode::F20 => Key::F20,
            VirtualKeyCode::F21 => Key::F21,
            VirtualKeyCode::F22 => Key::F22,
            VirtualKeyCode::F23 => Key::F23,
            VirtualKeyCode::F24 => Key::F24,
            VirtualKeyCode::Snapshot => Key::Snapshot,
            VirtualKeyCode::Scroll => Key::Scroll,
            VirtualKeyCode::Pause => Key::Pause,
            VirtualKeyCode::Insert => Key::Insert,
            VirtualKeyCode::Home => Key::Home,
            VirtualKeyCode::Delete => Key::Delete,
            VirtualKeyCode::End => Key::End,
            VirtualKeyCode::PageDown => Key::PageDown,
            VirtualKeyCode::PageUp => Key::PageUp,
            VirtualKeyCode::Left => Key::Left,
            VirtualKeyCode::Up => Key::Up,
            VirtualKeyCode::Right => Key::Right,
            VirtualKeyCode::Down => Key::Down,
            VirtualKeyCode::Back => Key::Back,
            VirtualKeyCode::Return => Key::Return,
            VirtualKeyCode::Space => Key::Space,
            VirtualKeyCode::Compose => Key::Compose,
            VirtualKeyCode::Caret => Key::Caret,
            VirtualKeyCode::Numlock => Key::Numlock,
            VirtualKeyCode::Numpad0 => Key::Numpad0,
            VirtualKeyCode::Numpad1 => Key::Numpad1,
            VirtualKeyCode::Numpad2 => Key::Numpad2,
            VirtualKeyCode::Numpad3 => Key::Numpad3,
            VirtualKeyCode::Numpad4 => Key::Numpad4,
            VirtualKeyCode::Numpad5 => Key::Numpad5,
            VirtualKeyCode::Numpad6 => Key::Numpad6,
            VirtualKeyCode::Numpad7 => Key::Numpad7,
            VirtualKeyCode::Numpad8 => Key::Numpad8,
            VirtualKeyCode::Numpad9 => Key::Numpad9,
            VirtualKeyCode::NumpadAdd => Key::NumpadAdd,
            VirtualKeyCode::NumpadDivide => Key::NumpadDivide,
            VirtualKeyCode::NumpadDecimal => Key::NumpadDecimal,
            VirtualKeyCode::NumpadComma => Key::NumpadComma,
            VirtualKeyCode::NumpadEnter => Key::NumpadEnter,
            VirtualKeyCode::NumpadEquals => Key::NumpadEquals,
            VirtualKeyCode::NumpadMultiply => Key::NumpadMultiply,
            VirtualKeyCode::NumpadSubtract => Key::NumpadSubtract,
            VirtualKeyCode::AbntC1 => Key::AbntC1,
            VirtualKeyCode::AbntC2 => Key::AbntC2,
            VirtualKeyCode::Apostrophe => Key::Apostrophe,
            VirtualKeyCode::Apps => Key::Apps,
            VirtualKeyCode::Asterisk => Key::Asterisk,
            VirtualKeyCode::At => Key::At,
            VirtualKeyCode::Ax => Key::Ax,
            VirtualKeyCode::Backslash => Key::Backslash,
            VirtualKeyCode::Calculator => Key::Calculator,
            VirtualKeyCode::Capital => Key::Capital,
            VirtualKeyCode::Colon => Key::Colon,
            VirtualKeyCode::Comma => Key::Comma,
            VirtualKeyCode::Convert => Key::Convert,
            VirtualKeyCode::Equals => Key::Equals,
            VirtualKeyCode::Grave => Key::Grave,
            VirtualKeyCode::Kana => Key::Kana,
            VirtualKeyCode::Kanji => Key::Kanji,
            VirtualKeyCode::LAlt => Key::LAlt,
            VirtualKeyCode::LBracket => Key::LBracket,
            VirtualKeyCode::LControl => Key::LControl,
            VirtualKeyCode::LShift => Key::LShift,
            VirtualKeyCode::LWin => Key::LWin,
            VirtualKeyCode::Mail => Key::Mail,
            VirtualKeyCode::MediaSelect => Key::MediaSelect,
            VirtualKeyCode::MediaStop => Key::MediaStop,
            VirtualKeyCode::Minus => Key::Minus,
            VirtualKeyCode::Mute => Key::Mute,
            VirtualKeyCode::MyComputer => Key::MyComputer,
            VirtualKeyCode::NavigateForward => Key::NavigateForward,
            VirtualKeyCode::NavigateBackward => Key::NavigateBackward,
            VirtualKeyCode::NextTrack => Key::NextTrack,
            VirtualKeyCode::NoConvert => Key::NoConvert,
            VirtualKeyCode::OEM102 => Key::OEM102,
            VirtualKeyCode::Period => Key::Period,
            VirtualKeyCode::PlayPause => Key::PlayPause,
            VirtualKeyCode::Plus => Key::Plus,
            VirtualKeyCode::Power => Key::Power,
            VirtualKeyCode::PrevTrack => Key::PrevTrack,
            VirtualKeyCode::RAlt => Key::RAlt,
            VirtualKeyCode::RBracket => Key::RBracket,
            VirtualKeyCode::RControl => Key::RControl,
            VirtualKeyCode::RShift => Key::RShift,
            VirtualKeyCode::RWin => Key::RWin,
            VirtualKeyCode::Semicolon => Key::Semicolon,
            VirtualKeyCode::Slash => Key::Slash,
            VirtualKeyCode::Sleep => Key::Sleep,
            VirtualKeyCode::Stop => Key::Stop,
            VirtualKeyCode::Sysrq => Key::Sysrq,
            VirtualKeyCode::Tab => Key::Tab,
            VirtualKeyCode::Underline => Key::Underline,
            VirtualKeyCode::Unlabeled => Key::Unlabeled,
            VirtualKeyCode::VolumeDown => Key::VolumeDown,
            VirtualKeyCode::VolumeUp => Key::VolumeUp,
            VirtualKeyCode::Wake => Key::Wake,
            VirtualKeyCode::WebBack => Key::WebBack,
            VirtualKeyCode::WebFavorites => Key::WebFavorites,
            VirtualKeyCode::WebForward => Key::WebForward,
            VirtualKeyCode::WebHome => Key::WebHome,
            VirtualKeyCode::WebRefresh => Key::WebRefresh,
            VirtualKeyCode::WebSearch => Key::WebSearch,
            VirtualKeyCode::WebStop => Key::WebStop,
            VirtualKeyCode::Yen => Key::Yen,
            VirtualKeyCode::Copy => Key::Copy,
            VirtualKeyCode::Paste => Key::Paste,
            VirtualKeyCode::Cut => Key::Cut,
        }
    }
}

// implement From for key and virtual key code
impl From<Key> for VirtualKeyCode {
    fn from(key: Key) -> Self {
        match key {
            Key::Key1 => VirtualKeyCode::Key1,
            Key::Key2 => VirtualKeyCode::Key2,
            Key::Key3 => VirtualKeyCode::Key3,
            Key::Key4 => VirtualKeyCode::Key4,
            Key::Key5 => VirtualKeyCode::Key5,
            Key::Key6 => VirtualKeyCode::Key6,
            Key::Key7 => VirtualKeyCode::Key7,
            Key::Key8 => VirtualKeyCode::Key8,
            Key::Key9 => VirtualKeyCode::Key9,
            Key::Key0 => VirtualKeyCode::Key0,
            Key::A => VirtualKeyCode::A,
            Key::B => VirtualKeyCode::B,
            Key::C => VirtualKeyCode::C,
            Key::D => VirtualKeyCode::D,
            Key::E => VirtualKeyCode::E,
            Key::F => VirtualKeyCode::F,
            Key::G => VirtualKeyCode::G,
            Key::H => VirtualKeyCode::H,
            Key::I => VirtualKeyCode::I,
            Key::J => VirtualKeyCode::J,
            Key::K => VirtualKeyCode::K,
            Key::L => VirtualKeyCode::L,
            Key::M => VirtualKeyCode::M,
            Key::N => VirtualKeyCode::N,
            Key::O => VirtualKeyCode::O,
            Key::P => VirtualKeyCode::P,
            Key::Q => VirtualKeyCode::Q,
            Key::R => VirtualKeyCode::R,
            Key::S => VirtualKeyCode::S,
            Key::T => VirtualKeyCode::T,
            Key::U => VirtualKeyCode::U,
            Key::V => VirtualKeyCode::V,
            Key::W => VirtualKeyCode::W,
            Key::X => VirtualKeyCode::X,
            Key::Y => VirtualKeyCode::Y,
            Key::Z => VirtualKeyCode::Z,
            Key::Escape => VirtualKeyCode::Escape,
            Key::F1 => VirtualKeyCode::F1,
            Key::F2 => VirtualKeyCode::F2,
            Key::F3 => VirtualKeyCode::F3,
            Key::F4 => VirtualKeyCode::F4,
            Key::F5 => VirtualKeyCode::F5,
            Key::F6 => VirtualKeyCode::F6,
            Key::F7 => VirtualKeyCode::F7,
            Key::F8 => VirtualKeyCode::F8,
            Key::F9 => VirtualKeyCode::F9,
            Key::F10 => VirtualKeyCode::F10,
            Key::F11 => VirtualKeyCode::F11,
            Key::F12 => VirtualKeyCode::F12,
            Key::F13 => VirtualKeyCode::F13,
            Key::F14 => VirtualKeyCode::F14,
            Key::F15 => VirtualKeyCode::F15,
            Key::F16 => VirtualKeyCode::F16,
            Key::F17 => VirtualKeyCode::F17,
            Key::F18 => VirtualKeyCode::F18,
            Key::F19 => VirtualKeyCode::F19,
            Key::F20 => VirtualKeyCode::F20,
            Key::F21 => VirtualKeyCode::F21,
            Key::F22 => VirtualKeyCode::F22,
            Key::F23 => VirtualKeyCode::F23,
            Key::F24 => VirtualKeyCode::F24,
            Key::Snapshot => VirtualKeyCode::Snapshot,
            Key::Scroll => VirtualKeyCode::Scroll,
            Key::Pause => VirtualKeyCode::Pause,
            Key::Insert => VirtualKeyCode::Insert,
            Key::Home => VirtualKeyCode::Home,
            Key::Delete => VirtualKeyCode::Delete,
            Key::End => VirtualKeyCode::End,
            Key::PageDown => VirtualKeyCode::PageDown,
            Key::PageUp => VirtualKeyCode::PageUp,
            Key::Left => VirtualKeyCode::Left,
            Key::Up => VirtualKeyCode::Up,
            Key::Right => VirtualKeyCode::Right,
            Key::Down => VirtualKeyCode::Down,
            Key::Back => VirtualKeyCode::Back,
            Key::Return => VirtualKeyCode::Return,
            Key::Space => VirtualKeyCode::Space,
            Key::Compose => VirtualKeyCode::Compose,
            Key::Caret => VirtualKeyCode::Caret,
            Key::Numlock => VirtualKeyCode::Numlock,
            Key::Numpad0 => VirtualKeyCode::Numpad0,
            Key::Numpad1 => VirtualKeyCode::Numpad1,
            Key::Numpad2 => VirtualKeyCode::Numpad2,
            Key::Numpad3 => VirtualKeyCode::Numpad3,
            Key::Numpad4 => VirtualKeyCode::Numpad4,
            Key::Numpad5 => VirtualKeyCode::Numpad5,
            Key::Numpad6 => VirtualKeyCode::Numpad6,
            Key::Numpad7 => VirtualKeyCode::Numpad7,
            Key::Numpad8 => VirtualKeyCode::Numpad8,
            Key::Numpad9 => VirtualKeyCode::Numpad9,
            Key::NumpadAdd => VirtualKeyCode::NumpadAdd,
            Key::NumpadDivide => VirtualKeyCode::NumpadDivide,
            Key::NumpadDecimal => VirtualKeyCode::NumpadDecimal,
            Key::NumpadComma => VirtualKeyCode::NumpadComma,
            Key::NumpadEnter => VirtualKeyCode::NumpadEnter,
            Key::NumpadEquals => VirtualKeyCode::NumpadEquals,
            Key::NumpadMultiply => VirtualKeyCode::NumpadMultiply,
            Key::NumpadSubtract => VirtualKeyCode::NumpadSubtract,
            Key::AbntC1 => VirtualKeyCode::AbntC1,
            Key::AbntC2 => VirtualKeyCode::AbntC2,
            Key::Apostrophe => VirtualKeyCode::Apostrophe,
            Key::Apps => VirtualKeyCode::Apps,
            Key::Asterisk => VirtualKeyCode::Asterisk,
            Key::At => VirtualKeyCode::At,
            Key::Ax => VirtualKeyCode::Ax,
            Key::Backslash => VirtualKeyCode::Backslash,
            Key::Calculator => VirtualKeyCode::Calculator,
            Key::Capital => VirtualKeyCode::Capital,
            Key::Colon => VirtualKeyCode::Colon,
            Key::Comma => VirtualKeyCode::Comma,
            Key::Convert => VirtualKeyCode::Convert,
            Key::Equals => VirtualKeyCode::Equals,
            Key::Grave => VirtualKeyCode::Grave,
            Key::Kana => VirtualKeyCode::Kana,
            Key::Kanji => VirtualKeyCode::Kanji,
            Key::LAlt => VirtualKeyCode::LAlt,
            Key::LBracket => VirtualKeyCode::LBracket,
            Key::LControl => VirtualKeyCode::LControl,
            Key::LShift => VirtualKeyCode::LShift,
            Key::LWin => VirtualKeyCode::LWin,
            Key::Mail => VirtualKeyCode::Mail,
            Key::MediaSelect => VirtualKeyCode::MediaSelect,
            Key::MediaStop => VirtualKeyCode::MediaStop,
            Key::Minus => VirtualKeyCode::Minus,
            Key::Mute => VirtualKeyCode::Mute,
            Key::MyComputer => VirtualKeyCode::MyComputer,
            Key::NavigateForward => VirtualKeyCode::NavigateForward,
            Key::NavigateBackward => VirtualKeyCode::NavigateBackward,
            Key::NextTrack => VirtualKeyCode::NextTrack,
            Key::NoConvert => VirtualKeyCode::NoConvert,
            Key::OEM102 => VirtualKeyCode::OEM102,
            Key::Period => VirtualKeyCode::Period,
            Key::PlayPause => VirtualKeyCode::PlayPause,
            Key::Plus => VirtualKeyCode::Plus,
            Key::Power => VirtualKeyCode::Power,
            Key::PrevTrack => VirtualKeyCode::PrevTrack,
            Key::RAlt => VirtualKeyCode::RAlt,
            Key::RBracket => VirtualKeyCode::RBracket,
            Key::RControl => VirtualKeyCode::RControl,
            Key::RShift => VirtualKeyCode::RShift,
            Key::RWin => VirtualKeyCode::RWin,
            Key::Semicolon => VirtualKeyCode::Semicolon,
            Key::Slash => VirtualKeyCode::Slash,
            Key::Sleep => VirtualKeyCode::Sleep,
            Key::Stop => VirtualKeyCode::Stop,
            Key::Sysrq => VirtualKeyCode::Sysrq,
            Key::Tab => VirtualKeyCode::Tab,
            Key::Underline => VirtualKeyCode::Underline,
            Key::Unlabeled => VirtualKeyCode::Unlabeled,
            Key::VolumeDown => VirtualKeyCode::VolumeDown,
            Key::VolumeUp => VirtualKeyCode::VolumeUp,
            Key::Wake => VirtualKeyCode::Wake,
            Key::WebBack => VirtualKeyCode::WebBack,
            Key::WebFavorites => VirtualKeyCode::WebFavorites,
            Key::WebForward => VirtualKeyCode::WebForward,
            Key::WebHome => VirtualKeyCode::WebHome,
            Key::WebRefresh => VirtualKeyCode::WebRefresh,
            Key::WebSearch => VirtualKeyCode::WebSearch,
            Key::WebStop => VirtualKeyCode::WebStop,
            Key::Yen => VirtualKeyCode::Yen,
            Key::Copy => VirtualKeyCode::Copy,
            Key::Paste => VirtualKeyCode::Paste,
            Key::Cut => VirtualKeyCode::Cut,
            Key::Any => {
                panic!("Key::Any cannot be converted to a VirtualKeyCode")
            }
        }
    }
}

// implement IntoIterator for Key (so that a single key can be iterated over)
impl IntoIterator for Key {
    type Item = Key;
    type IntoIter = std::iter::Once<Key>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

// implement partial equality for key (and handle the special case of Key::Any)
impl PartialEq<Key> for Key {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Key::Any => true,
            _ => {
                std::mem::discriminant(self)
                    == std::mem::discriminant(other)
            }
        }
    }
}

impl Eq for Key {}

#[derive(Debug, Hash, Ord, PartialOrd, Clone, Copy)]
pub enum KeyState {
    Pressed,
    Released,
    Any, // special key state that matches any key state
}

// convert from winit key event to key state
impl From<winit::event::ElementState> for KeyState {
    fn from(state: winit::event::ElementState) -> Self {
        match state {
            winit::event::ElementState::Pressed => KeyState::Pressed,
            winit::event::ElementState::Released => {
                KeyState::Released
            }
        }
    }
}

// convert from winit key event to key state
impl From<KeyState> for winit::event::ElementState {
    fn from(state: KeyState) -> Self {
        match state {
            KeyState::Pressed => winit::event::ElementState::Pressed,
            KeyState::Released => {
                winit::event::ElementState::Released
            }
            KeyState::Any => {
                panic!("KeyState::Any cannot be converted to a winit::event::ElementState")
            }
        }
    }
}

// implement PartialEq for KeyState (and handle the special case of KeyState::Any)
impl PartialEq<KeyState> for KeyState {
    fn eq(&self, other: &Self) -> bool {
        match self {
            KeyState::Any => true,
            _ => {
                std::mem::discriminant(self)
                    == std::mem::discriminant(other)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyEvent {
    pub key: Key,
    pub state: KeyState,
}

// allow KeyEvent to be compared with Key
impl PartialEq<Key> for KeyEvent {
    fn eq(&self, other: &Key) -> bool {
        self.key == *other
    }
}

// allow KeyEvent to be compared with Key
impl PartialEq<KeyEvent> for Key {
    fn eq(&self, other: &KeyEvent) -> bool {
        *self == other.key
    }
}

impl Eq for KeyState {}

pub struct KeyPressReceiver {
    receiver: async_broadcast::Receiver<winit::event::KeyboardInput>,
}

impl KeyPressReceiver {
    pub fn new(window: &Window) -> Self {
        Self {
            receiver: window.keyboard_receiver.activate_cloned(),
        }
    }
    pub fn get_keys(&mut self) -> Vec<KeyEvent> {
        let mut keys = Vec::new();
        while let Ok(key_event) = self.receiver.try_recv() {
            let key = Key::from(key_event.virtual_keycode.unwrap()); // TODO: this should never fail but we should handle it anyway
            let state = KeyState::from(key_event.state);
            keys.push(KeyEvent { key, state });
        }
        return keys;
    }
}
