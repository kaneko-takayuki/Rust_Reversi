extern crate reversi;

use reversi::reversi::core;
use reversi::util::cl;

fn main() {
    let (black, white) = core::init_board();
    let put_board: u64 = core::get_put_board(&black, &white);
    cl::display_board(&black, &white, &put_board);
}
