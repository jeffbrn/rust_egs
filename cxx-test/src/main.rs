use autocxx::prelude::*;

include_cpp! {
    #include "test.hpp"

    name!(my_cpp_test)
    safety!(unsafe)
    generate!("Test1")
}

fn main() {
    println!("Hello, world!");
}
