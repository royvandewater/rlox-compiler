use derive_more::{Deref, DerefMut};

#[derive(Deref, DerefMut)]
pub struct Value(f64);

#[derive(Deref, DerefMut)]
pub struct ValueArray(Vec<Value>);

impl ValueArray {
    pub(crate) fn new() -> Self {
        Self(Vec::new())
    }
}
