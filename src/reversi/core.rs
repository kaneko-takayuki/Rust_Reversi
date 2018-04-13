///
/// ビットボードを初期化する
///
pub fn init_board() -> (u64, u64) {
    let black: u64 = (1 << 28) | (1 << 35);
    let white: u64 = (1 << 27) | (1 << 36);
    (black, white)
}

///
/// 石数をカウントする
///
pub fn count(x: &u64) -> i32 {
    let mut tmp: u64 = (*x & 0x5555_5555_5555_5555) + ((*x >> 1) & 0x5555_5555_5555_5555);
    tmp = (tmp & 0x3333_3333_3333_3333) + ((tmp >> 2) & 0x3333_3333_3333_3333);
    tmp = (tmp & 0x0f0f_0f0f_0f0f_0f0f) + ((tmp >> 4) & 0x0f0f_0f0f_0f0f_0f0f);
    tmp = (tmp & 0x00ff_00ff_00ff_00ff) + ((tmp >> 8) & 0x00ff_00ff_00ff_00ff);
    tmp = (tmp & 0x0000_ffff_0000_ffff) + ((tmp >> 16) & 0x0000_ffff_0000_ffff);
    tmp = (tmp & 0x0000_0000_ffff_ffff) + ((tmp >> 32) & 0x0000_0000_ffff_ffff);
    tmp as i32
}

///
/// ゲームの終了判定
///
pub fn is_end(black: &u64, white: &u64) -> bool {
    (get_put_board(&black, &white) == 0) && (get_put_board(&white, &black) == 0)
}

///
/// 着手可能位置を求める
///
pub fn get_put_board(me: &u64, op: &u64) -> u64 {
    let mut put_board: u64 = 0;   // 着手できる位置
    let mut put_candidate: u64;   // 着手できる候補
    let blank: u64 = !(me | op);  // 空白マス

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

///
/// pos_nに石を置いた時に反転する箇所が1のビットボードを求める
///
fn get_reverse_board(me: &u64, op: &u64, pos_n: &i32) -> u64 {
    let mut reverse_board: u64 = 0;  // 反転位置
    let mut reverse_candidate: u64;  // 反転候補
    let pos: u64 = 1 << (*pos_n as u64);      // 石を置く位置
    let pos_n = *pos_n as u64; // 石を置くマス番号

    // 左右
    // 左右0、それ以外1のマスク
    let masked_op: u64 = op & 0x7e7e_7e7e_7e7e_7e7e;

    // 右方向
    // range_maskは、反転するかもしれない範囲マスク
    // interposeは、pos_nに置いた時右方向に返せる場合、挟む石の位置
    let range_mask: u64 = 0x0000_0000_0000_00fe << pos_n;
    let interpose: u64 = ((masked_op | !range_mask).wrapping_add(1)) & (range_mask & me);
    if interpose != 0 {reverse_board |= (interpose.wrapping_sub(1)) & range_mask}

    // 左方向
    reverse_candidate = (pos >> 1) & masked_op;
    reverse_candidate |= masked_op & (reverse_candidate >> 1);
    reverse_candidate |= masked_op & (reverse_candidate >> 1);
    reverse_candidate |= masked_op & (reverse_candidate >> 1);
    reverse_candidate |= masked_op & (reverse_candidate >> 1);
    reverse_candidate |= masked_op & (reverse_candidate >> 1);
    reverse_board |= reverse_candidate & ((!(reverse_candidate >> 1 & me)).wrapping_add(1));

    // 上下
    // 上下0、それ以外1のマスク
    let masked_op = op & 0x00ff_ffff_ffff_ff00;

    // 上方向
    reverse_candidate = (pos >> 8) & masked_op;
    reverse_candidate |= masked_op & (reverse_candidate >> 8);
    reverse_candidate |= masked_op & (reverse_candidate >> 8);
    reverse_candidate |= masked_op & (reverse_candidate >> 8);
    reverse_candidate |= masked_op & (reverse_candidate >> 8);
    reverse_candidate |= masked_op & (reverse_candidate >> 8);
    reverse_board |= reverse_candidate & ((!(reverse_candidate >> 8 & me)).wrapping_add(1));

    // 下方向
    let range_mask: u64 = 0x0101_0101_0101_0100 << pos_n;
    let interpose: u64 = ((masked_op | !range_mask).wrapping_add(1)) & (range_mask & me);
    if interpose != 0 {reverse_board |= (interpose.wrapping_sub(1)) & range_mask}

    // 斜め
    // 上下左右0、それ以外1のマスク
    let masked_op = op & 0x007e_7e7e_7e7e_7e00;

    // 右上方向
    reverse_candidate = (pos >> 7) & masked_op;
    reverse_candidate |= masked_op & (reverse_candidate >> 7);
    reverse_candidate |= masked_op & (reverse_candidate >> 7);
    reverse_candidate |= masked_op & (reverse_candidate >> 7);
    reverse_candidate |= masked_op & (reverse_candidate >> 7);
    reverse_candidate |= masked_op & (reverse_candidate >> 7);
    reverse_board |= reverse_candidate & ((!(reverse_candidate >> 7 & me)).wrapping_add(1));

    // 左上方向
    reverse_candidate = (pos >> 9) & masked_op;
    reverse_candidate |= masked_op & (reverse_candidate >> 9);
    reverse_candidate |= masked_op & (reverse_candidate >> 9);
    reverse_candidate |= masked_op & (reverse_candidate >> 9);
    reverse_candidate |= masked_op & (reverse_candidate >> 9);
    reverse_candidate |= masked_op & (reverse_candidate >> 9);
    reverse_board |= reverse_candidate & ((!(reverse_candidate >> 9 & me)).wrapping_add(1));

    // 右下方向
    let range_mask: u64 = 0x8040_2010_0804_0200 << pos_n;
    let interpose: u64 = ((masked_op | !range_mask).wrapping_add(1)) & (range_mask & me);
    if interpose != 0 {reverse_board |= (interpose.wrapping_sub(1)) & range_mask}

    // 左下方向
    let range_mask: u64 = 0x0102_0408_1020_4080 << pos_n;
    let interpose: u64 = ((masked_op | !range_mask).wrapping_add(1)) & (range_mask & me);
    if interpose != 0 {reverse_board |= (interpose.wrapping_sub(1)) & range_mask}

    reverse_board
}

///
/// ターンをスキップするかどうか判定
///
pub fn skip_turn(black: &u64, white: &u64, black_turn: &bool) -> bool {
    if *black_turn {
        get_put_board(&black, &white) == 0
    } else {
        get_put_board(&white, &black) == 0
    }
}

///
/// pos_nに石が置けるかどうか検証
///
pub fn enable_put_stone(black: &u64, white: &u64, pos_n: &i32, black_turn: &bool) -> bool {
    let pos: u64 = 1 << (*pos_n as u64);

    if *black_turn {
        (get_put_board(&black, &white) & pos) != 0
    } else {
        (get_put_board(&white, &black) & pos) != 0
    }
}

///
/// 石を置く
///
pub fn put_stone(black: &u64, white: &u64, pos_n: &i32, black_turn: &bool) -> (u64, u64) {
    let pos: u64 = 1 << (*pos_n as u64);

    if *black_turn {
        let reverse_board: u64 = get_reverse_board(&black, &white, &pos_n);
        let new_black = black ^ (reverse_board | pos);
        let new_white = white ^ reverse_board;
        return (new_black, new_white);
    }
    else {
        let reverse_board: u64 = get_reverse_board(&white, &black, &pos_n);
        let new_black = *black ^ reverse_board;
        let new_white = *white ^ (reverse_board | pos);
        return (new_black, new_white);
    }
}
