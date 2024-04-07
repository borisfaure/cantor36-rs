use defmt::*;
use embassy_stm32::gpio::Input;
use embassy_time::{Duration, Ticker};

/// Pins for the keyboard matrix
pub type Matrix<'a> = [[Option<Input<'a>>; 5]; 4];

fn scan_matrix(matrix: &Matrix) {
    for row in 0..4 {
        for col in 0..5 {
            if let Some(ref key) = matrix[row][col] {
                if key.is_low() {
                    info!("Key pressed: row={}, col={}", row, col);
                }
            }
        }
    }
}

/// Loop that scans the keyboard matrix
pub async fn matrix_scanner(matrix: Matrix<'_>) {
    let mut ticker = Ticker::every(Duration::from_hz(1000));

    loop {
        scan_matrix(&matrix);

        ticker.next().await;
    }
}
