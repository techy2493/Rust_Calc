pub trait Processor {
    fn process(self: &Self) -> Result<i32, String>;
}