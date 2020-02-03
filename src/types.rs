use std::rc::Rc;
use std::cell::RefCell;
use crate::cell::Cell;

pub type ReferencedCell=Rc<RefCell<Cell>>;