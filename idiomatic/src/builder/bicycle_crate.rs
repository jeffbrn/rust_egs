use derive_builder::Builder;

/*
 * Use crate for builder pattern: https://crates.io/crates/derive_builder
 * documentation: https://docs.rs/derive_builder/latest/derive_builder/
 */

#[derive(Debug, Default, Builder)]
#[builder(pattern = "owned")]
#[allow(dead_code)]
struct Bicycle {
    make: String,
    model: String,
    size: i32,
    color: String,
}

impl Bicycle {
    #[allow(dead_code)]
    fn make(&self) -> &String {
        &self.make
    }

    #[allow(dead_code)]
    fn model(&self) -> &String {
        &self.model
    }

    #[allow(dead_code)]
    fn size(&self) -> i32 {
        self.size
    }

    #[allow(dead_code)]
    fn color(&self) -> &String {
        &self.color
    }
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn test_bicycle_builder_crate() {
        let builder = BicycleBuilder::default();
        let bike = builder
            .make("Trek".to_string())
            .model("Domane SL6".to_string())
            .size(56)
            .color("Red".to_string())
            .build()
            .unwrap();

        assert_eq!(bike.make(), "Trek");
        assert_eq!(bike.model(), "Domane SL6");
        assert_eq!(bike.size(), 56);
        assert_eq!(bike.color(), "Red");
        println!("My new bike:{:?}", bike);
    }
}
