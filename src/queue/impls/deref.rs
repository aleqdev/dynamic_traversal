use std::ops::Deref;

use crate::DynamicQueue;

impl<T> Deref for DynamicQueue<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.storage
    }
}
