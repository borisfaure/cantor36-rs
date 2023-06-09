use core::convert::Infallible;
use core::fmt::Debug;
use keyberon::action::{
    d, k, l, m, Action, HoldTapAction, HoldTapConfig,
    SequenceEvent::{self, Press, Release, Tap},
};
use keyberon::key_code::KeyCode::*;
use keyberon::layout::Layout;

/// Keyboard Layout type to mask the number of layers
pub type KBLayout = Layout<10, 4, 9, Infallible>;

/// Helper to create keys shifted
macro_rules! s {
    ($k:ident) => {
        m(&[LShift, $k].as_slice())
    };
}

/// Timeout to consider a key as held
const TIMEOUT: u16 = 200;
/// Disable tap_hold_interval
const TAP_HOLD_INTERVAL: u16 = 0;

/// Helper to create a HoldTapAction
macro_rules! ht {
    ($h:expr, $t:expr) => {
        Action::HoldTap(&HoldTapAction {
            timeout: TIMEOUT,
            tap_hold_interval: TAP_HOLD_INTERVAL,
            config: HoldTapConfig::Default,
            hold: $h,
            tap: $t,
        })
    };
}

/// Win when held, or W
const HT_W_W: Action = ht!(k(LGui), k(W));
/// Win when held, or O
const HT_W_O: Action = ht!(k(RGui), k(O));
/// Left Control when held, or A
const HT_C_A: Action = ht!(k(LCtrl), k(A));
/// Left Control when held, or Shift-A
const HT_C_SA: Action = ht!(k(LCtrl), s!(A));
/// Right Control when held, or SemiColon
const HT_C_SC: Action = ht!(k(RCtrl), k(SColon));
/// Left Shift when held, or Z
const HT_S_Z: Action = ht!(k(LShift), k(Z));
/// Right Shift when held, or Slash
const HT_S_SL: Action = ht!(k(RShift), k(Slash));
/// Left Alt when held, or X
const HT_A_X: Action = ht!(k(LAlt), k(X));
/// Left Alt when held, or .
const HT_A_DOT: Action = ht!(k(LAlt), k(Dot));

/// Layer 1 (lower) when held, or Tab
const HT_1_TAB: Action = ht!(l(1), k(Tab));

/// Layer 2 (raise) when held, or BackSpace
const HT_2_BS: Action = ht!(l(2), k(BSpace));

/// Layer 3 (numbers/Fx) when held, or B
const HT_3_B: Action = ht!(l(3), k(B));
/// Layer 3 (numbers/Fx) when held, or N
const HT_3_N: Action = ht!(l(3), k(N));

/// Num Mode
const NUM: Action = ma(&[k(NumLock), d(4)].as_slice());
/// Unset Num Mode
const UNNUM: Action = ma(&[k(NumLock), d(0)].as_slice());

/// Layer 5 (misc) when held, or T
const HT_5_T: Action = ht!(l(5), k(T));
/// Layer 5 (misc) when held, or Y
const HT_5_Y: Action = ht!(l(5), k(Y));

/// Layer 6 (tmux) when held, or F
const HT_6_F: Action = ht!(l(6), k(F));

/// Shift-Insert
const S_INS: Action = m(&[LShift, Insert].as_slice());

/// A shortcut to create a `Action::MultipleActions`, useful to
/// create compact layout.
const fn ma<T, K>(actions: &'static &'static [Action<T, K>]) -> Action<T, K>
where
    T: Debug,
    K: Debug,
{
    Action::MultipleActions(actions)
}

/// Caps Mode
const CAPS: Action = ma(&[k(CapsLock), d(8)].as_slice());
/// Unset Caps Mode
const UNCAPS: Action = ma(&[k(CapsLock), d(0)].as_slice());

/// Change default layer to GAME
const GAME: Action = d(7);
/// Change default layer to BASE
const BASE: Action = d(0);

/// A shortcut to create a `Action::Sequence`, useful to
/// create compact layout.
const fn seq<T, K>(events: &'static &'static [SequenceEvent<K>]) -> Action<T, K>
where
    T: 'static + Debug,
    K: 'static + Debug,
{
    Action::Sequence(events)
}

/// à
const A_GRV: Action = seq(&[Tap(RAlt), Tap(Grave), Tap(A)].as_slice());
/// è
const E_GRV: Action = seq(&[Tap(RAlt), Tap(Grave), Tap(E)].as_slice());
/// ù
const U_GRV: Action = seq(&[Tap(RAlt), Tap(Grave), Tap(U)].as_slice());
/// é
const E_ACU: Action = seq(&[Tap(RAlt), Tap(Quote), Tap(E)].as_slice());
/// ê
const E_CIR: Action =
    seq(&[Tap(RAlt), Press(LShift), Tap(Kb6), Release(LShift), Tap(E)].as_slice());
/// î
const I_CIR: Action =
    seq(&[Tap(RAlt), Press(LShift), Tap(Kb6), Release(LShift), Tap(I)].as_slice());
/// ô
const O_CIR: Action =
    seq(&[Tap(RAlt), Press(LShift), Tap(Kb6), Release(LShift), Tap(O)].as_slice());
/// ç
const C_CED: Action = seq(&[Tap(RAlt), Tap(Comma), Tap(C)].as_slice());
/// œ
const OE: Action = seq(&[Tap(RAlt), Tap(O), Tap(E)].as_slice());
/// €
const EURO: Action = seq(&[Tap(RAlt), Tap(Equal), Tap(E)].as_slice());
/// …
const DOTS: Action = seq(&[Tap(RAlt), Tap(Dot), Tap(Dot)].as_slice());

/// Tmux: new window
const T_NEW: Action = seq(&[Press(LCtrl), Tap(A), Release(LCtrl), Tap(C)].as_slice());
/// Tmux: previous window
const T_PRV: Action = seq(&[Press(LCtrl), Tap(A), Release(LCtrl), Tap(P)].as_slice());
/// Tmux: next window
const T_NXT: Action = seq(&[Press(LCtrl), Tap(A), Release(LCtrl), Tap(N)].as_slice());
/// Tmux: last window
const T_LST: Action = seq(&[Press(LCtrl), Tap(A), Release(LCtrl), Tap(L)].as_slice());
/// Tmux: command
const T_CMD: Action = seq(&[
    Press(LCtrl),
    Tap(A),
    Release(LCtrl),
    Press(LShift),
    Tap(SColon),
    Release(LShift),
]
.as_slice());
/// Tmux: copy
const T_CPY: Action = seq(&[Press(LCtrl), Tap(A), Release(LCtrl), Tap(LBracket)].as_slice());
/// Tmux: paste
const T_PST: Action = seq(&[Press(LCtrl), Tap(A), Release(LCtrl), Tap(RBracket)].as_slice());
/// Tmux: scroll
const T_SCR: Action = seq(&[Press(LCtrl), Tap(A), Release(LCtrl), Tap(PgUp)].as_slice());
/// Tmux: move
const T_MOV: Action = seq(&[Press(LCtrl), Tap(A), Release(LCtrl), Tap(Dot)].as_slice());
/// Tmux: rename
const T_RNM: Action = seq(&[Press(LCtrl), Tap(A), Release(LCtrl), Tap(Comma)].as_slice());
/// Tmux: go to window 1
const T_1: Action = seq(&[Press(LCtrl), Tap(A), Release(LCtrl), Tap(Kb1)].as_slice());
/// Tmux: go to window 2
const T_2: Action = seq(&[Press(LCtrl), Tap(A), Release(LCtrl), Tap(Kb2)].as_slice());
/// Tmux: go to window 3
const T_3: Action = seq(&[Press(LCtrl), Tap(A), Release(LCtrl), Tap(Kb3)].as_slice());
/// Tmux: go to window 4
const T_4: Action = seq(&[Press(LCtrl), Tap(A), Release(LCtrl), Tap(Kb4)].as_slice());
/// Tmux: go to window 5
const T_5: Action = seq(&[Press(LCtrl), Tap(A), Release(LCtrl), Tap(Kb5)].as_slice());
/// Tmux: go to window 6
const T_6: Action = seq(&[Press(LCtrl), Tap(A), Release(LCtrl), Tap(Kb6)].as_slice());
/// Tmux: go to window 7
const T_7: Action = seq(&[Press(LCtrl), Tap(A), Release(LCtrl), Tap(Kb7)].as_slice());
/// Tmux: go to window 8
const T_8: Action = seq(&[Press(LCtrl), Tap(A), Release(LCtrl), Tap(Kb8)].as_slice());
/// Tmux: go to window 9
const T_9: Action = seq(&[Press(LCtrl), Tap(A), Release(LCtrl), Tap(Kb9)].as_slice());
/// Tmux: go to window 0
const T_0: Action = seq(&[Press(LCtrl), Tap(A), Release(LCtrl), Tap(Kb0)].as_slice());

#[rustfmt::skip]
/// Layout
pub static LAYERS: keyberon::layout::Layers<10, 4, 9, Infallible> = keyberon::layout::layout! {
    { /* 0: BASE */
[  Q         {HT_W_W}   E       R         {HT_5_T}    {HT_5_Y}   U         I  {HT_W_O}     P        ],
[ {HT_C_A}    S         D      {HT_6_F}    G           H         J         K   L          {HT_C_SC} ],
[ {HT_S_Z}   {HT_A_X}   C       V         {HT_3_B}    {HT_3_N}   M         ,  {HT_A_DOT}  {HT_S_SL} ],
[  n          n        Escape  {HT_1_TAB}  Space       Enter    {HT_2_BS}  n   n           n        ],
    } { /* 1: LOWER */
        [ !  #  $    '(' ')'    ^       &       {S_INS}  *      ~   ],
        [ =  -  '`'  '{' '}'    Left    PgDown   PgUp   Right  '\\' ],
        [ @  &  %    '[' ']'    n       n         n     '\''   '"'  ],
        [ n  n  n     n   n     Escape  Delete    n      n      n   ],
    } { /* 2: RAISE */
        [ {BASE}   n    {E_ACU}  {E_CIR}  {E_GRV}      Home   {U_GRV}  {I_CIR}  {O_CIR}  PScreen ],
        [ {A_GRV} '_'    +        &        |           Left    Down     Up       Right   PgUp    ],
        [ {EURO}  {OE}  {C_CED}  {CAPS}   {NUM}        End     Menu     n       {DOTS}   PgDown  ],
        [ n        n     n        n        n           Enter   BSpace   n        n       n       ],
    } { /* 3: NUMBERS Fx */
        [ .  4  5   6          =         /       F1   F2   F3   F4  ],
        [ 0  1  2   3          -         *       F5   F6   F7   F8  ],
        [ ,  7  8   9          {NUM}     +       F9   F10  F11  F12 ],
        [ n  n  n  {HT_1_TAB}  Space    Enter   {HT_2_BS}    n    n    n   ],
    } { /* 4: NUMBERS Fx Lock */
        [ .  4  5   6          =         /       F1   F2   F3   F4  ],
        [ 0  1  2   3          -         *       F5   F6   F7   F8  ],
        [ ,  7  8   9          {UNNUM}   +       F9   F10  F11  F12 ],
        [ n  n  n  {HT_1_TAB}  Space    Enter   {HT_2_BS}    n    n    n   ],
    } { /* 5: MISC TODO: mouse */
        [ Pause  {GAME}             n               R              n      n  n  n  n  n ],
        [ n      VolUp              Mute            VolDown        n      n  n  n  n  n ],
        [ n      MediaPreviousSong  MediaPlayPause  MediaNextSong  n      n  n  n  n  n ],
        [ n      n                  n               n              n      n  n  n  n  n ],
    } { /* 6: TMUX TODO: sequences */
        [ {T_6}   {T_7} {T_8}   {T_9}   {T_0}      {T_1}   {T_2} {T_3}   {T_4}   {T_5}   ],
        [ {T_LST}  n     n       n       n         {T_PRV}  n    {T_SCR} {T_NXT} {T_CMD} ],
        [  n       n    {T_NEW} {T_CPY} {T_PST}     n       n    {T_RNM} {T_MOV} {T_PST} ],
        [  n       n     n       n       n          n       n     n       n       n      ],
    } { /* 7: Gaming */
        [ Q  W  E   R           T      Y       U          I  {HT_W_O}     P       ],
        [ A  S  D   F           G      H       J          K   L         {HT_C_SC} ],
        [ Z  X  C   V           B      N       M          ,  {HT_A_DOT} {HT_S_SL} ],
        [ n  n  n  {HT_1_TAB}  Space  Enter   {HT_2_BS}  n   n          n        ],
    } { /* 8: Caps */
[ {s!(Q)}   {s!(W)}  {s!(E)}   {s!(R)}  {s!(T)}         {s!(Y)}   {s!(U)}     {s!(I)}  {s!(O)}   {s!(P)}   ],
[ {HT_C_SA} {s!(S)}  {s!(D)}   {s!(F)}  {s!(G)}         {s!(H)}   {s!(J)}     {s!(K)}  {s!(L)}   {HT_C_SC} ],
[ {s!(Z)}   {s!(X)}  {s!(C)}   {s!(V)}  {s!(B)}         {s!(N)}   {s!(M)}      ,        .         /        ],
[  n         n       {UNCAPS}   '_'      Space           Enter    {HT_2_BS}   n        n         n        ],
    }
};
