use autocxx::prelude::*;

include_cpp! {
    #include "test.hpp"

    name!(my_cpp_test)
    safety!(unsafe)
    generate!("Test1")
}

fn main() {
    println!("Hello, world!");
    cxx::let_cxx_string!(cxx_str = "Hello from Rust via CXX!");
    let mut tst1 = my_cpp_test::Test1::new(&cxx_str).within_unique_ptr();
    let a = tst1.pin_mut().method_a();
    println!("Result from C++ Test1::method_a(): {}", a);
    let b = tst1.pin_mut().get_msg();
    println!("Result from C++ Test1::get_msg(): {}", b);
}
