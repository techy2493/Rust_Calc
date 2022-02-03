use std::str::FromStr;

use crate::{operation_group::OperationGroup, operation_group::OperationType};

use super::{math::MathAble};


pub trait ProcessableNumber: ToString + FromStr + Copy + MathAble {}
impl<T:ToString + FromStr + Copy + MathAble> ProcessableNumber for T {}

pub trait Processor<T, O>  where
T: Processor<T, O>,
O: ProcessableNumber
{
    fn process(group: &OperationGroup<T, O>) -> Result<O, String>;
    fn get_valid_operations() -> Vec<OperationType>;
    fn get_valid_operations_string() -> String;
    fn match_operation(op_string: String) -> Result<OperationType, String>;
}

// impl<T, O> Default for dyn Processor<T, O> where
// T: Processor<T, O>,
// O: ProcessableNumber 
// {
//     fn default() -> Self {
//         Self::new()
//     }
// }
// impl<T, O> Default for Processor<T, O>   where
// T: Processor<T, O>,
// O: ProcessableNumber
// {
//     fn default() -> Self {
//         OperationGroup::<T, O> {
//             operations: Vec::<Operation<O>>::new(),
//             processor: T::new()
//         }
//     }
// }