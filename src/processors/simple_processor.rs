use crate::traits::processor::ProcessableNumber;
use crate::traits::{newable::Newable, processor::Processor};
use crate::operation_group::{OperationGroup, OperationType};

pub struct SimpleProcessor<T: ProcessableNumber > {
    _last_value: T
}

impl<T: ProcessableNumber> Newable for SimpleProcessor<T> {
    fn new() -> Self {
        return SimpleProcessor::<T> { 
            _last_value: match "0".parse::<T>() {
                Ok(val) => val,
                Err(_) => panic!("An error occured instantiating the calculator. E1")
            }
        }
    }
}

impl<T: ProcessableNumber>  Processor<SimpleProcessor<T>, T> for SimpleProcessor<T> {
    fn process(self: &Self, group: &OperationGroup<SimpleProcessor<T>, T>) -> Result<T, String> {
        let mut val: T =  match "0".parse::<T>() {
            Ok(val) => val,
            Err(_) => panic!("An error occured while calculating. E2")
        };
        for (i, op) in group.get_operations().iter().enumerate() {
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

    fn match_operation(op_string: String) -> Result<OperationType, ()> {
        match op_string.as_str() {
            "+" => Ok(OperationType::Add),
            "-" => Ok(OperationType::Substract),
            "/" => Ok(OperationType::Divide),
            "*" => Ok(OperationType::Multiply),
            _ => Err(())
        }
    }

    fn get_valid_operations_string() -> String {
        String::from("+, -, *, and /")
    }
}

