///
/// ビットボードを初期化する
///
pub fn init_board() -> (u64, u64) {
    let black: u64 = (1 << 28) | (1 << 35);
    let white: u64 = (1 << 27) | (1 << 36);
    (black, white)
}

