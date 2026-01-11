/*
 * Use macros for repetitive code
 */

#[derive(Debug, Default)]
#[allow(dead_code)]
struct Bicycle {
    make: String,
    model: String,
    size: i32,
    color: String,
}

macro_rules! accessor {
    ($name:ident, &$rettype:ty) => {
        // matches on type where we want to return a reference
        #[allow(dead_code)]
        fn $name(&self) -> &$rettype {
            &self.$name
        }
    };

    ($name:ident, $rettype:ty) => {
        // matches on type where we want to return a copy
        #[allow(dead_code)]
        fn $name(&self) -> $rettype {
            self.$name
        }
    };
}

impl Bicycle {
    accessor!(make, &String);
    accessor!(model, &String);
    accessor!(size, i32);
    accessor!(color, &String);
}

macro_rules! with_str {
    ($name:ident, $func:ident) => {
        #[allow(dead_code)]
        fn $func(self, $name: &str) -> Self {
            Self {
                bicycle: Bicycle {
                    $name: $name.into(),
                    ..self.bicycle
                },
            }
        }
    };
}
macro_rules! with {
    ($name:ident, $func:ident, $type:ty) => {
        #[allow(dead_code)]
        fn $func(self, $name: $type) -> Self {
            Self {
                bicycle: Bicycle {
                    $name,
                    ..self.bicycle
                },
            }
        }
    };
}

#[allow(dead_code)]
pub struct BuilderMk2 {
    bicycle: Bicycle,
}

impl BuilderMk2 {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            bicycle: Bicycle::default(),
        }
    }

    with_str!(make, with_make);
    with_str!(model, with_model);
    with!(size, with_size, i32);
    with_str!(color, with_color);

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

impl Builder<Bicycle> for BuilderMk2 {
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

impl Buildable<Bicycle, BuilderMk2> for Bicycle {
    fn builder() -> BuilderMk2 {
        BuilderMk2::new()
    }
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn test_builder_mk2() {
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
