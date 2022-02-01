use std::{rc::Rc, cell::RefCell};
use crate::traits::{processor::{Processor, ProcessableNumber}};

#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum OperationType {
    Invalid,
    Add,
    Substract,
    Divide,
    Multiply
}
#[derive(Clone)]
pub struct Operation<O: ProcessableNumber> {
    pub value: O,
    pub value_text: String,
    pub operation_type: OperationType
}

pub struct OperationGroup<T: Processor<T, O>, O: ProcessableNumber> {
    operations: RefCell<Vec<Operation<O>>>,
    processor: T
}

impl<T: Processor<T, O>, O: ProcessableNumber> OperationGroup<T, O> {
    pub fn new() -> Rc<Self> {
        let this = Rc::new(OperationGroup::<T, O> {
            operations: RefCell::new(Vec::<Operation<O>>::new()),
            processor: T::new()
        });
        this
    }

    pub fn add_operation(self: &Self, op: Operation<O>) {
        self.operations.borrow_mut().push(op);
    }

    pub fn calculate(self: &Self) -> Result<O, String>
    {
        self.processor.process(self)
    }

    pub fn get_operations(self: &Self) -> Vec<Operation<O>> {
        self.operations.borrow().clone()
    }
}