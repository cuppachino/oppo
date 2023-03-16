/// Bucketmodule #2: Isomorphic Type-State
mod iso {
    #[derive(Debug, Clone, Copy)]
    pub struct Iso<T, U>(pub T, pub U);

    pub trait IsoTrait {
        type Back;
        type There;

        fn back(self) -> Back<Self::Back, Self::There>;
        fn there(self) -> There<Self::There, Self::Back>;
    }

    impl<T, U> IsoTrait for Iso<T, U> {
        type Back = T;
        type There = U;

        fn back(self) -> Back<T, U> {
            Back(self.0, self.1)
        }

        fn there(self) -> There<U, T> {
            There(self.1, self.0)
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Back<T, U>(T, U);

    impl<T, U> Back<T, U> {
        pub fn there(self) -> There<T, U> {
            There(self.0, self.1)
        }
        pub fn value(self) -> T {
            self.0
        }
    }
    #[derive(Debug, Clone, Copy)]
    pub struct There<T, U>(T, U);

    impl<T, U> There<T, U> {
        pub fn back(self) -> Back<U, T> {
            Back(self.1, self.0)
        }
        pub fn value(self) -> T {
            self.0
        }
    }

    pub trait Reflect {
        type Output;
        fn reflect(self) -> Self::Output;
    }

    impl<T, U> Reflect for Iso<T, U> {
        type Output = Iso<U, T>;
        fn reflect(self) -> Self::Output {
            Iso(self.1, self.0)
        }
    }

    impl<T, U> Reflect for Back<T, U> {
        type Output = There<U, T>;
        fn reflect(self) -> Self::Output {
            There(self.1, self.0)
        }
    }

    impl<T, U> Reflect for There<T, U> {
        type Output = Back<U, T>;
        fn reflect(self) -> Self::Output {
            Back(self.1, self.0)
        }
    }
}

fn main() {
    use iso::*;

    let my_iso = Iso("Hi", 42);
    println!("{:?}", my_iso.there());
    println!("{:?}", my_iso.back());

    println!("{}", "-".repeat(20));

    let there = my_iso.there();

    println!("{:?}", there); // There(42, "Hi")
    println!("{:?}", there.back().there().reflect()); // Back(42, "Hi")
    println!("{:?}", there.back().there().reflect().reflect()); // There(42, "Hi")
    println!("{:?}", there.back().there().reflect()); // Back(42, "Hi")
    println!("{:?}", there.back().there().reflect().there()); // There(42, "Hi")
    println!("{:?}", there.back().there().reflect().there().value()); // 42
    println!("{:?}", there.reflect().reflect().back().value()); // "Hi"

    // what's cool is when you are in the "There" state, you don't have access to a "there" method.
    // you can only go back to the "Back" state, and then go back to the "There" state.
    // type state machine!
}
