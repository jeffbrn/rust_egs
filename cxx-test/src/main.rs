mod cpp_iface;
use cpp_iface::el_test::ElTest;

fn main() {
    let mut tst = ElTest::new("Hello from Rust via CXX!");
    println!("Result from C++ Test1::method_a(): {}", tst.method_a());
    println!("Result from C++ Test1::get_msg(): {}", tst.get_msg());
}
