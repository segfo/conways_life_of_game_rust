use crate::types::ReferencedCell;

const AROUND_CELLS:usize = 8;


#[derive(Debug,Clone,PartialEq)]
pub struct Point(usize,usize);
impl Point{
    pub fn new(x:usize,y:usize)->Self{
        Point(x,y)
    }
    #[allow(dead_code)]
    pub fn x(&self)->usize{
        self.0
    }
    #[allow(dead_code)]
    pub fn y(&self)->usize{
        self.1
    }
}

// 各セルを表現する構造体
#[derive(Debug)]
pub struct Cell{
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

// 参照はコピーしない。
impl Clone for Cell{
    fn clone(&self)->Cell{
        let mut cell = Cell::new(self.pt.clone());
        cell.survive = self.survive;
        cell.around_survivers_count = self.around_survivers_count;
        cell
    }
}

impl PartialEq for Cell{
    fn eq(&self,rhs: &Cell)->bool{
        rhs.survive == self.survive
    }
}

impl Cell{
    pub fn new(pt:Point)->Self{
        Cell{
            survive:false,
            around_cells:Vec::new(),
            around_survivers_count:0,
            pt:pt
        }
    }
    // 周りの誰かが生まれた時に、そのインスタンスから呼ばれる
    fn notify_born(&mut self){
        self.around_survivers_count+=1;
    }
    // 周りの誰かが死んだときに、そのインスタンスから呼ばれる
    fn notify_kill(&mut self){
        self.around_survivers_count-=1;
    }
    // 周りのセルに生存状況を通知する
    pub fn around_cells_notify(&mut self,survive:bool){
        for y in 0..3{
            for x in 0..3{
                // 自分自身に対する「通知」はしない。
                // 周りのセルから報告してもらう。
                if x==y && x == 1{continue;}
                // 周辺のセルに対して計算する。
                let mut around_cell = self.around_cells[x+y*3].borrow_mut();
                if survive{
                    around_cell.notify_born();
                }else{
                    around_cell.notify_kill();
                }
            }
        }
        self.survive = survive;
    }

    // 自身を死んだと判定するメソッド
    pub fn kill(&mut self){
        assert_ne!(self.around_cells.len(), 0,
        "(x,y)=({},{}):周りにセルがありません（番兵用の外周セルがselfに指定されたようです。）
        指定したセルが外周セルでない場合はリファレンスを設定しなおしてください
        この挙動はソフト的なバグである可能性が高いです。",self.pt.0,self.pt.1);
        // すでに死んでいるなら、設定の必要がないため終了する。
        if self.survive == false {return;}
        // 周りのセルに自分は死んだことを報告する。
        self.around_cells_notify(false);
    }
    // 自身を誕生と判定するメソッド
    pub fn born(&mut self){
        assert_ne!(self.around_cells.len(), 0,
        "(x,y)=({},{}):周りにセルがありません（番兵用の外周セルがselfに指定されたようです。）
        指定したセルが外周セルでない場合はリファレンスを設定しなおしてください
        この挙動はソフト的なバグである可能性が高いです。",self.pt.0,self.pt.1);
        // すでに生きているなら、設定の必要がないため終了する。
        if self.survive == true {return;}
        // 周りのセルに自分は生きていることを報告する。
        self.around_cells_notify(true);
    }
    pub fn get_survive(&self)->bool{
        self.survive
    }
    pub fn get_around_survivers_count(&self)->u8{
        self.around_survivers_count
    }
    #[allow(dead_code)]
    pub fn get_point(&self)->Point{
        self.pt.clone()
    }
    pub fn set_arround_cells(&mut self,cell:ReferencedCell){
        if self.around_cells.len() <= AROUND_CELLS{
            self.around_cells.push(cell)
        }
    }
    #[allow(dead_code)]
    pub fn get_aroundcell_refcnts(&self)->usize{
        self.around_cells.len()
    }
}
