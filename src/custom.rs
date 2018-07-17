use std::cell::RefCell;
use std::rc::Rc;

use super::{Parse, ParseOption, ParseList, ParseCollect, FromCommandLine};
use super::action::Action;
use super::action::{TypedAction, IArgAction, IArgsAction};
use super::action::ParseResult;
use super::action::ParseResult::{Parsed, Error};
use super::action::Action::{Single, Push, Many};

pub struct ParseAction<'a, T: 'a> {
    pub cell: Rc<RefCell<&'a mut T>>,
}

pub struct ParseOptionAction<'a, T: 'a> {
    cell: Rc<RefCell<&'a mut Option<T>>>,
}

pub struct ParseListAction<'a, T: 'a> {
    cell: Rc<RefCell<&'a mut Vec<T>>>,
}

impl<T: FromCommandLine> TypedAction<T> for Parse {
    fn bind<'x>(&self, cell: Rc<RefCell<&'x mut T>>) -> Action<'x> {
        return Single(Box::new(ParseAction { cell: cell }));
    }
}

impl<T: FromCommandLine> TypedAction<Option<T>> for ParseOption {
    fn bind<'x>(&self, cell: Rc<RefCell<&'x mut Option<T>>>) -> Action<'x> {
        return Single(Box::new(ParseOptionAction { cell: cell }));
    }
}

impl<T: FromCommandLine + Clone> TypedAction<Vec<T>> for ParseList {
    fn bind<'x>(&self, cell: Rc<RefCell<&'x mut Vec<T>>>) -> Action<'x> {
        return Many(Box::new(ParseListAction { cell: cell }));
    }
}

// ?AK? how ParseList and ParseCollect differentiated? they both use ParseListAction
impl<T: FromCommandLine + Clone> TypedAction<Vec<T>> for ParseCollect {
    fn bind<'x>(&self, cell: Rc<RefCell<&'x mut Vec<T>>>) -> Action<'x> {
        return Push(Box::new(ParseListAction { cell: cell }))
    }
}

impl<'a, T: FromCommandLine> IArgAction for ParseAction<'a, T> {
    fn parse_arg(&self, arg: &str) -> ParseResult {
        match T::from_argument(arg) {
            Ok(x) => {
                **self.cell.borrow_mut() = x;
                return Parsed;
            }
            Err(error) => {
                return Error(format!("Bad value {:?}: {}", arg, error));
            }
        }
    }
}

impl<'a, T: FromCommandLine> IArgAction for ParseOptionAction<'a, T> {
    fn parse_arg(&self, arg: &str) -> ParseResult {
        match T::from_argument(arg) {
            Ok(x) => {
                **self.cell.borrow_mut() = Some(x);
                return Parsed;
            }
            Err(error) => {
                return Error(format!("Bad value {:?}: {}", arg, error));
            }
        }
    }
}

impl<'a, T: FromCommandLine + Clone> IArgsAction for ParseListAction<'a, T> {
    fn parse_args(&self, args: &[&str]) -> ParseResult {
        let mut result = vec![];
        for arg in args {
            match FromCommandLine::from_argument(*arg) {
                Ok(x) => {
                    result.push(x);
                }
                Err(error) => {
                    return Error(format!("Bad value {:?}: {}", arg, error));
                }
            }
        }
        **self.cell.borrow_mut() = result;
        return Parsed;
    }
}


