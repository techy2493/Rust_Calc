pub trait Newable {
    fn new() -> Self;
}

// impl Default for Newable
// {
//     fn default() -> Self {
//         Self::new()
//     }
// }