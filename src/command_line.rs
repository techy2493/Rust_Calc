use cancellation::{CancellationToken, OperationCanceled};
use crate::operation_group;
use crate::traits::processor::{Processor, ProcessableNumber};

pub fn get_number<T: ProcessableNumber>() -> (T, String) {
    let mut text = String::from("Please Enter A Whole Number");
    loop {
        println!("{}", text);
        let line = read_input();
        let out = line.trim().parse::<T>();
        match out {
            Err(_) => {text = String::from("Try Again! Only whole integers are valid!"); eprintln!("Invalid Input"); continue;}
            Ok(value) => return (value, line)
        }
    }
}

pub fn get_operation<T: Processor<T, O>, O: ProcessableNumber>() -> operation_group::OperationType {
    let mut text = format!("Please Enter an Operation ({})", T::get_valid_operations_string());
    loop {
        println!("{}", text);
        let line = read_input();
        match T::match_operation(line) {
            Ok(op) => break op,
            Err(_) => {text = format!("Try Again! Only {} are valid operators!", T::get_valid_operations_string())}
        }
    }
}

pub fn print_result(value: String) {
    println!("The result of the last operation is: {}", value);
}

pub fn print_calc_error(error: String) {
    eprintln!("An error occured while calculating your result: \n {}", error)
}

pub fn begin_loop<T: Processor<T, O>, O: ProcessableNumber>(ct: &CancellationToken) -> Result<(), OperationCanceled>{
    let op_group = operation_group::OperationGroup::<T, O>::new();
    let loop_result = loop {
        if let Err(_) = ct.result() {
            break Err(OperationCanceled)
        }

        let op = get_operation::<T, O>();
        let (value, value_text) = get_number();
        op_group.add_operation(operation_group::Operation {
            operation_type: op,
            value: value,
            value_text: value_text
        });
        match op_group.calculate() {
            Err(error) => print_calc_error(error),
            Ok(result) => print_result(result.to_string())
        }
    };
    drop(op_group);
    return loop_result;
}
 
fn read_input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    String::from(line.trim())
}