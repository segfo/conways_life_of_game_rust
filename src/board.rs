use std::rc::Rc;
use std::cell::RefCell;
use crate::types::*;
use crate::cell::*;
// ボードを表現する実装
#[derive(Debug)]
pub struct Board{
    inner:Vec<ReferencedCell>,
    width:usize,
    height:usize,
}

impl Clone for Board{
    fn clone(&self)->Board{
        let mut cloned_board = Board::new(self.width-2,self.height-2);
        for i in 0..self.inner.len(){
            cloned_board.inner.push(
                Rc::new(RefCell::new(
                    // Cloneするとリファレンスの設定が解除される。
                    // あとで設定しなおす。
                    self.inner[i].borrow_mut().clone()
                ))
            );
        }
        // 周辺セルへのリファレンスを再設定する。
        cloned_board.set_ref();
        cloned_board
    }
}

impl PartialEq for Board{
    fn eq(&self,rhs: &Board)->bool{
        let size = self.width == rhs.width && self.height == rhs.height;
        if size{
            for cell in self.inner.iter().zip(rhs.inner.iter()){
                if cell.0 != cell.1{
                    return false;
                }
            }
            true
        }else{
            false
        }
    }
}

impl Board{
    pub fn set_boardstate(&mut self,x:usize,y:usize,state_alive:bool){
        let x = x + 1;
        let y = y + 1;
        // 範囲外なら何もせずリターン
        if x >= self.width-1 || y >= self.height-1{
            return;
        }
        let mut cell = self.inner[x+y*self.width].borrow_mut();
        if state_alive{
            cell.born();
        }else{
            cell.kill();
        }
    }
    pub fn new(width:usize,height:usize)->Self{
        // 番兵セル分を縦横2レーンずつ増やす
        let width = width + 2;
        let height = height + 2;
        Board{
            inner:Vec::new(),
            width:width,
            height:height
        }
    }
    pub fn init(mut self)->Self{
        let width = self.width;
        let height = self.height;
        for y in 0..height{
            for x in 0..width{
                // ボードの初期化を行う。
                self.inner.push(
                    Rc::new(RefCell::new(Cell::new(Point::new(x,y))))
                );
            }
        }
        self.set_ref();
        self
    }
    fn set_ref(&mut self){
        let width = self.width;
        let height = self.height;
        // ボード全体を走査する。
        // 1,1～height-1,width-1 に対して、refを設定する。（これ以外の部分にアクセスはできない）
        for y in 1..height-1{
            for x in 1..width-1{
                // カレントの前後左右1マスずつのマスに対する参照を、「周辺8マス参照リスト」に登録する。
                // ループ上は自分自身も追加しているが自分自身は参照できない。（二重借用になるため。）
                for i in 0..3{
                    for j in 0..3{
                        self.inner[x+y*width].borrow_mut()
                            .set_arround_cells(
                                self.inner[(x-1+j)+(y+1-i)*width].clone()
                            );
                    }
                }
            }
        }
    }
    // リフレッシュする。
    pub fn refresh(&mut self){
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
                match o.get_around_survivers_count(){
                    4..=8|0..=1=>{o.kill();}, // 過密/過疎により死亡する。
                    3=>{
                        o.born();
                    },
                    2=>{},  // 生存する
                    _=>{panic!("「周辺マスの個数」が0-8の範囲を超えています。");}
                } 
            }
        }
    }
    pub fn show_board(&mut self){
        println!("--------alive/dead(around_survivers_count)--------");
        for y in 0..self.height{
            for x in 0..self.width{
                let cell = self.inner[x+y*self.width].borrow();
                let bit = if cell.get_survive(){1}else{0};
                print!("{}({}) ", bit,cell.get_around_survivers_count());
            }
            println!();
        }
    }
    pub fn show_refcnt_board(&mut self){
        // 
        println!("--------RefCnt(around_survivers_count)--------");
        for y in 0..self.height{
            for x in 0..self.width{
                let cell = self.inner[x+y*self.width].borrow();
                let cells = cell.get_aroundcell_refcnts();
                print!("{}({}) ", cells,cell.get_around_survivers_count());
            }
            println!();
        }
    }
}
