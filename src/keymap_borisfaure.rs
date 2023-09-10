use core::convert::Infallible;
use core::fmt::Debug;
use keyberon::action::{
    d, k, l, m, Action, HoldTapAction, HoldTapConfig,
    SequenceEvent::{self, Filter, Press, Release, Restore, Tap},
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

/// QWERTY layer
const L_QWERTY: usize = 0;
/// LOWER layer
const L_LOWER: usize = 1;
/// RAISE layer
const L_RAISE: usize = 2;
/// NUMBERS layer
const L_NUM: usize = 3;
/// MISC layer
const L_MISC: usize = 4;
/// TMUX layer
const L_TMUX: usize = 5;
/// GAMING layer
const L_GAMING: usize = 6;
/// CAPS layer
const L_CAPS: usize = 7;
/// COLEMAN-DH layer
const L_COLEMAN: usize = 8;

/// Win when held, or W
const HT_W_W: Action = ht!(k(LGui), k(W));
/// Win when held, or O
const HT_W_O: Action = ht!(k(RGui), k(O));
/// Win when held, or Y
const HT_W_Y: Action = ht!(k(RGui), k(Y));
/// Left Control when held, or A
const HT_C_A: Action = ht!(k(LCtrl), k(A));
/// Left Control when held, or Shift-A
const HT_C_SA: Action = ht!(k(LCtrl), s!(A));
/// Right Control when held, or SemiColon
const HT_C_SC: Action = ht!(k(RCtrl), k(SColon));
/// Right Control when held, or O
const HT_C_O: Action = ht!(k(RCtrl), k(O));
/// Left Shift when held, or Z
const HT_S_Z: Action = ht!(k(LShift), k(Z));
/// Right Shift when held, or Slash
const HT_S_SL: Action = ht!(k(RShift), k(Slash));
/// Left Alt when held, or X
const HT_A_X: Action = ht!(k(LAlt), k(X));
/// Left Alt when held, or .
const HT_A_DOT: Action = ht!(k(LAlt), k(Dot));

/// Layer 1 (lower) when held, or Space
const HT_1_SP: Action = ht!(l(L_LOWER), k(Space));

/// Layer 2 (raise) when held, or BackSpace
const HT_2_BS: Action = ht!(l(L_RAISE), k(BSpace));

/// Layer 3 (numbers/Fx) when held, or B
const HT_3_B: Action = ht!(l(L_NUM), k(B));
/// Layer 3 (numbers/Fx) when held, or N
const HT_3_N: Action = ht!(l(L_NUM), k(N));
/// Layer 3 (numbers/Fx) when held, or V
const HT_3_V: Action = ht!(l(L_NUM), k(V));
/// Layer 3 (numbers/Fx) when held, or J
const HT_3_J: Action = ht!(l(L_NUM), k(J));

/// Layer 4 (misc) when held, or T
const HT_4_T: Action = ht!(l(L_MISC), k(T));
/// Layer 4 (misc) when held, or Y
const HT_4_Y: Action = ht!(l(L_MISC), k(Y));
/// Layer 4 (misc) when held, or B
const HT_4_B: Action = ht!(l(L_MISC), k(B));
/// Layer 4 (misc) when held, or K
const HT_4_K: Action = ht!(l(L_MISC), k(K));

/// Layer 5 (tmux) when held, or F
const HT_5_F: Action = ht!(l(L_TMUX), k(F));
/// Layer 5 (tmux) when held, or T
const HT_5_T: Action = ht!(l(L_TMUX), k(T));

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
const CAPS: Action = ma(&[k(CapsLock), d(L_CAPS)].as_slice());
/// Unset Caps Mode
const UNCAPS: Action = ma(&[k(CapsLock), d(L_QWERTY)].as_slice());

/// Change default layer to GAMING
const GAME: Action = d(L_GAMING);
/// Change default layer to QWERTY
const QWERTY: Action = d(L_QWERTY);
/// Change default layer to COLEMAN_DH
const COLEMAN_DH: Action = d(L_COLEMAN);

/// A shortcut to create a `Action::Sequence`, useful to
/// create compact layout.
const fn seq<T, K>(events: &'static &'static [SequenceEvent<K>]) -> Action<T, K>
where
    T: 'static + Debug,
    K: 'static + Debug,
{
    Action::Sequence(events)
}

/// à or À
const A_GRV: Action = seq(&[
    Filter(&[LShift, RShift].as_slice()),
    Tap(RAlt),
    Tap(Grave),
    Restore,
    Tap(A),
]
.as_slice());
/// è or È
const E_GRV: Action = seq(&[
    Filter(&[LShift, RShift].as_slice()),
    Tap(RAlt),
    Tap(Grave),
    Restore,
    Tap(E),
]
.as_slice());
/// ù or Ù
const U_GRV: Action = seq(&[
    Filter(&[LShift, RShift].as_slice()),
    Tap(RAlt),
    Tap(Grave),
    Restore,
    Tap(U),
]
.as_slice());
/// é or É
const E_ACU: Action = seq(&[
    Filter(&[LShift, RShift].as_slice()),
    Tap(RAlt),
    Tap(Quote),
    Restore,
    Tap(E),
]
.as_slice());
/// ê or Ê
const E_CIR: Action = seq(&[
    Filter(&[LShift, RShift].as_slice()),
    Tap(RAlt),
    Press(LShift),
    Tap(Kb6),
    Release(LShift),
    Restore,
    Tap(E),
]
.as_slice());
/// î or Î
const I_CIR: Action = seq(&[
    Filter(&[LShift, RShift].as_slice()),
    Tap(RAlt),
    Press(LShift),
    Tap(Kb6),
    Release(LShift),
    Restore,
    Tap(I),
]
.as_slice());
/// ô or Ô
const O_CIR: Action = seq(&[
    Filter(&[LShift, RShift].as_slice()),
    Tap(RAlt),
    Press(LShift),
    Tap(Kb6),
    Release(LShift),
    Restore,
    Tap(O),
]
.as_slice());
/// ç or Ç
const C_CED: Action = seq(&[
    Filter(&[LShift, RShift].as_slice()),
    Tap(RAlt),
    Tap(Comma),
    Restore,
    Tap(C),
]
.as_slice());
/// œ or Œ
const OE: Action = seq(&[
    Filter(&[LShift, RShift].as_slice()),
    Tap(RAlt),
    Restore,
    Tap(O),
    Tap(E),
]
.as_slice());
/// €
const EURO: Action = seq(&[
    Filter(&[LShift, RShift].as_slice()),
    Tap(RAlt),
    Tap(Equal),
    Press(LShift),
    Tap(E),
    Release(LShift),
    Restore,
]
.as_slice());
/// …
const DOTS: Action = seq(&[
    Filter(&[LShift, RShift].as_slice()),
    Tap(RAlt),
    Tap(Dot),
    Tap(Dot),
    Restore,
]
.as_slice());

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
    { /* 0: QWERTY */
[  Q         {HT_W_W}   E       R         {HT_4_T}       {HT_4_Y}   U         I    {HT_W_O}     P        ],
[ {HT_C_A}    S         D      {HT_5_F}    G              H         J         K     L          {HT_C_SC} ],
[ {HT_S_Z}   {HT_A_X}   C       V         {HT_3_B}       {HT_3_N}   M         ,    {HT_A_DOT}  {HT_S_SL} ],
[  n          n        Escape  {HT_1_SP}   Tab           Enter    {HT_2_BS}  RAlt   n           n        ],
    } { /* 1: LOWER */
        [ !  #  $    '(' ')'    ^       &       {S_INS}  *      ~   ],
        [ =  -  '`'  '{' '}'    Left    PgDown   PgUp   Right  '\\' ],
        [ @  &  %    '[' ']'    n       n         n     '\''   '"'  ],
        [ n  n  n     n   n     Escape  Delete    n      n      n   ],
    } { /* 2: RAISE */
        [ {QWERTY}  n    {E_ACU}  {E_CIR}  {E_GRV}      Home   {U_GRV}  {I_CIR}  {O_CIR}  PScreen ],
        [ {A_GRV}  '_'    +        &        |           Left    Down     Up       Right   PgUp    ],
        [ {EURO}   {OE}  {C_CED}  {CAPS}    n           End     Menu     n       {DOTS}   PgDown  ],
        [ n         n     n       BSpace    Delete      Enter    n       n        n       n       ],
    } { /* 3: NUMBERS Fx */
        [ .  4  5   6         =         /    F1         F2   F3   F4   ],
        [ 0  1  2   3         -         *    F5         F6   F7   F8   ],
        [ ,  7  8   9         +         +    F9         F10  F11  F12  ],
        [ n  n  n  {HT_1_SP} Tab     Enter  {HT_2_BS}    n    n    n   ],
    } { /* 4: MISC TODO: mouse */
        [ Pause  {GAME}             {COLEMAN_DH}    {QWERTY}       n      n  n  n  n  n ],
        [ n      VolDown            Mute            VolUp          n      n  n  n  n  n ],
        [ n      MediaPreviousSong  MediaPlayPause  MediaNextSong  n      n  n  n  n  n ],
        [ n      n                  n               n              n      n  n  n  n  n ],
    } { /* 5: TMUX */
        [ {T_6}   {T_7} {T_8}   {T_9}   {T_0}      {T_1}   {T_2} {T_3}   {T_4}   {T_5}   ],
        [ {T_LST}  n     n       n       n         {T_PRV}  n    {T_SCR} {T_NXT} {T_CMD} ],
        [  n       n    {T_NEW} {T_CPY} {T_PST}     n       n    {T_RNM} {T_MOV} {T_PST} ],
        [  n       n     n       n       n          n       n     n       n       n      ],
    } { /* 6: Gaming */
        [ Q  W  E   R         T     {HT_4_Y} U          I  {HT_W_O}     P       ],
        [ A  S  D   F         G      H       J          K   L         {HT_C_SC} ],
        [ Z  X  C   V         B      N       M          ,  {HT_A_DOT} {HT_S_SL} ],
        [ n  n  n  {HT_1_SP}  Tab    Enter   {HT_2_BS}  n   n          n        ],
    } { /* 7: Caps */
[ {s!(Q)}   {s!(W)}  {s!(E)}   {s!(R)}  {s!(T)}         {s!(Y)}   {s!(U)}     {s!(I)}  {s!(O)}   {s!(P)}   ],
[ {HT_C_SA} {s!(S)}  {s!(D)}   {s!(F)}  {s!(G)}         {s!(H)}   {s!(J)}     {s!(K)}  {s!(L)}   {HT_C_SC} ],
[ {s!(Z)}   {s!(X)}  {s!(C)}   {s!(V)}  {s!(B)}         {s!(N)}   {s!(M)}      ,        .         /        ],
[  n         n       {UNCAPS}   '_'      Space           Enter    {HT_2_BS}    n        n         n        ],
    } { /* 8: Coleman-DH */
[  Q         {HT_W_W}   F       P         {HT_4_B}    {HT_4_K}   L         U   {HT_W_Y}     ;        ],
[ {HT_C_A}    R         S      {HT_5_T}    G           M         N         E    I          {HT_C_O}  ],
[ {HT_S_Z}   {HT_A_X}   C       D         {HT_3_V}    {HT_3_J}   H         ,   {HT_A_DOT}  {HT_S_SL} ],
[  n          n        Escape  {HT_1_SP}   Tab         Enter    {HT_2_BS} RAlt  n           n        ],
    }
};
