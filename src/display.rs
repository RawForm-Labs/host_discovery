// display: Sugar for implementing the Display trait on types
#[macro_export]
macro_rules! display {
    ($type:ident) => {
        impl std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}

// display: Sugar for implementing the Display trait on types
#[macro_export]
macro_rules! display_with_lifetimes {
    ($type:ident) => {
        impl<'a, 'b> std::fmt::Display for $type<'a, 'b> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}
