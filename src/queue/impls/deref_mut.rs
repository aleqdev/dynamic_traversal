use std::ops::DerefMut;

use crate::DynamicQueue;

impl<T> DerefMut for DynamicQueue<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.storage
    }
}
