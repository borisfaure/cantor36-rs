use core::convert::Infallible;
use keyberon::layout::Layout;

/// Keyboard Layout type to mask the number of layers
pub type KBLayout = Layout<10, 4, 1, Infallible>;

#[rustfmt::skip]
/// Layout
pub static LAYERS: keyberon::layout::Layers<10, 4, 1, Infallible> = keyberon::layout::layout! {
    { // 0: Base Layer
        [ Q  W  E  R  T      Y  U  I  O  P ],
        [ A  S  D  F  G      H  J  K  L  ; ],
        [ Z  X  C  V  B      N  M  ,  .  / ],
        [ n  n  1  2  3      4  5  6  n  n ],
    }
};
