// TODO: Figure out another way to bind this. And get it calling to operation_group
// Would like to make a separate file to actually house bindings to other parts of the app
// And keep this file as strictly a basic designer file.

use cpp_core::{Ptr, StaticUpcast};
use qt_core::{
    slot, QBox, QObject, QPtr,
    QSortFilterProxyModel, SlotNoArgs, QCoreApplication, QString
};
use qt_gui::{QStandardItemModel};
use qt_ui_tools::ui_form;
use qt_widgets::{QPushButton, QWidget, QLineEdit, QAction};
use std::{rc::Rc, cell::RefCell};
use paste;

macro_rules! makeConnect {
    ($self:ident, $ob:ident, $func:ident) => {
        $self.form
        .$ob
        .clicked()
        .connect(&$self.$func());
    }
}

macro_rules! argConnect {
    ( $self:ident, $($arg0:literal),+ ) => {
            $(
                paste::item! {
                    makeConnect!($self, [<pushButton_ $arg0>], [<slot_on_ $arg0 _clicked>])
                }
            )+
    }
}
 
macro_rules! argSlot {
    ( $func:ident, $($arg0:literal),+ ) => {
        $(
            paste::item! {
                #[slot(SlotNoArgs)]
                unsafe fn [<on_ $arg0 _clicked>](self: &Rc<Self>) {
                    self.$func($arg0);
                }
            }
        )+
    }
}

#[ui_form("../ui/calc.ui")]
#[allow(non_snake_case)]
struct Form {

    mainForm: QBox<QWidget>,
    inputValue: QPtr<QLineEdit>,
    resultValue: QPtr<QLineEdit>,
    pushButton_0: QPtr<QPushButton>,
    pushButton_1: QPtr<QPushButton>,
    pushButton_2: QPtr<QPushButton>,
    pushButton_3: QPtr<QPushButton>,
    pushButton_4: QPtr<QPushButton>,
    pushButton_5: QPtr<QPushButton>,
    pushButton_6: QPtr<QPushButton>,
    pushButton_7: QPtr<QPushButton>,
    pushButton_8: QPtr<QPushButton>,
    pushButton_9: QPtr<QPushButton>,
    pushButton_minus: QPtr<QPushButton>,
    pushButton_plus: QPtr<QPushButton>,
    actionClose: QPtr<QAction>
}

pub enum OperationType {
    Add,
    Substract
}

pub struct CalcOperation {
    value: String,
    operation_type: OperationType
}

pub struct CalcApp {
    form: Form, 
    source_model: QBox<QStandardItemModel>,
    proxy_model: QBox<QSortFilterProxyModel>,
    operations: RefCell<Vec<CalcOperation>>
}

impl StaticUpcast<QObject> for CalcApp {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.form.mainForm.as_ptr().static_upcast()
    }
}

impl CalcApp {

    pub fn new() -> Rc<Self> {
        unsafe {
            let this = Rc::new(CalcApp {
                form: Form::load(),
                source_model: QStandardItemModel::new_0a(),
                proxy_model: QSortFilterProxyModel::new_0a(),
                operations: RefCell::new(Vec::<CalcOperation>::new())
            });
            this.init();
            this
        }
    }

    argSlot!(number_clicked, 9,8,7,6,5,4,3,2,1,0);


    pub unsafe fn number_clicked(self: &Rc<Self>, number: i32) {
        self.form.inputValue.set_text(self.form.inputValue.text().append_q_string(&QString::number_int(number)));
        println!("Number Pressed {}", number);
    }

    pub unsafe fn operation_clicked(self: &Rc<Self>, operation: OperationType) {
        self.operations.borrow_mut().push(CalcOperation {
            value: self.form.inputValue.text().to_std_string(),
            operation_type: operation
        });
        self.form.inputValue.clear();
        self.calculate_result();
        println!("{}", self.operations.borrow().len())
    }

    unsafe fn calculate_result(self: &Rc<Self>) {
        let ops = self.operations.borrow();
        let mut value: i32 = 0;
        for (_, op) in ops.iter().enumerate() {
            let opval = &op.value.parse::<i32>().unwrap();
            match op.operation_type {
                OperationType::Add => value += opval,
                OperationType::Substract => value -= opval,
            }
        }
        self.form.resultValue.set_text(&QString::number_int(value));
    }

    unsafe fn init(self: &Rc<Self>) {
        self.proxy_model.set_source_model(&self.source_model);
        self.form
        .actionClose
        .triggered()
        .connect(&self.slot_on_close_clicked());
        argConnect!(self, 0,1,2,3,4,5,6,7,8,9);
        self.form.pushButton_minus.clicked().connect(&self.slot_on_minus_clicked());
        self.form.pushButton_plus.clicked().connect(&self.slot_on_plus_clicked());
        
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_minus_clicked(self: &Rc<Self>) {
        self.operation_clicked(OperationType::Substract)
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_plus_clicked(self: &Rc<Self>) {
        self.operation_clicked(OperationType::Add)
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_close_clicked(self: &Rc<Self>) {
        QCoreApplication::quit()
    }

    pub fn show(self: &Rc<Self>) {
        unsafe {
            self.form.mainForm.show();
        }
    }
}

