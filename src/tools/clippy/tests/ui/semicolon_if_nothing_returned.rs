//@aux-build:proc_macro_attr.rs

#![warn(clippy::semicolon_if_nothing_returned)]
#![allow(
    clippy::redundant_closure,
    clippy::uninlined_format_args,
    clippy::needless_late_init,
    clippy::empty_docs
)]

#[macro_use]
extern crate proc_macro_attr;

fn get_unit() {}

// the functions below trigger the lint
fn main() {
    println!("Hello")
    //~^ semicolon_if_nothing_returned
}

fn hello() {
    get_unit()
    //~^ semicolon_if_nothing_returned
}

fn basic101(x: i32) {
    let y: i32;
    y = x + 1
    //~^ semicolon_if_nothing_returned
}

#[rustfmt::skip]
fn closure_error() {
    let _d = || {
        hello()
        //~^ semicolon_if_nothing_returned
    };
}

#[rustfmt::skip]
fn unsafe_checks_error() {
    use std::mem::MaybeUninit;
    use std::ptr;

    let mut s = MaybeUninit::<String>::uninit();
    let _d = || unsafe {
        ptr::drop_in_place(s.as_mut_ptr())
        //~^ semicolon_if_nothing_returned
    };
}

// this is fine
fn print_sum(a: i32, b: i32) {
    println!("{}", a + b);
    assert_eq!(true, false);
}

fn foo(x: i32) {
    let y: i32;
    if x < 1 {
        y = 4;
    } else {
        y = 5;
    }
}

fn bar(x: i32) {
    let y: i32;
    match x {
        1 => y = 4,
        _ => y = 32,
    }
}

fn foobar(x: i32) {
    let y: i32;
    'label: {
        y = x + 1;
    }
}

fn loop_test(x: i32) {
    let y: i32;
    for &ext in &["stdout", "stderr", "fixed"] {
        println!("{}", ext);
    }
}

fn closure() {
    let _d = || hello();
}

#[rustfmt::skip]
fn closure_block() {
    let _d = || { hello() };
}

unsafe fn some_unsafe_op() {}
unsafe fn some_other_unsafe_fn() {}

fn do_something() {
    unsafe { some_unsafe_op() };

    unsafe { some_other_unsafe_fn() };
}

fn unsafe_checks() {
    use std::mem::MaybeUninit;
    use std::ptr;

    let mut s = MaybeUninit::<String>::uninit();
    let _d = || unsafe { ptr::drop_in_place(s.as_mut_ptr()) };
}

// Issue #7768
#[rustfmt::skip]
fn macro_with_semicolon() {
    macro_rules! repro {
        () => {
            while false {
            }
        };
    }
    repro!();
}

fn function_returning_option() -> Option<i32> {
    Some(1)
}

// No warning
fn let_else_stmts() {
    let Some(x) = function_returning_option() else {
        return;
    };
}

mod issue12123 {
    #[rustfmt::skip]
    mod this_triggers {
        #[fake_main]
        async fn main() {

        }
    }

    mod and_this {
        #[fake_main]
        async fn main() {
            println!("hello");
        }
    }

    #[rustfmt::skip]
    mod maybe_this {
        /** */ #[fake_main]
        async fn main() {
        }
    }

    mod but_this_does_not {
        #[fake_main]
        async fn main() {}
    }
}
