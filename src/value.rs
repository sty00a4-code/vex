use crate::{code::Code, vm::Vm};
use std::{
    cell::RefCell,
    collections::HashMap,
    error::Error,
    fmt::{Debug, Display},
    rc::Rc,
};

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
        }
    }
}
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Char(v) => write!(f, "{v}"),
            Self::Str(v) => write!(f, "{v}"),
            _ => Debug::fmt(self, f),
        }
    }
}
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nil, Self::Nil) => true,
            (Self::Int(left), Self::Int(right)) => left == right,
            (Self::Float(left), Self::Float(right)) => left == right,
            (Self::Bool(left), Self::Bool(right)) => left == right,
            (Self::Char(left), Self::Char(right)) => left == right,
            (Self::Str(left), Self::Str(right)) => left == right,
            (Self::Fn(FnKind::Fn(left)), Self::Fn(FnKind::Fn(right))) => {
                std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right))
            }
            (Self::Fn(FnKind::NativeFn(left)), Self::Fn(FnKind::NativeFn(right))) => {
                std::ptr::addr_eq(left as *const NativeFn, right as *const NativeFn)
            }
            (Self::Coroutine(left), Self::Coroutine(right)) => {
                std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right))
            }
            (Self::Tuple(left), Self::Tuple(right)) => left
                .borrow()
                .iter()
                .zip(right.borrow().iter())
                .all(|(left, right)| left == right),
            (Self::Vec(left), Self::Vec(right)) => left
                .borrow()
                .iter()
                .zip(right.borrow().iter())
                .all(|(left, right)| left == right),
            (Self::Map(left), Self::Map(right)) => {
                for (key, left) in left.borrow().iter() {
                    if left != right.borrow().get(key).unwrap_or(&Value::default()) {
                        return false;
                    }
                }
                true
            }
            (Self::Class(left), Self::Class(right)) => {
                std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right))
            }
            (Self::Object(left), Self::Object(right)) => {
                std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right))
            }
            (Self::Iter(left), Self::Iter(right)) => {
                std::ptr::addr_eq(Rc::as_ptr(left), Rc::as_ptr(right))
            }
            _ => false,
        }
    }
}
