#![windows_subsystem = "windows"]

use std::env;

use fixed::types::I20F12;
use processors::simple_processor::SimpleProcessor;
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
        //Todo: Use Args to determine processor type
        let _ = command_line::begin_loop::<SimpleProcessor<i32>, i32>(&cts);
    }
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