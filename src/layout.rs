use crate::hid::HID_CHANNEL;
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use keyberon::key_code::KbHidReport;
use keyberon::layout::Event;
use keyberon::layout::Layout;

/// Basic layout for the keyboard
#[cfg(feature = "keymap_basic")]
use crate::keymap_basic::LAYERS;

/// Keymap by Boris Faure
#[cfg(feature = "keymap_borisfaure")]
use crate::keymap_borisfaure::LAYERS;

/// Test layout for the keyboard
#[cfg(feature = "keymap_test")]
use crate::keymap_test::LAYERS;

const NB_EVENTS: usize = 8;
pub static LAYOUT_CHANNEL: Channel<ThreadModeRawMutex, Event, NB_EVENTS> = Channel::new();

/// Keyboard layout handler
/// Handles layout events into the keymap and sends HID reports to the HID handler
pub async fn layout_handler() {
    let mut layout = Layout::new(&LAYERS);
    loop {
        let event = LAYOUT_CHANNEL.receive().await;
        layout.event(event);
        let report: KbHidReport = layout.keycodes().collect();
        HID_CHANNEL.send(report).await;
    }
}
