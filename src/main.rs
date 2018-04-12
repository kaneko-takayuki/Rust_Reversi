extern crate reversi;

use reversi::reversi::core;
use reversi::util::cl;

fn main() {
    let (black, white) = core::init_board();
    cl::display_board(&black, &white);

}
