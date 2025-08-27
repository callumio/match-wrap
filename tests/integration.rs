use match_wrap::*;

struct Foo;
struct Bar;
struct Baz;

trait MyTrait {
    fn my_function(&self) -> &'static str {
        "test"
    }
}

impl Foo {}
impl MyTrait for Foo {}
impl MyTrait for Bar {}
impl MyTrait for Baz {}

#[test]
fn test_box_compiles() {
    let match_on = 1;
    let my_box = match_box! {
        dyn MyTrait;
        match_on;
        1 => Foo {},
        2 => Bar {},
        3 => Baz {},
        #[diverges]
        _ => unimplemented!(),
    };
    assert_eq!(my_box.my_function(), "test");
}

#[test]
fn test_arc_compiles() {
    let match_on = 2;
    let my_arc = match_arc! {
        dyn MyTrait;
        match_on;
        1 => Foo {},
        2 => Bar {},
        3 => Baz {},
        #[diverges]
        _ => unimplemented!(),
    };
    assert_eq!(my_arc.my_function(), "test");
}

#[test]
fn test_rc_compiles() {
    let match_on = 3;
    let my_rc = match_rc! {
        dyn MyTrait;
        match_on;
        1 => Foo {},
        2 => Bar {},
        3 => Baz {},
        #[diverges]
        _ => unimplemented!(),
    };
    assert_eq!(my_rc.my_function(), "test");
}
