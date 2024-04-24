use crate::layout::CustomEvent;
use usbd_hid::descriptor::MouseReport;

/// Mouse handler
pub struct MouseHandler {
    /// X position
    pub x: i8,
    /// Y position
    pub y: i8,
    /// Buttons
    pub buttons: u8,
}

impl MouseHandler {
    /// Create a new mouse handler
    pub fn new() -> Self {
        MouseHandler {
            x: 0,
            y: 0,
            buttons: 0,
        }
    }

    pub fn new_event(&mut self, kb_cs_event: keyberon::layout::CustomEvent<CustomEvent>) {
        if let Some((event, _is_pressed)) = match kb_cs_event {
            keyberon::layout::CustomEvent::Press(event) => Some((event, true)),
            keyberon::layout::CustomEvent::Release(event) => Some((event, false)),
            _ => None,
        } {
            match event {
                CustomEvent::MouseNorth => self.y -= 1,
                CustomEvent::MouseNorthEast => {
                    self.x += 1;
                    self.y -= 1;
                }
                CustomEvent::MouseEast => self.x += 1,
                CustomEvent::MouseSouthEast => {
                    self.x += 1;
                    self.y += 1;
                }
                CustomEvent::MouseSouth => self.y += 1,
                CustomEvent::MouseSouthWest => {
                    self.x -= 1;
                    self.y += 1;
                }
                CustomEvent::MouseWest => self.x -= 1,
                CustomEvent::MouseNorthWest => {
                    self.x -= 1;
                    self.y -= 1;
                }
                CustomEvent::MouseLeftClick => self.buttons |= 1,
                CustomEvent::MouseRightClick => self.buttons |= 2,
                CustomEvent::MouseMiddleClick => self.buttons |= 4,
                CustomEvent::MouseScrollUp => self.buttons |= 8,
                CustomEvent::MouseScrollDown => self.buttons |= 16,
            }
        }
    }

    /// Generate a HID report for the mouse
    pub fn generate_hid_report(&mut self) -> MouseReport {
        let mut report = MouseReport::default();
        report.x = self.x;
        report.y = self.y;
        report.buttons = self.buttons;
        self.x = 0;
        self.y = 0;
        self.buttons = 0;
        report
    }
}
