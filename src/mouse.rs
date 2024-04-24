use crate::layout::CustomEvent;
use keyberon::layout::CustomEvent as KbCustomEvent;
use usbd_hid::descriptor::MouseReport;

/// Mouse handler
#[derive(Debug, Default)]
pub struct MouseHandler {
    /// Going Up
    pub up: bool,
    /// Going Down
    pub down: bool,
    /// Going Left
    pub left: bool,
    /// Going Right
    pub right: bool,

    /// Left click is pressed
    pub left_click: bool,
    /// Right click is pressed
    pub right_click: bool,
    /// Middle click is pressed
    pub middle_click: bool,

    /// Wheel up
    pub wheel_up: bool,
    /// Wheel down
    pub wheel_down: bool,

    /// How fast the mouse moves
    pub rate: i8,
}

impl MouseHandler {
    /// Create a new mouse handler
    pub fn new() -> Self {
        MouseHandler {
            rate: 1,
            ..Default::default()
        }
    }
    /// Process a custom event
    pub fn process_event(&mut self, kb_cs_event: keyberon::layout::CustomEvent<CustomEvent>) {
        if let Some((event, is_pressed)) = match kb_cs_event {
            KbCustomEvent::Press(event) => Some((event, true)),
            KbCustomEvent::Release(event) => Some((event, false)),
            _ => None,
        } {
            defmt::info!(
                "Mouse event: {:?} (is_pressed:{:?})",
                defmt::Debug2Format(&event),
                is_pressed
            );
            match event {
                CustomEvent::MouseUp => self.up = is_pressed,
                CustomEvent::MouseDown => self.down = is_pressed,
                CustomEvent::MouseRight => self.right = is_pressed,
                CustomEvent::MouseLeft => self.left = is_pressed,
                CustomEvent::MouseLeftClick => self.left_click = is_pressed,
                CustomEvent::MouseRightClick => self.right_click = is_pressed,
                CustomEvent::MouseMiddleClick => self.middle_click = is_pressed,
                CustomEvent::MouseScrollUp => self.wheel_up = is_pressed,
                CustomEvent::MouseScrollDown => self.wheel_down = is_pressed,
            }
        }
    }

    /// Compute the state of the mouse
    pub fn tick(&mut self) {}

    /// Generate a HID report for the mouse
    pub fn generate_hid_report(&mut self) -> MouseReport {
        let mut report = MouseReport::default();
        if self.up {
            report.y = self.rate;
        } else if self.down {
            report.y = -self.rate;
        }
        if self.left {
            report.x = -self.rate;
        } else if self.right {
            report.x = self.rate;
        }
        if self.left_click {
            report.buttons |= 1;
        }
        if self.right_click {
            report.buttons |= 2;
        }
        if self.middle_click {
            report.buttons |= 4;
        }
        if self.wheel_up {
            report.wheel = 1;
        } else if self.wheel_down {
            report.wheel = -1;
        }
        report
    }
}
