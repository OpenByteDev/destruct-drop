# destruct-drop

[![CI](https://github.com/OpenByteDev/destruct-drop/actions/workflows/ci.yml/badge.svg)](https://github.com/OpenByteDev/destruct-drop/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/destruct-drop.svg)](https://crates.io/crates/destruct-drop)
[![Documentation](https://docs.rs/destruct-drop/badge.svg)](https://docs.rs/destruct-drop)
[![dependency status](https://deps.rs/repo/github/openbytedev/destruct-drop/status.svg)](https://deps.rs/repo/github/openbytedev/destruct-drop)
[![MIT](https://img.shields.io/crates/l/destruct-drop.svg)](https://github.com/OpenByteDev/destruct-drop/blob/master/LICENSE)

Macro for dropping the fields of a struct or enum without dropping the container.

## Usage
Add `#[derive(DestructDrop)]` to your `struct` or `enum` definition.
```rust
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

## License
Licensed under MIT license ([LICENSE](https://github.com/OpenByteDev/destruct-drop/blob/master/LICENSE) or http://opensource.org/licenses/MIT)
