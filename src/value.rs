use crate::{code::Code, vm::Vm};
use std::{cell::RefCell, collections::HashMap, error::Error, fmt::Debug, rc::Rc};

pub type Pointer<T> = Rc<RefCell<T>>;
#[derive(Clone, Default)]
pub enum Value {
    #[default]
    Nil,
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    Str(String),
    Fn(FnKind),
    Coroutine(Pointer<Vm>),
    Tuple(Pointer<[Self]>),
    Vec(Pointer<Vec<Self>>),
    Map(Pointer<HashMap<String, Self>>),
    Class(Pointer<Class>),
    Object(Pointer<Object>),
    Iter(Pointer<dyn Iterator<Item = Value>>),
    Box(Pointer<Self>),
}
#[derive(Clone)]
pub enum FnKind {
    Fn(Rc<Code>),
    NativeFn(NativeFn),
}
pub type NativeFn = fn(&mut Vm, &[Value]) -> Result<Value, Box<dyn Error>>;
pub struct Class {
    pub name: String,
    pub fields: Box<[String]>,
    pub methods: Box<[FnKind]>,
    pub meta_methods: Box<[FnKind]>,
}
pub struct Object {
    pub class: Pointer<Class>,
    pub values: Pointer<[Value]>,
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Int(v) => write!(f, "{v:?}"),
            Value::Float(v) => write!(f, "{v:?}"),
            Value::Bool(v) => write!(f, "{v:?}"),
            Value::Char(v) => write!(f, "{v:?}"),
            Value::Str(v) => write!(f, "{v:?}"),
            Value::Fn(FnKind::Fn(ptr)) => write!(f, "fn:{:08x?}", Rc::as_ptr(ptr)),
            Value::Fn(FnKind::NativeFn(func)) => write!(f, "fn:{:08x?}", func as *const NativeFn),
            Value::Coroutine(ptr) => write!(f, "coroutine:{:08x?}", Rc::as_ptr(ptr)),
            Value::Tuple(ptr) => write!(
                f,
                "({})",
                ptr.borrow()
                    .iter()
                    .map(|v| format!("{v:?}"))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Value::Vec(ptr) => write!(f, "{:?}", ptr.borrow()),
            Value::Map(ptr) => write!(
                f,
                "{{ {} }}",
                ptr.borrow()
                    .iter()
                    .map(|(k, v)| format!("{k:?} = {v:?}"))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Value::Class(ptr) => write!(f, "class<{}>:{:08x?}", ptr.borrow().name, Rc::as_ptr(ptr)),
            Value::Object(ptr) => write!(
                f,
                "{}:{:08x?}",
                ptr.borrow().class.borrow().name,
                Rc::as_ptr(ptr)
            ),
            Value::Iter(ptr) => write!(f, "iter:{:08x?}", Rc::as_ptr(ptr)),
            Value::Box(ptr) => write!(f, "box({:?})", ptr.borrow()),
        }
    }
}
