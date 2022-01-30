#![windows_subsystem = "windows"]

use qt_core::{q_init_resource};
use qt_widgets::{QApplication,};


mod calc;

fn main() {
    QApplication::init(|_| {
        q_init_resource!("resources");
        let calc_app = calc::CalcApp::new();
        calc_app.show();
        unsafe { QApplication::exec() }
    })
}