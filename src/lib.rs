//! String-mapped enum
//!
//! This crate exports the macro `strmapped_enum!` that builds an enum and
//! automatically implements `std::str::FromStr` and `std::fmt::Display`.

#[macro_export]
macro_rules! strmapped_enum_impl {
    ($name:ident, $( $variant:ident = $value:expr ),+) => {
        impl $name {
            fn to_str(&self) -> &'static str {
                match self {
                    $( &$name::$variant => $value, )+
                }
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = String;

            fn from_str(value: &str) -> Result<$name, String> {
                match value {
                    $( $value => Ok($name::$variant), )+
                    _ => return Err(value.to_string()),
                }
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match self {
                    $( &$name::$variant => write!(f, "{}", $value.to_string()), )+
                }
            }
        }
    };
}

#[macro_export]
macro_rules! strmapped_enum {
    (
        $( #[$attr:meta] )*
        pub enum $name:ident {
            $( $variant:ident = $value:expr, )+
        }
    ) => {
        $( #[$attr])*
        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        pub enum $name {
            $( $variant ),+
        }

        strmapped_enum_impl! { $name, $( $variant = $value ),+ }
    };

    (
        $( #[$attr:meta] )*
        enum $name:ident {
            $( $variant:ident = $value:expr, )+
        }
    ) => {
        $( #[$attr])*
        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        enum $name {
            $( $variant ),+
        }

        strmapped_enum_impl! { $name, $( $variant = $value ),+ }
    };
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    strmapped_enum!(
    pub enum Test {
        First = "First",
        Second = "Second",
    }
    );

    #[test]
    fn check_eq_bare() {
        assert_eq!(Test::First, Test::First);
    }

    #[test]
    fn check_eq_to_str() {
        assert_eq!(Test::First.to_str(), "First");
    }

    #[test]
    fn check_eq_to_string() {
        assert_eq!(Test::First.to_string(), "First");
    }

    #[test]
    fn check_eq_from_str() {
        assert_eq!(Test::from_str("First"), Ok(Test::First));
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn check_ne_bare() {
        assert_eq!(Test::First, Test::Second);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn check_ne_to_str() {
        assert_eq!(Test::Second.to_str(), "First");
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn check_ne_to_string() {
        assert_eq!(Test::First.to_string(), "Second");
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn check_ne_from_str() {
        assert_eq!(Test::from_str("First"), Ok(Test::Second));
    }
}
