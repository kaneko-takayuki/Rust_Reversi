use std::io;
use reversi::core;

///
/// 盤面を表示する
///
pub fn display_board(black: &u64, white: &u64, black_turn: &bool) {
    // 着手可能位置を求める
    let (me, op) = if *black_turn {
        (black, white)
    } else {
        (white, black)
    };
    let put_board: u64 = core::get_put_board(me, op);

    println!(" |A|B|C|D|E|F|G|H|");
    println!("- - - - - - - - - ");

    // 各マスについて、黒なら'o'、白なら'x'で置き換えて表示する
    for i in 0..8 {
        let row: Vec<char> = (0..8)
            .into_iter()
            .map(|j: i32| -> char {
                let pos = 1 << (i * 8 + j);
                if black & pos != 0 {return 'o';}
                if white & pos != 0 {return 'x';}
                if put_board & pos != 0 {return '.'}
                return ' '
            })
            .collect();

        println!("{}|{}|{}|{}|{}|{}|{}|{}|{}|", i + 1, row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7]);
    }

    // 改行
    println!();
}

///
/// 標準入力から石を置く位置を受け付ける
///
pub fn input_pos_n() -> i32 {
    println!("どこに置く？: ");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            if let (Some(col), Some(row)) = (input.chars().nth(0), input.chars().nth(1)) {
                convert(&col) + (convert(&row) * 8)
            } else {
                -1
            }
        }
        Err(error) => {
            println!("error: {}", error);
            -1
        }
    }
}

///
/// 文字からインデックスを求める
///
fn convert(c: &char) -> i32 {
    match *c {
        'A' | 'a' | '1' => 0,
        'B' | 'b' | '2' => 1,
        'C' | 'c' | '3' => 2,
        'D' | 'd' | '4' => 3,
        'E' | 'e' | '5' => 4,
        'F' | 'f' | '6' => 5,
        'G' | 'g' | '7' => 6,
        'H' | 'h' | '8' => 7,
        _ => 100
    }
}
