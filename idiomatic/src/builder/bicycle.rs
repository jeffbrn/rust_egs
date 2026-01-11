#[derive(Debug, Default)]
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

/*
 * Builder pattern using a fluent interface
 */
#[allow(dead_code)]
pub struct BuilderMk1 {
    bicycle: Bicycle,
}

impl BuilderMk1 {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            bicycle: Bicycle::default(),
        }
    }

    #[allow(dead_code)]
    fn with_make(self, make: &str) -> Self {
        Self {
            bicycle: Bicycle {
                make: make.to_string(),
                ..self.bicycle
            },
        }
    }

    #[allow(dead_code)]
    fn with_model(self, model: &str) -> Self {
        Self {
            bicycle: Bicycle {
                model: model.to_string(),
                ..self.bicycle
            },
        }
    }

    #[allow(dead_code)]
    fn with_size(self, size: i32) -> Self {
        Self {
            bicycle: Bicycle {
                size,
                ..self.bicycle
            },
        }
    }

    #[allow(dead_code)]
    fn with_color(self, color: &str) -> Self {
        Self {
            bicycle: Bicycle {
                color: color.to_string(),
                ..self.bicycle
            },
        }
    }

    #[allow(dead_code)]
    fn build(self) -> Bicycle {
        self.bicycle
    }
}

/*
 * Builder pattern using traits
 */
#[allow(dead_code)]
trait Builder<T> {
    fn new() -> Self;
    fn build(self) -> T;
}

impl Builder<Bicycle> for BuilderMk1 {
    fn new() -> Self {
        Self {
            bicycle: Bicycle::default(),
        }
    }

    fn build(self) -> Bicycle {
        self.bicycle
    }
}

#[allow(dead_code)]
trait Buildable<Target, B: Builder<Target>> {
    fn builder() -> B;
}

impl Buildable<Bicycle, BuilderMk1> for Bicycle {
    fn builder() -> BuilderMk1 {
        BuilderMk1::new()
    }
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn test_builder_mk1() {
        let builder = Bicycle::builder();
        let bike = builder
            .with_make("Trek")
            .with_model("Domane SL6")
            .with_size(56)
            .with_color("Red")
            .build();

        assert_eq!(bike.make(), "Trek");
        assert_eq!(bike.model(), "Domane SL6");
        assert_eq!(bike.size(), 56);
        assert_eq!(bike.color(), "Red");
        println!("My new bike:{:?}", bike);
    }
}
