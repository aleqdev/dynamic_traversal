use crate::DynamicQueue;

impl<T> Default for DynamicQueue<T> {
    fn default() -> Self {
        Self {
            storage: Default::default(),
        }
    }
}
