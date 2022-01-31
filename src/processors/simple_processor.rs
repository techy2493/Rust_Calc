use crate::traits::{newable::Newable, processor::Processor};
pub struct SimpleProcessor {

}

impl Newable for SimpleProcessor {
    fn new() -> Self {
        return SimpleProcessor {}
    }
}

impl Processor for SimpleProcessor {
    fn process(self: &Self) -> Result<i32, String> {
        Err(String::from("Not Implemented Yet!"))
    }
}
