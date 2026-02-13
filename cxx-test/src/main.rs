mod cpp_iface;
use cpp_iface::el_test::ElTest;

fn main() {
    let mut tst = ElTest::new("Hello from Rust via CXX!");
    println!("Result from C++ Test1::method_a(): {}", tst.method_a());
    println!("Result from C++ Test1::get_msg(): {}", tst.get_msg());
    println!("n = {}", tst.get_n());
    tst.set_n(123);
    println!("n = {}", tst.get_n());

    let data: [u32; 5] = [1, 2, 3, 4, 5];
    tst.dump(&data);
}
