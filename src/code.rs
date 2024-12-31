use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(u8)]
pub enum ByteCode {
    #[default]
    None,

    Jump {
        addr: Address,
    },
    JumpIf {
        not: bool,
        src: Register,
        addr: Address,
    },
    JumpIfSome {
        not: bool,
        src: Register,
        addr: Address,
    },

    Copy {
        dst: Register,
        src: Register,
    },
    Global {
        dst: Register,
        addr: usize,
    },
    SetGlobal {
        addr: usize,
        src: Register,
    },
    Field {
        dst: Register,
        head: Register,
        field: Register,
    },
    SetField {
        head: Register,
        field: Register,
        src: Register,
    },

    Call {
        dst: Register,
        start: Register,
        amount: Register,
    },
    Return {
        src: Option<Register>,
    },

    Nil {
        dst: Register,
    },
    Int {
        dst: Register,
        value: i64,
    },
    Float {
        dst: Register,
        value: f64,
    },
    Bool {
        dst: Register,
        value: bool,
    },
    Char {
        dst: Register,
        value: char,
    },
    Fn {
        dst: Register,
        addr: usize,
    },
    Tuple {
        dst: Register,
        start: Register,
        amount: Register,
    },
    Vec {
        dst: Register,
        start: Register,
        amount: Register,
    },
    Map {
        dst: Register,
    },
    Class {
        dst: Register,
        name_addr: usize,
        fields_start: Register,
        fields_amount: Register,
        methods_start: Register,
        methods_end: Register,
        meta_methods_start: Register,
        meta_methods_ed: Register,
    },
    Box {
        reg: Register,
    },
}
pub type Register = u16;
pub type Address = usize;

#[derive(Debug, Clone)]
pub struct Code {
    pub bytecodes: Vec<ByteCode>,
    pub strings: Vec<String>,
    pub refs: Vec<Rc<Self>>,
    pub registers: Register,
}
