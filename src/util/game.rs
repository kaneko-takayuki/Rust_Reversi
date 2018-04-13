use reversi::core;
use util::cl;

pub fn game_start() {
    // ゲームの初期化
    let (mut black, mut white) = core::init_board();
    let mut black_turn: bool = true;

    // お互いに置けなくなるまで繰り返し
    loop {
        // 現在の盤面を表示
        cl::display_board(&black, &white, &black_turn);

        // ゲームの終了判定
        if core::is_end(&black, &white) {
            println!("ゲームを終了します。");
            break;
        }

        // スキップ判定
        if core::skip_turn(&black, &white, &black_turn) {
            println!("置けないのでスキップします！");
            black_turn = !black_turn;
            continue;
        }

        // 置く場所の決定
        let pos_n: i32 = cl::input_pos_n();

        // 範囲外の入力検証
        if !(0 <= pos_n && pos_n < 64) {
            println!("範囲外の入力が行われました！");
            continue;
        }

        // 着手可能かどうかの検証
        if !core::enable_put_stone(&black, &white, &pos_n, &black_turn) {
            println!("入力された場所に着手できません！");
            continue;
        }

        // 石を置く
        let (new_black, new_white) = core::put_stone(&black, &white, &pos_n, &black_turn);
        black = new_black;
        white = new_white;

        // 手番の交代
        black_turn = !black_turn;
    }
}
