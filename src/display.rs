#[macro_export]
macro_rules! display_profile {
    ($type:ident) => {
        impl<'a, 'b> std::fmt::Display for $type<'a, 'b> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}

#[macro_export]
macro_rules! display_graphics {
    ($type:ident) => {
        impl std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}

#[macro_export]
macro_rules! display_processor {
    ($type:ty) => {
        impl<M, C> std::fmt::Display for $type
        where
            M: std::fmt::Debug,
            C: std::fmt::Debug,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}

