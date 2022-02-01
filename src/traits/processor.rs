use std::str::FromStr;

use crate::{operation_group::OperationGroup, operation_group::OperationType};

use super::{newable::Newable, math::MathAble};


pub trait ProcessableNumber: ToString + FromStr + Copy + MathAble {}
impl<T:ToString + FromStr + Copy + MathAble> ProcessableNumber for T {}

pub trait Processor<T:Processor<T, O>, O: ProcessableNumber> : Newable {
    fn process(self: &Self, group: &OperationGroup<T, O>) -> Result<O, String>;
    fn get_valid_operations() -> Vec<OperationType>;
    fn get_valid_operations_string() -> String;
    fn match_operation(op_string: String) -> Result<OperationType, ()>;
}