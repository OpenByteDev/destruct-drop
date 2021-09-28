/*!
Macro for dropping the fields of a struct or enum without dropping the container.

## Usage
Add `#[derive(DestructDrop)]` to your `struct` or `enum` definition.

```
use destruct_drop::DestructDrop;

#[derive(DestructDrop)]
struct Container {
    inner: Inner
}

struct Inner;

impl Drop for Container {
    fn drop(&mut self) {
        println!("dropped Container");
    }
}

impl Drop for Inner {
    fn drop(&mut self) {
        println!("dropped Inner");
    }
}

fn main() {
    // prints "dropped Inner" and then "dropped Container"
    drop(Container { inner: Inner });

    // prints only "dropped Inner"
    Container { inner: Inner }.destruct_drop();
}
```
!*/


pub use destruct_drop_derive::*;

/// Trait for consuming a struct or enum without dropping it while dropping all of its contents normally.
pub trait DestructDrop {
    /// Consume self without dropping it while dropping all of its contents normally.
    fn destruct_drop(self);
}
