mod types;
mod cell;
mod board;
use board::*;
fn main() {
    let mut board = Board::new(10,10).init();
    board.set_boardstate(1,0,true);
    board.set_boardstate(1,1,true);
    board.set_boardstate(1,2,true);
    // board.show_board();
    for i in 0..10{
        let mut old_board = board.clone();
        board.refresh();
//        old_board.show_board();
        board.show_board();
        if old_board == board{
            println!("一致しました");
            break;
        }
    }
    let mut clone = board.clone();
/*     clone.set_boardstate(4,4, true);
    println!("clone board!");;
    clone.show_board();
    println!("origin board!");;
    board.show_board();
    board.show_refcnt_board(); */
}
