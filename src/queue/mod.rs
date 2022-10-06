pub mod impls;
pub use impls::*;

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub struct DynamicQueue<T> {
    pub storage: Vec<T>,
}
