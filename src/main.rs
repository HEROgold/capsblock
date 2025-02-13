use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::{Enigo, Key, Keyboard, Settings};


fn main() {
    let mut enigo = Enigo::new(&Settings::default()).expect("Failed to create Enigo");
    let mut is_pressed = false;

    loop {
        let state = DeviceState::new();
        let target = &Keycode::CapsLock;
        let pressed = state.get_keys();

        // When key is pressed
        if pressed.contains(target) && !is_pressed {
            is_pressed = true;
        }
        // when key is released
        if !pressed.contains(target) && is_pressed {
            is_pressed = false;
            // TODO: check if the capslock state is on, then turn it off
            enigo.key(Key::CapsLock, enigo::Direction::Release).unwrap();
        }
    }
}
