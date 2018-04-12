///
/// 盤面を表示する
///
pub fn display_board(black: &u64, white: &u64) {
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
                return ' '
            })
            .collect();

        println!("{}|{}|{}|{}|{}|{}|{}|{}|{}|", i + 1, row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7]);
    }
}