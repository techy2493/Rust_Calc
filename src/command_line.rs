use cancellation::{CancellationToken, OperationCanceled};
use crate::operation_group;
use crate::processors::{simple_processor:: SimpleProcessor};

pub fn get_number() -> (i32, String) {
    let mut text = String::from("Please Enter A Whole Number");
    loop {
        println!("{}", text);
        let line = read_input();
        let out = line.trim().parse::<i32>();
        match out {
            Err(_) => {text = String::from("Try Again! Only whole integers are valid!"); eprintln!("Invalid Input"); continue;}
            Ok(value) => return (value, line)
        }
    }
}

pub fn get_operation() -> operation_group::OperationType {
    let mut text = String::from("Please Enter an Operation (+, -)");
    loop {
        println!("{}", text);
        let line = read_input();
        match line.as_str() {
            "+" =>  break operation_group::OperationType::Add,
            "-" => break operation_group::OperationType::Substract,
            _ => {text = String::from("Try Again! Only + and - are valid operators!"); eprintln!("Invalid Input"); continue;}
        }
        
    }
}

pub fn print_result(value: i32) {
    println!("The result of the last operation is: {}", value);
}

pub fn print_calc_error(error: String) {
    eprintln!("An error occured while calculating your result: \n {}", error)
}

pub fn begin_loop(ct: &CancellationToken) -> Result<(), OperationCanceled>{
    // Todo, implement a switch for processor types.
    let op_group = operation_group::OperationGroup::<SimpleProcessor>::new();
    let loop_result = loop {
        if let Err(_) = ct.result() {
            break Err(OperationCanceled)
        }

        let op = get_operation();
        let (value, value_text) = get_number();
        op_group.add_operation(operation_group::Operation {
            operation_type: op,
            value: value,
            value_text: value_text
        });
        match op_group.calculate() {
            Err(error) => print_calc_error(error),
            Ok(result) => print_result(result)
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