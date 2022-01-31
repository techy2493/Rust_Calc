use std::{rc::Rc, cell::RefCell};
use crate::traits::{newable::Newable, processor::Processor};
pub enum OperationType {
    Invalid,
    Add,
    Substract
}

pub struct Operation {
    pub value: i32,
    pub value_text: String,
    pub operation_type: OperationType
}

pub struct OperationGroup<T: Newable + Processor> {
    operations: RefCell<Vec<Operation>>,
    processor: T
}

impl<T: Newable + Processor> OperationGroup<T> {
    pub fn new() -> Rc<Self> {
        let this = Rc::new(OperationGroup::<T> {
            operations: RefCell::new(Vec::<Operation>::new()),
            processor: T::new()
        });
        this
    }

    pub fn add_operation(self: &Self, op: Operation) {
        self.operations.borrow_mut().push(op);
    }

    pub fn calculate(self: &Self) -> Result<i32, String>
    {
        self.processor.process()
    }
}