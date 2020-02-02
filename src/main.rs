// (9+2)x(9+2)=121 
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Point(usize,usize);
impl Point{
    fn new(x:usize,y:usize)->Self{
        Point(x,y)
    }
}

// 各セルを表現する構造体
#[derive(Debug)]
struct Cell{
    // true: 生存
    // false: 死亡
    survive:bool,
    // 周辺の生存マスのカウント
    around_survivers_count:u8,
    // 周りのセル
    around_cells:Vec<ReferencedCell>,
    // 自身のセル位置,あると便利。無くてもいいけどとりあえず。
    pt:Point
}
type ReferencedCell=Rc<RefCell<Cell>>;

impl Cell{
    fn new(pt:Point)->Self{
        Cell{
            survive:false,
            around_cells:Vec::new(),
            around_survivers_count:0,
            pt:pt
        }
    }
    fn cell_search(&mut self,op:fn(u8)->u8,switch:bool){
        for y in 0..3{
            for x in 0..3{
                // 自分自身はスキップする / 2重借用になるので取ろうとするとpanicになる。
                if x==y && x == 1{continue;}
                let v = self.around_cells[x+y*3].borrow_mut().around_survivers_count;
                // 周辺のセルに対して計算する。
                self.around_cells[x+y*3].borrow_mut().around_survivers_count = op(v);
            }
        }
        self.survive = switch;
    }

    // 自身を死んだと判定するメソッド
    fn kill(&mut self){
        assert_ne!(self.around_cells.len(), 0,"(x,y)=({},{}):周りにセルがありません（番兵用の外周セルがselfに指定されたようです。）\nこの挙動はソフト的なバグである可能性が高いです。",self.pt.0,self.pt.1);
        // すでに死んでいるなら、設定の必要がないため終了する。
        if self.survive == false {return;}
        // 周りのセルからaround_survivers_countを1引いて、自身のセルをdead(false)にする。
        self.cell_search(|v|{v-1},false);
    }
    // 自身を誕生と判定するメソッド
    fn born(&mut self){
        assert_ne!(self.around_cells.len(), 0,"(x,y)=({},{}):周りにセルがありません（番兵用の外周セルがselfに指定されたようです。）\nこの挙動はソフト的なバグである可能性が高いです。",self.pt.0,self.pt.1);
        // すでに生きているなら、設定の必要がないため終了する。
        if self.survive == true {return;}
        // 周りのセルにaround_survivers_countを1足して、自身のセルをalive(true)にする。
        self.cell_search(|v|{v+1},true);
    }
}

// ボードを表現する実装
#[derive(Debug)]
struct Board{
    inner:Vec<ReferencedCell>,
    width:usize,
    height:usize,
}

impl Board{
    fn set_boardstate(&mut self,x:usize,y:usize){
        self.inner[x+y*self.width].borrow_mut().born();
    }
    fn new(width:usize,height:usize)->Self{
        let mut s = Board{
            inner:Vec::new(),
            width:width,
            height:height
        };
        for y in 0..height{
            for x in 0..width{
                // ボードの初期化を行う。
                s.inner.push(
                    Rc::new(RefCell::new(Cell::new(Point::new(x,y))))
                );
            }
        }
        // ボード全体を走査する。
        // 1,1～height-1,width-1 に対して、refを設定する。（これ以外の部分にアクセスはできない）
        for y in 1..height-1{
            for x in 1..width-1{
                // カレントの前後左右1マスずつのマスに対する参照を、「周辺8マス参照リスト」に登録する。
                // ループ上は自分自身も追加しているが自分自身は参照できない。（二重借用になるため。）
                for i in 0..3{
                    for j in 0..3{
                        s.inner[x+y*width].borrow_mut()
                            .around_cells.push(
                                s.inner[(x-1+j)+(y+1-i)*width].clone()
                            );
                    }
                }
            }
        }
        s
    }
    // リフレッシュする。
    fn refresh(&mut self){
        // Cell::alive(); は
        // 0 < x < width-1
        // 0 < y < height-1
        // の範囲でのみアクセスできる（この範囲外はセルの参照を持たないため、Out boundになる。）
        // 詳細は初期化ルーチン(Board::newのロジックを参考にすること)
        for y in 1..self.height-1{
            for x in 1..self.width-1{
                // 3 == 生存
                let mut o = self.inner[x+y*self.width].borrow_mut();
                // 3個ちょうどなら誕生となる。
                match o.around_survivers_count{
                    4..=8|0..=1=>{o.kill();}, // 過密/過疎により死亡する。
                    3=>{
                        println!("x:{} y:{} / alive.",o.pt.0,o.pt.1);
                        o.born();
                    },
                    2=>{},  // 生存する
                    _=>{panic!("「周辺マスの個数」が0-8の範囲を超えています。");}
                } 
            }
        }
    }
    fn show_board(&mut self){
        println!("--------alive/dead(around_survivers_count)--------");
        for y in 0..self.height{
            for x in 0..self.width{
                let cell = self.inner[x+y*self.width].borrow();
                let bit = if cell.survive{1}else{0};
                print!("{}({}) ", bit,cell.around_survivers_count);
            }
            println!();
        }
    }
    fn show_refcnt_board(&mut self){
        // 
        println!("--------RefCnt(around_survivers_count)--------");
        for y in 0..self.height{
            for x in 0..self.width{
                let cell = self.inner[x+y*self.width].borrow();
                let cells = cell.around_cells.len();
                print!("{}({}) ", cells,cell.around_survivers_count);
            }
            println!();
        }
    }
}


fn main() {
    let mut board = Board::new(9+2,9+2);
    board.set_boardstate(1,1);
    board.set_boardstate(2,1);
    board.set_boardstate(1,2);
    // board.set_boardstate(3,3);
    board.show_board();
    board.refresh();
    println!("------");

    board.show_board();
    println!("------");
    board.show_refcnt_board();
}
