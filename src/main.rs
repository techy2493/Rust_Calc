#![windows_subsystem = "windows"]

use std::env;

use qt_core::{q_init_resource};
use qt_widgets::{QApplication,};
use cancellation::{CancellationTokenSource};
mod calc;
pub mod command_line;
pub mod operation_group;
pub mod traits;
pub mod processors;

fn main() {
    let cts = CancellationTokenSource::new();

    let args: Vec<String> = env::args().collect();

    // TODO: MULTI-THREAD THIS, mutex? So that this thread can watch for signals.
    if args.contains(&String::from("--gui")) {
        load_ui();
    } else {
        let _ = command_line::begin_loop(&cts);
    }
    







    // // loadUI();
    // let mut has_not_exited = true;
    // let mut result: i32;
    //     while has_not_exited {
    //     let op = command_line::get_operation();
    //     let (value, value_text) = command_line::get_number();
    //     let operation = operation_group::Operation {
    //         operation_type: op,
    //         value: value,
    //         value_text: value_text
    //     };
    //     result = calculate();
    //     println!("New Value Is: {}", result);
    //}
}

fn load_ui() {
    QApplication::init(|_| {
        // This error isn't really an error. Go figure. 
        // Seems like Analyzer is missing type references?
        q_init_resource!("resources"); 
        let calc_app = calc::CalcApp::new();
        calc_app.show();
        unsafe { QApplication::exec() }
    })
}