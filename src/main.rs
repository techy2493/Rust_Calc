#![windows_subsystem = "windows"]

use std::env;

use processors::simple_processor::SimpleProcessor;
use qt_core::q_init_resource;
use qt_widgets::QApplication;
use cancellation::CancellationTokenSource;
mod calc;
pub mod command_line;
pub mod operation_group;
pub mod traits;
pub mod processors;

fn main() {
    let cts = CancellationTokenSource::new();

    // TODO: MULTI-THREAD THIS, mutex? So that this thread can watch for signals.
    for arg in  env::args() {
        match arg.as_str() {
            "--gui" => {load_ui(); break;}
            _ => {let _ = command_line::begin_loop::<SimpleProcessor<i32>, i32>(&cts);}
        }
    };
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


