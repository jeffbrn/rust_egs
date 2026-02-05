use autocxx::prelude::*;

include_cpp! {
    #include "test.hpp"

    name!(my_cpp_test)
    safety!(unsafe)
    generate!("Test1")
}

#[cxx::bridge]
mod ffi_wrapper {
    unsafe extern "C++" {
        include!("test1_helper.h");

        type Test1 = crate::cpp_iface::bindings::my_cpp_test::Test1;

        fn get_n(t: &Test1) -> i32;
        fn set_n(t: &mut Test1, val: i32);
    }
}

pub use ffi_wrapper::*;
pub use my_cpp_test::Test1;
