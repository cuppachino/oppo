use thiserror::Error;

// Possible errors
#[derive(Error, Debug)]
pub enum Error {
    #[error("Default object")]
    UnknownObject,
}

fn main() -> Result<(), Error> {
    let object = Draft::new().name("carl").build();

    println!("Hello, {}!", object.name);
    Ok(())
}

// Object --
#[derive(Debug)]
pub struct Object {
    pub name: String,
}

// -- Builder
#[derive(Default, Clone)]
pub struct Draft<T> {
    name: T,
}
// -- Builder states
#[derive(Default, Clone)]
pub struct Default;
#[derive(Default, Clone)]
pub struct Name(String);

// # State interface layers
impl Draft<Default> {
    pub fn new() -> Self {
        Draft::default()
    }
}

impl<T> Draft<T> {
    pub fn name(self, name: impl Into<String>) -> Draft<Name> {
        Draft {
            name: Name(name.into()),
        }
    }
}

impl Draft<Name> {
    pub fn build(self) -> Object {
        Object { name: self.name.0 }
    }
}
