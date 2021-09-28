use std::{cell::Cell, rc::Rc};

use destruct_drop::DestructDrop;

#[derive(DestructDrop)]
struct DontDropThis {
    a: DropThis,
    b: u32,
    c: DropThis,
}

impl Drop for DontDropThis {
    fn drop(&mut self) {
        panic!("Dropped DontDropThis");
    }
}

#[derive(Default)]
struct DropThis(Rc<Cell<bool>>);

impl Drop for DropThis {
    fn drop(&mut self) {
        if self.0.get() {
            panic!("Tried to drop DropThis twice!");
        }
        self.0.set(true);
    }
}

#[test]
fn basic_struct() {
    let dropped_a = Rc::new(Cell::new(false));
    let dropped_c = Rc::new(Cell::new(false));

    let dont_drop_this = DontDropThis {
        a: DropThis(dropped_a.clone()),
        b: 0,
        c: DropThis(dropped_c.clone()),
    };

    assert!(!dropped_a.get());
    assert!(!dropped_c.get());

    dont_drop_this.destruct_drop();

    assert!(dropped_a.get());
    assert!(dropped_c.get());
}

#[derive(DestructDrop)]
struct DontDropThisTuple(DropThis, u32, DropThis);

impl Drop for DontDropThisTuple {
    fn drop(&mut self) {
        panic!("Dropped DontDropThisTuple");
    }
}

#[test]
fn basic_tuple_struct() {
    let dropped_0 = Rc::new(Cell::new(false));
    let dropped_2 = Rc::new(Cell::new(false));

    let dont_drop_this =
        DontDropThisTuple(DropThis(dropped_0.clone()), 0, DropThis(dropped_2.clone()));

    assert!(!dropped_0.get());
    assert!(!dropped_2.get());

    dont_drop_this.destruct_drop();

    assert!(dropped_0.get());
    assert!(dropped_2.get());
}

#[derive(DestructDrop)]
struct DontDropThisUnit;

impl Drop for DontDropThisUnit {
    fn drop(&mut self) {
        panic!("Dropped DontDropThisUnit");
    }
}

#[test]
fn basic_unit_struct() {
    let dont_drop_this = DontDropThisUnit;
    dont_drop_this.destruct_drop();
}

#[derive(DestructDrop)]
enum DontDropThisEnum {
    A,
    B { v: DropThis },
    C(DropThis),
}

impl Drop for DontDropThisEnum {
    fn drop(&mut self) {
        panic!("Dropped DontDropThisEnum");
    }
}

#[test]
fn basic_unit_enum() {
    let dont_drop_this_a = DontDropThisEnum::A;
    dont_drop_this_a.destruct_drop();

    let dropped_b = Rc::new(Cell::new(false));
    let dont_drop_this_b = DontDropThisEnum::B {
        v: DropThis(dropped_b.clone()),
    };
    assert!(!dropped_b.get());
    dont_drop_this_b.destruct_drop();
    assert!(dropped_b.get());

    let dropped_c = Rc::new(Cell::new(false));
    let dont_drop_this_c = DontDropThisEnum::C(DropThis(dropped_c.clone()));
    assert!(!dropped_c.get());
    dont_drop_this_c.destruct_drop();
    assert!(dropped_c.get());
}
