///
/// ビットボードを初期化する
///
pub fn init_board() -> (u64, u64) {
    let black: u64 = (1 << 28) | (1 << 35);
    let white: u64 = (1 << 27) | (1 << 36);
    (black, white)
}

pub fn get_put_board(me: &u64, op: &u64) -> u64 {
    let blank: u64 = !(me | op);  // 空白マス
    let mut put_board: u64 = 0;   // 着手できる位置
    let mut put_candidate: u64;   // 着手できる候補

    // 左右
    // 左右0、それ以外1のマスク
    let masked_op: u64 = op & 0x7e7e_7e7e_7e7e_7e7e;

    // 右方向に返せる場所を求める
    // 自分の石がある所から、左向きに連続して相手の石があるかどうか
    put_candidate = masked_op & (me << 1);
    put_candidate |= masked_op & (put_candidate << 1);
    put_candidate |= masked_op & (put_candidate << 1);
    put_candidate |= masked_op & (put_candidate << 1);
    put_candidate |= masked_op & (put_candidate << 1);
    put_candidate |= masked_op & (put_candidate << 1);
    put_board |= blank & (put_candidate << 1);

    // 左向きに返せる場所を求める
    put_candidate = masked_op & (me >> 1);
    put_candidate |= masked_op & (put_candidate >> 1);
    put_candidate |= masked_op & (put_candidate >> 1);
    put_candidate |= masked_op & (put_candidate >> 1);
    put_candidate |= masked_op & (put_candidate >> 1);
    put_candidate |= masked_op & (put_candidate >> 1);
    put_board |= blank & (put_candidate >> 1);

    // 上下
    // 上下0、それ以外1のマスク
    let masked_op = op & 0x00ff_ffff_ffff_ff00;

    // 上方向
    put_candidate = masked_op & (me << 8);
    put_candidate |= masked_op & (put_candidate << 8);
    put_candidate |= masked_op & (put_candidate << 8);
    put_candidate |= masked_op & (put_candidate << 8);
    put_candidate |= masked_op & (put_candidate << 8);
    put_candidate |= masked_op & (put_candidate << 8);
    put_board |= blank & (put_candidate << 8);

    // 下方向
    put_candidate = masked_op & (me >> 8);
    put_candidate |= masked_op & (put_candidate >> 8);
    put_candidate |= masked_op & (put_candidate >> 8);
    put_candidate |= masked_op & (put_candidate >> 8);
    put_candidate |= masked_op & (put_candidate >> 8);
    put_candidate |= masked_op & (put_candidate >> 8);
    put_board |= blank & (put_candidate >> 8);

    // 斜め
    // 上下左右0、それ以外1のマスク
    let masked_op = op & 0x007e_7e7e_7e7e_7e00;

    // 右上方向
    put_candidate = masked_op & (me << 7);
    put_candidate |= masked_op & (put_candidate << 7);
    put_candidate |= masked_op & (put_candidate << 7);
    put_candidate |= masked_op & (put_candidate << 7);
    put_candidate |= masked_op & (put_candidate << 7);
    put_candidate |= masked_op & (put_candidate << 7);
    put_board |= blank & (put_candidate << 7);

    // 左上方向
    put_candidate = masked_op & (me << 9);
    put_candidate |= masked_op & (put_candidate << 9);
    put_candidate |= masked_op & (put_candidate << 9);
    put_candidate |= masked_op & (put_candidate << 9);
    put_candidate |= masked_op & (put_candidate << 9);
    put_candidate |= masked_op & (put_candidate << 9);
    put_board |= blank & (put_candidate << 9);

    // 右下方向
    put_candidate = masked_op & (me >> 9);
    put_candidate |= masked_op & (put_candidate >> 9);
    put_candidate |= masked_op & (put_candidate >> 9);
    put_candidate |= masked_op & (put_candidate >> 9);
    put_candidate |= masked_op & (put_candidate >> 9);
    put_candidate |= masked_op & (put_candidate >> 9);
    put_board |= blank & (put_candidate >> 9);

    // 左下方向
    put_candidate = masked_op & (me >> 7);
    put_candidate |= masked_op & (put_candidate >> 7);
    put_candidate |= masked_op & (put_candidate >> 7);
    put_candidate |= masked_op & (put_candidate >> 7);
    put_candidate |= masked_op & (put_candidate >> 7);
    put_candidate |= masked_op & (put_candidate >> 7);
    put_board |= blank & (put_candidate >> 7);

    put_board
}