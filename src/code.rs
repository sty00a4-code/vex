use std::{collections::HashMap, rc::Rc};

use parse_pos::Position;

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
    Str {
        dst: Register,
        addr: usize,
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
    pub positions: HashMap<usize, Position>,
    pub strings: Vec<String>,
    pub refs: Vec<Rc<Self>>,
    pub registers: Register,
}
impl Code {
    pub fn write(&mut self, bytecode: ByteCode) -> Address {
        let addr = self.bytecodes.len();
        self.bytecodes.push(bytecode);
        addr as Address
    }
    pub fn write_pos(&mut self, bytecode: ByteCode, pos: Position) -> Address {
        let addr = self.write(bytecode);
        self.positions.insert(addr, pos);
        addr
    }
    pub fn none(&mut self) -> Address {
        let addr = self.bytecodes.len();
        self.bytecodes.push(ByteCode::default());
        addr as Address
    }
    pub fn overwrite(&mut self, addr: Address, bytecode: ByteCode) {
        self.bytecodes[addr] = bytecode;
    }
    pub fn overwrite_pos(&mut self, addr: Address, bytecode: ByteCode, pos: Position) {
        self.overwrite(addr, bytecode);
        self.positions.insert(addr, pos);
    }
    pub fn new_string(&mut self, string: String) -> usize {
        let addr = self.strings.len();
        self.strings.push(string);
        addr
    }
    pub fn new_ref(&mut self, code: &Rc<Code>) -> usize {
        let addr = self.refs.len();
        self.refs.push(Rc::clone(code));
        addr
    }
}
