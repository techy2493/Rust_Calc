use crate::traits::processor::ProcessableNumber;
use crate::traits::{processor::Processor};
use crate::operation_group::{OperationGroup, OperationType};
use std::marker::PhantomData;

pub struct SimpleProcessor<T> where
T: ProcessableNumber
{
    processor_data_type: PhantomData<T>
}

impl<T>  Processor<SimpleProcessor<T>, T> for SimpleProcessor<T>  where 
T: ProcessableNumber
{
    fn process(group: &OperationGroup<SimpleProcessor<T>, T>) -> Result<T, String> {
        let mut val: T =  match "0".parse::<T>() {
            Ok(val) => val,
            Err(_) => panic!("An error occured while calculating. E2")
        };
        for (_, op) in group.get_operations().iter().enumerate() {
            match op.operation_type {
                OperationType::Add => val += op.value,
                OperationType::Substract => val -= op.value,
                OperationType::Divide => val /= op.value,
                OperationType::Multiply => val *= op.value,
                _ => return Err(String::from("Invalid operaiton type encountered while calculating result."))
            };
        };
        Ok(val)
    }

    fn get_valid_operations() -> Vec<OperationType> {
        vec![OperationType::Add, OperationType::Substract]
    }

    fn match_operation(op_string: String) -> Result<OperationType, String> {
        match op_string.as_str() {
            "+" => Ok(OperationType::Add),
            "-" => Ok(OperationType::Substract),
            "/" => Ok(OperationType::Divide),
            "*" => Ok(OperationType::Multiply),
            _ => Err(format!("Could not handle operation type {}", op_string))
        }
    }

    fn get_valid_operations_string() -> String {
        String::from("+, -, *, and /")
    }


}

