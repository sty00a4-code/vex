use crate::{
    code::{Code, Register},
    value::{Pointer, Value},
};
use std::{collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct Vm {
    pub call_stack: Vec<Call>,
    pub globlas: Pointer<HashMap<String, Pointer<Value>>>,
}
#[derive(Debug)]
pub struct Call {
    pub code: Rc<Code>,
    pub stack: Vec<Pointer<Value>>,
    pub dst: Register,
}
