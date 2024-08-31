use crate::layout::CustomEvent;
use keyberon::layout::CustomEvent as KbCustomEvent;
use usbd_hid::descriptor::MouseReport;

/// Maximum rate for the mouse
const MAX_RATE: i8 = 127;
/// Minimum rate for the mouse
const MIN_RATE: i8 = 1;
/// Update Frequency, in Hz
const UPDATE_FREQUENCY: u32 = 16;
/// Number of ticks between rate changes
const RATE_CHANGE_TICKS: u32 = 1000 / UPDATE_FREQUENCY;

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

    /// How fast the mouse moves horizontally
    pub rate_horizontal: i8,
    /// How fast the mouse moves vertically
    pub rate_vertical: i8,

    /// Number of ticks
    n: u32,
    /// Had change
    has_changed: bool,
}

impl MouseHandler {
    /// Create a new mouse handler
    pub fn new() -> Self {
        MouseHandler {
            rate_horizontal: MIN_RATE,
            rate_vertical: MIN_RATE,
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
            self.has_changed = true;
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

    /// Check if the mouse is active
    fn is_active(&self) -> bool {
        self.up
            || self.down
            || self.left
            || self.right
            || self.left_click
            || self.right_click
            || self.middle_click
            || self.wheel_up
            || self.wheel_down
    }

    /// Compute the state of the mouse. Called every 1ms
    pub fn tick(&mut self) -> Option<MouseReport> {
        self.n += 1;
        if self.n < RATE_CHANGE_TICKS {
            return None;
        }
        self.n = 0;
        if self.up || self.down {
            self.rate_vertical = self.rate_vertical.checked_mul(2).unwrap_or(MAX_RATE);
        } else {
            self.rate_vertical = MIN_RATE;
        }
        if self.left || self.right {
            self.rate_horizontal = self.rate_horizontal.checked_mul(2).unwrap_or(MAX_RATE);
        } else {
            self.rate_horizontal = MIN_RATE;
        }
        if self.has_changed || self.is_active() {
            self.has_changed = false;
            Some(self.generate_hid_report())
        } else {
            None
        }
    }

    /// Generate a HID report for the mouse
    fn generate_hid_report(&mut self) -> MouseReport {
        let mut report = MouseReport {
            x: 0,
            y: 0,
            buttons: 0,
            wheel: 0,
            pan: 0,
        };
        if self.up {
            report.y = self.rate_vertical;
        } else if self.down {
            report.y = -self.rate_vertical;
        }
        if self.left {
            report.x = -self.rate_horizontal;
        } else if self.right {
            report.x = self.rate_horizontal;
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
