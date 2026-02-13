use super::bindings::{get_n, set_n, Test1, dump};
use autocxx::WithinUniquePtr;
use cxx::UniquePtr;

pub struct ElTest {
    wrapped: UniquePtr<Test1>,
}

impl ElTest {
    pub fn new(msg: &str) -> Self {
        cxx::let_cxx_string!(cxx_str = msg);
        let wrapped = Test1::new(&cxx_str).within_unique_ptr();
        ElTest { wrapped }
    }

    pub fn method_a(&mut self) -> bool {
        self.wrapped.pin_mut().method_a()
    }

    pub fn get_msg(&mut self) -> String {
        self.wrapped
            .pin_mut()
            .get_msg()
            .to_string_lossy()
            .into_owned()
    }

    pub fn get_n(&self) -> i32 {
        get_n(&self.wrapped)
    }

    pub fn set_n(&mut self, val: i32) {
        set_n(&mut self.wrapped, val);
    }

    pub fn dump(&self, data: &[u32]) {
        dump(data);
    }
}
