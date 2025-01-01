use crate::{
    code::{Address, Code, Register},
    value::{FnKind, Pointer, Value},
};
use std::{cell::RefCell, collections::HashMap, error::Error, rc::Rc};

#[derive(Debug, Default)]
pub struct Vm {
    pub call_stack: Vec<Call>,
    pub globlas: Pointer<HashMap<String, Pointer<Value>>>,
}
#[derive(Debug)]
pub struct Call {
    pub code: Rc<Code>,
    pub ip: usize,
    pub stack: Vec<Pointer<Value>>,
    pub dst: Register,
}

impl Vm {
    pub fn init(globals: HashMap<String, Pointer<Value>>) -> Self {
        Self {
            globlas: Rc::new(RefCell::new(globals)),
            ..Default::default()
        }
    }
    pub fn global(&self, name: &str) -> Option<Pointer<Value>> {
        self.globlas.borrow().get(name).cloned()
    }
    pub fn set_global(&self, name: String, value: Value) -> Option<Pointer<Value>> {
        self.globlas
            .borrow_mut()
            .insert(name, Rc::new(RefCell::new(value)))
    }
    pub fn get_call(&self) -> &Call {
        self.call_stack.last().unwrap()
    }
    pub fn get_call_mut(&mut self) -> &mut Call {
        self.call_stack.last_mut().unwrap()
    }
    pub fn call(
        &mut self,
        f: &FnKind,
        args: Box<[Value]>,
        dst: Register,
    ) -> Result<(), Box<dyn Error>> {
        match f {
            FnKind::Fn(code) => {
                let mut stack: Vec<Pointer<Value>> = Vec::with_capacity(code.registers as usize);
                stack.fill_with(Rc::default);
                for (ptr, arg) in stack.iter_mut().zip(args) {
                    *ptr.borrow_mut() = arg;
                }
                self.call_stack.push(Call {
                    code: Rc::clone(code),
                    ip: 0,
                    stack,
                    dst,
                });
                Ok(())
            }
            FnKind::NativeFn(f) => {
                *self.register(dst).borrow_mut() = f(self, args.as_ref())?;
                Ok(())
            }
        }
    }
    pub fn return_call(&mut self, src: Option<Register>) -> Result<Value, Box<dyn Error>> {
        let Call { dst, mut stack, .. } = self.call_stack.pop().unwrap();
        let value = if let Some(src) = src {
            stack.remove(src as usize).borrow().clone()
        } else {
            Value::default()
        };
        *self.register(dst).borrow_mut() = value.clone();
        Ok(value)
    }
    pub fn register(&self, reg: Register) -> &Pointer<Value> {
        self.get_call().stack.get(reg as usize).unwrap()
    }
    pub fn registers(&self, start: Register, amount: Register) -> &[Pointer<Value>] {
        self.get_call()
            .stack
            .get((start as usize)..(start as usize + amount as usize))
            .unwrap()
    }
    pub fn string(&self, addr: Address) -> &String {
        self.get_call().code.strings.get(addr).unwrap()
    }
    pub fn code(&self, addr: Address) -> &Rc<Code> {
        self.get_call().code.refs.get(addr).unwrap()
    }
    pub fn copy(&self, dst: Register, src: Register) {
        *self.register(dst).borrow_mut() = self.register(src).borrow().clone();
    }
    pub fn jump(&mut self, addr: Address) {
        self.get_call_mut().ip = addr;
    }
}
