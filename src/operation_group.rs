use crate::traits::{processor::{Processor, ProcessableNumber}};
use std::marker::PhantomData;

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
pub struct Operation<O> where
O: ProcessableNumber
{
    pub value: O,
    pub value_text: String,
    pub operation_type: OperationType
}

pub struct OperationGroup<T, O> where
T: Processor<T, O>,
O: ProcessableNumber
{
    operations: Vec<Operation<O>>,
    processor_data_type: PhantomData<T>
}
impl<T, O> Default for OperationGroup<T, O> where
T: Processor<T, O>,
O: ProcessableNumber
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, O> OperationGroup<T, O> where
T: Processor<T, O>,
O: ProcessableNumber {
    pub fn new() -> Self {
        OperationGroup::<T, O> {
            operations: Vec::<Operation<O>>::new(),
            processor_data_type: PhantomData
        }
    }

    pub fn add_operation(&mut self, op: Operation<O>) {
        self.operations.push(op);
    }

    pub fn calculate(&self) -> Result<O, String>
    {
        T::process(self)
    }

    pub fn get_operations(&self) -> &Vec<Operation<O>> {
        &self.operations
    }
}