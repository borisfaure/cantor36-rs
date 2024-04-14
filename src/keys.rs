use crate::layout::LAYOUT_CHANNEL;
use crate::side::is_host;
use embassy_stm32::gpio::Input;
use embassy_time::{Duration, Ticker};
use keyberon::debounce::Debouncer;
use keyberon::layout::Event;

/// Keyboard matrix rows
const ROWS: usize = 4;
/// Keyboard matrix columns
const COLS: usize = 5;
/// Keyboard matrix refresh rate, in Hz
const REFRESH_RATE: u16 = 1000;
/// Keyboard matrix debouncing time, in ms
const DEBOUNCE_TIME: u16 = 5;
/// Keyboard bounce number
const NB_BOUNCE: u16 = REFRESH_RATE * DEBOUNCE_TIME / 1000;

/// Pins for the keyboard matrix
pub type Matrix<'a> = [[Option<Input<'a>>; COLS]; ROWS];
/// Keyboard matrix state
type MatrixState = [[bool; COLS]; ROWS];
/// Create a new keyboard matrix state
fn matrix_state_new() -> MatrixState {
    [[false; COLS]; ROWS]
}

/// Scan the keyboard matrix
fn scan_matrix(matrix: &Matrix) -> MatrixState {
    let mut matrix_state = matrix_state_new();
    for row in 0..ROWS {
        for col in 0..COLS {
            if let Some(ref key) = matrix[row][col] {
                if key.is_low() {
                    matrix_state[row][col] = true;
                }
            }
        }
    }
    matrix_state
}

/// Transform key events from other keyboard half by mirroring coordinates
#[cfg(feature = "right")]
fn transform_keypress_coordinates(e: Event) -> Event {
    // mirror coordinates for events for right half
    e.transform(|i, j| (i, 9 - j))
}

/// Do not transform key events from other keyboard half
#[cfg(feature = "left")]
fn transform_keypress_coordinates(e: Event) -> Event {
    e
}

/// Loop that scans the keyboard matrix
pub async fn matrix_scanner(matrix: Matrix<'_>) {
    let mut ticker = Ticker::every(Duration::from_hz(REFRESH_RATE.into()));

    let mut debouncer = Debouncer::new(matrix_state_new(), matrix_state_new(), NB_BOUNCE);

    loop {
        let is_host = is_host();
        for event in debouncer
            .events(scan_matrix(&matrix))
            .map(transform_keypress_coordinates)
        {
            if is_host {
                LAYOUT_CHANNEL.send(event).await;
            }
        }

        ticker.next().await;
    }
}
