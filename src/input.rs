use device_query::{DeviceQuery, Keycode};

#[derive(Clone, PartialEq)]
pub enum KeyCode {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
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
    Escape,
    Space,
    LControl,
    RControl,
    LShift,
    RShift,
    LAlt,
    RAlt,
    Enter,
    Up,
    Down,
    Left,
    Right,
    Backspace,
    CapsLock,
    Tab,
    Home,
    End,
    PageUp,
    PageDown,
    Insert,
    Delete
}


pub struct NeonInput {
    pub pressed_keys: Vec<KeyCode>, // List of currently pressed keys
    previous_keys: Vec<KeyCode>, // List of previously pressed keys
    pub buffer: String, // Optional buffer that can be turned on or off
    buffer_state: bool, // Whether the buffer is enabled or not
    pub caps_lock: bool // Whether Caps Lock is enabled
}

impl NeonInput {
    pub fn new() -> Self {
        NeonInput {
            pressed_keys: Vec::new(),
            previous_keys: Vec::new(),
            buffer: String::new(),
            buffer_state: false,
            caps_lock: false
        }
    }

    pub fn update(&mut self) {
        // Update the state of the input
        self.previous_keys = self.pressed_keys.clone();
        self.pressed_keys.clear();

        // Get the current state of the keyboard
        let keys = device_query::DeviceState::new().get_keys();
        for key in keys {
            match key {
                Keycode::Key0 => self.pressed_keys.push(KeyCode::Key0),
                Keycode::Key1 => self.pressed_keys.push(KeyCode::Key1),
                Keycode::Key2 => self.pressed_keys.push(KeyCode::Key2),
                Keycode::Key3 => self.pressed_keys.push(KeyCode::Key3),
                Keycode::Key4 => self.pressed_keys.push(KeyCode::Key4),
                Keycode::Key5 => self.pressed_keys.push(KeyCode::Key5),
                Keycode::Key6 => self.pressed_keys.push(KeyCode::Key6),
                Keycode::Key7 => self.pressed_keys.push(KeyCode::Key7),
                Keycode::Key8 => self.pressed_keys.push(KeyCode::Key8),
                Keycode::Key9 => self.pressed_keys.push(KeyCode::Key9),
                Keycode::Numpad0 => self.pressed_keys.push(KeyCode::Key0),
                Keycode::Numpad1 => self.pressed_keys.push(KeyCode::Key1),
                Keycode::Numpad2 => self.pressed_keys.push(KeyCode::Key2),
                Keycode::Numpad3 => self.pressed_keys.push(KeyCode::Key3),
                Keycode::Numpad4 => self.pressed_keys.push(KeyCode::Key4),
                Keycode::Numpad5 => self.pressed_keys.push(KeyCode::Key5),
                Keycode::Numpad6 => self.pressed_keys.push(KeyCode::Key6),
                Keycode::Numpad7 => self.pressed_keys.push(KeyCode::Key7),
                Keycode::Numpad8 => self.pressed_keys.push(KeyCode::Key8),
                Keycode::Numpad9 => self.pressed_keys.push(KeyCode::Key9),
                Keycode::A => self.pressed_keys.push(KeyCode::A),
                Keycode::B => self.pressed_keys.push(KeyCode::B),
                Keycode::C => self.pressed_keys.push(KeyCode::C),
                Keycode::D => self.pressed_keys.push(KeyCode::D),
                Keycode::E => self.pressed_keys.push(KeyCode::E),
                Keycode::F => self.pressed_keys.push(KeyCode::F),
                Keycode::G => self.pressed_keys.push(KeyCode::G),
                Keycode::H => self.pressed_keys.push(KeyCode::H),
                Keycode::I => self.pressed_keys.push(KeyCode::I),
                Keycode::J => self.pressed_keys.push(KeyCode::J),
                Keycode::K => self.pressed_keys.push(KeyCode::K),
                Keycode::L => self.pressed_keys.push(KeyCode::L),
                Keycode::M => self.pressed_keys.push(KeyCode::M),
                Keycode::N => self.pressed_keys.push(KeyCode::N),
                Keycode::O => self.pressed_keys.push(KeyCode::O),
                Keycode::P => self.pressed_keys.push(KeyCode::P),
                Keycode::Q => self.pressed_keys.push(KeyCode::Q),
                Keycode::R => self.pressed_keys.push(KeyCode::R),
                Keycode::S => self.pressed_keys.push(KeyCode::S),
                Keycode::T => self.pressed_keys.push(KeyCode::T),
                Keycode::U => self.pressed_keys.push(KeyCode::U),
                Keycode::V => self.pressed_keys.push(KeyCode::V),
                Keycode::W => self.pressed_keys.push(KeyCode::W),
                Keycode::X => self.pressed_keys.push(KeyCode::X),
                Keycode::Y => self.pressed_keys.push(KeyCode::Y),
                Keycode::Z => self.pressed_keys.push(KeyCode::Z),
                Keycode::F1 => self.pressed_keys.push(KeyCode::F1),
                Keycode::F2 => self.pressed_keys.push(KeyCode::F2),
                Keycode::F3 => self.pressed_keys.push(KeyCode::F3),
                Keycode::F4 => self.pressed_keys.push(KeyCode::F4),
                Keycode::F5 => self.pressed_keys.push(KeyCode::F5),
                Keycode::F6 => self.pressed_keys.push(KeyCode::F6),
                Keycode::F7 => self.pressed_keys.push(KeyCode::F7),
                Keycode::F8 => self.pressed_keys.push(KeyCode::F8),
                Keycode::F9 => self.pressed_keys.push(KeyCode::F9),
                Keycode::F10 => self.pressed_keys.push(KeyCode::F10),
                Keycode::F11 => self.pressed_keys.push(KeyCode::F11),
                Keycode::F12 => self.pressed_keys.push(KeyCode::F12),
                Keycode::Escape => self.pressed_keys.push(KeyCode::Escape),
                Keycode::Space => self.pressed_keys.push(KeyCode::Space),
                Keycode::LControl => self.pressed_keys.push(KeyCode::LControl),
                Keycode::RControl => self.pressed_keys.push(KeyCode::RControl),
                Keycode::LShift => self.pressed_keys.push(KeyCode::LShift),
                Keycode::RShift => self.pressed_keys.push(KeyCode::RShift),
                Keycode::LAlt => self.pressed_keys.push(KeyCode::LAlt),
                Keycode::RAlt => self.pressed_keys.push(KeyCode::RAlt),
                Keycode::Enter => self.pressed_keys.push(KeyCode::Enter),
                Keycode::Up => self.pressed_keys.push(KeyCode::Up),
                Keycode::Down => self.pressed_keys.push(KeyCode::Down),
                Keycode::Left => self.pressed_keys.push(KeyCode::Left),
                Keycode::Right => self.pressed_keys.push(KeyCode::Right),
                Keycode::Backspace => self.pressed_keys.push(KeyCode::Backspace),
                Keycode::CapsLock => self.pressed_keys.push(KeyCode::CapsLock),
                Keycode::Tab => self.pressed_keys.push(KeyCode::Tab),
                Keycode::Home => self.pressed_keys.push(KeyCode::Home),
                Keycode::End => self.pressed_keys.push(KeyCode::End),
                Keycode::PageUp => self.pressed_keys.push(KeyCode::PageUp),
                Keycode::PageDown => self.pressed_keys.push(KeyCode::PageDown),
                Keycode::Insert => self.pressed_keys.push(KeyCode::Insert),
                Keycode::Delete => self.pressed_keys.push(KeyCode::Delete),
                _ => {} // Ignore other keys not in our KeyCode enum
            }
        }

        // Check if Caps Lock is pressed
        if self.pressed_keys.contains(&KeyCode::CapsLock) && !self.previous_keys.contains(&KeyCode::CapsLock) {
            self.caps_lock = !self.caps_lock; // Toggle Caps Lock state
        }

        // Update buffer if enabled
        if self.buffer_state {
            self.update_buffer();
        }
    }

    pub fn get_keydown_events(&self) -> Vec<KeyCode> {
        // Get the keys that were pressed since the last update
        let mut new_keys = Vec::new();
        for key in &self.pressed_keys {
            if !self.previous_keys.contains(key) {
                new_keys.push(key.clone());
            }
        }
        return new_keys;
    }

    pub fn get_keyup_events(&self) -> Vec<KeyCode> {
        // Get the keys that were released since the last update
        let mut released_keys = Vec::new();
        for key in &self.previous_keys {
            if !self.pressed_keys.contains(key) {
                released_keys.push(key.clone());
            }
        }
        return released_keys;
    }


    // Buffer
    pub fn enable_buffer(&mut self) {
        // Enable the input buffer
        self.buffer_state = true;
        self.buffer.clear(); // Clear the buffer when enabling
    }
    pub fn disable_buffer(&mut self) {
        // Disable the input buffer
        self.buffer_state = false;
    }

    pub fn clear_buffer(&mut self) {
        // Clear the input buffer
        self.buffer.clear();
    }


    fn update_buffer(&mut self) {
        // For each key down event, append the corresponding character to the buffer
        let capital = self.caps_lock ^ (self.pressed_keys.contains(&KeyCode::LShift) || self.pressed_keys.contains(&KeyCode::RShift));

        // Get keydown events
        let keydown_events = self.get_keydown_events();
        for key in keydown_events {
            match key {
                KeyCode::Key0 => self.buffer.push('0'),
                KeyCode::Key1 => self.buffer.push('1'),
                KeyCode::Key2 => self.buffer.push('2'),
                KeyCode::Key3 => self.buffer.push('3'),
                KeyCode::Key4 => self.buffer.push('4'),
                KeyCode::Key5 => self.buffer.push('5'),
                KeyCode::Key6 => self.buffer.push('6'),
                KeyCode::Key7 => self.buffer.push('7'),
                KeyCode::Key8 => self.buffer.push('8'),
                KeyCode::Key9 => self.buffer.push('9'),
                KeyCode::A => if capital { self.buffer.push('A') } else { self.buffer.push('a') },
                KeyCode::B => if capital { self.buffer.push('B') } else { self.buffer.push('b') },
                KeyCode::C => if capital { self.buffer.push('C') } else { self.buffer.push('c') },
                KeyCode::D => if capital { self.buffer.push('D') } else { self.buffer.push('d') },
                KeyCode::E => if capital { self.buffer.push('E') } else { self.buffer.push('e') },
                KeyCode::F => if capital { self.buffer.push('F') } else { self.buffer.push('f') },
                KeyCode::G => if capital { self.buffer.push('G') } else { self.buffer.push('g') },
                KeyCode::H => if capital { self.buffer.push('H') } else { self.buffer.push('h') },
                KeyCode::I => if capital { self.buffer.push('I') } else { self.buffer.push('i') },
                KeyCode::J => if capital { self.buffer.push('J') } else { self.buffer.push('j') },
                KeyCode::K => if capital { self.buffer.push('K') } else { self.buffer.push('k') },
                KeyCode::L => if capital { self.buffer.push('L') } else { self.buffer.push('l') },
                KeyCode::M => if capital { self.buffer.push('M') } else { self.buffer.push('m') },
                KeyCode::N => if capital { self.buffer.push('N') } else { self.buffer.push('n') },
                KeyCode::O => if capital { self.buffer.push('O') } else { self.buffer.push('o') },
                KeyCode::P => if capital { self.buffer.push('P') } else { self.buffer.push('p') },
                KeyCode::Q => if capital { self.buffer.push('Q') } else { self.buffer.push('q') },
                KeyCode::R => if capital { self.buffer.push('R') } else { self.buffer.push('r') },
                KeyCode::S => if capital { self.buffer.push('S') } else { self.buffer.push('s') },
                KeyCode::T => if capital { self.buffer.push('T') } else { self.buffer.push('t') },
                KeyCode::U => if capital { self.buffer.push('U') } else { self.buffer.push('u') },
                KeyCode::V => if capital { self.buffer.push('V') } else { self.buffer.push('v') },
                KeyCode::W => if capital { self.buffer.push('W') } else { self.buffer.push('w') },
                KeyCode::X => if capital { self.buffer.push('X') } else { self.buffer.push('x') },
                KeyCode::Y => if capital { self.buffer.push('Y') } else { self.buffer.push('y') },
                KeyCode::Z => if capital { self.buffer.push('Z') } else { self.buffer.push('z') },
                KeyCode::Space => self.buffer.push(' '),
                KeyCode::Enter => self.buffer.push('\n'),
                KeyCode::Tab => self.buffer.push('\t'),
                _ => {}
            }
            // Backspace handling
            if key == KeyCode::Backspace {
                self.buffer.pop(); // Remove the last character from the buffer
            }
        }
    }
}