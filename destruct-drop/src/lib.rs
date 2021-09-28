pub use destruct_drop_derive::*;

pub trait DestructDrop {
    fn destruct_drop(self);
}
