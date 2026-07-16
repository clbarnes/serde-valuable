use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use crate::Value;

macro_rules! impl_from {
    ($($ty:ty => $variant:ident),* $(,)?) => {
        $(
            impl From<$ty> for Value {
                fn from(value: $ty) -> Self {
                    Value::$variant(value)
                }
            }
        )*
    };
}

impl_from! {
    bool => Bool,
    u8 => U8,
    u16 => U16,
    u32 => U32,
    u64 => U64,
    u128 => U128,
    i8 => I8,
    i16 => I16,
    i32 => I32,
    i64 => I64,
    i128 => I128,
    f32 => F32,
    f64 => F64,
    char => Char,
    String => String,
}

impl<const N: usize> From<[Value; N]> for Value {
    fn from(value: [Value; N]) -> Self {
        Value::Seq(value.to_vec())
    }
}

macro_rules! impl_from_tuple {
    // Entry point: takes the number of elements in the tuple and a matching
    // list of tuple index positions, e.g. `impl_from_tuple!(3; 0 1 2);`.
    ($len:literal; $($idx:tt)*) => {
        impl From<( $( impl_from_tuple!(@value $idx), )* )> for Value {
            fn from(value: ( $( impl_from_tuple!(@value $idx), )* )) -> Self {
                Value::Seq(::alloc::vec![ $( value.$idx ),* ])
            }
        }
    };
    // Internal helper: map each index token to the `Value` element type.
    (@value $idx:tt) => { Value };
}

impl_from_tuple!(1; 0);
impl_from_tuple!(2; 0 1);
impl_from_tuple!(3; 0 1 2);
impl_from_tuple!(4; 0 1 2 3);
impl_from_tuple!(5; 0 1 2 3 4);
impl_from_tuple!(6; 0 1 2 3 4 5);
impl_from_tuple!(7; 0 1 2 3 4 5 6);
impl_from_tuple!(8; 0 1 2 3 4 5 6 7);

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Self {
        Value::Bytes(value)
    }
}

impl From<()> for Value {
    fn from((): ()) -> Self {
        Value::Unit
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Value::Seq(value)
    }
}

impl From<crate::Map<Value, Value>> for Value {
    fn from(value: crate::Map<Value, Value>) -> Self {
        Value::Map(value)
    }
}

impl FromIterator<Value> for Value {
    fn from_iter<I: IntoIterator<Item = Value>>(iter: I) -> Self {
        Value::Seq(iter.into_iter().collect())
    }
}

impl FromIterator<(Value, Value)> for Value {
    fn from_iter<I: IntoIterator<Item = (Value, Value)>>(iter: I) -> Self {
        Value::Map(iter.into_iter().collect())
    }
}

impl From<Option<Value>> for Value {
    fn from(value: Option<Value>) -> Self {
        Value::Option(value.map(Box::new))
    }
}

macro_rules! impl_as_copy {
    ($($method:ident => $variant:ident : $ty:ty),* $(,)?) => {
        $(
            #[doc = concat!("Returns the inner value if this is a [`Value::", stringify!($variant), "`], otherwise [`None`].")]
            pub fn $method(&self) -> Option<$ty> {
                match self {
                    Value::$variant(v) => Some(*v),
                    _ => None,
                }
            }
        )*
    };
}

macro_rules! impl_as_ref {
    ($($method:ident, $method_mut:ident => $variant:ident : $ty:ty),* $(,)?) => {
        $(
            #[doc = concat!("Returns a reference to the inner value if this is a [`Value::", stringify!($variant), "`], otherwise [`None`].")]
            pub fn $method(&self) -> Option<&$ty> {
                match self {
                    Value::$variant(v) => Some(v),
                    _ => None,
                }
            }

            #[doc = concat!("Returns a mutable reference to the inner value if this is a [`Value::", stringify!($variant), "`], otherwise [`None`].")]
            pub fn $method_mut(&mut self) -> Option<&mut $ty> {
                match self {
                    Value::$variant(v) => Some(v),
                    _ => None,
                }
            }
        )*
    };
}

impl Value {
    pub fn newtype(value: Value) -> Self {
        Value::Newtype(Box::new(value))
    }

    impl_as_copy! {
        as_bool => Bool: bool,
        as_u8 => U8: u8,
        as_u16 => U16: u16,
        as_u32 => U32: u32,
        as_u64 => U64: u64,
        as_u128 => U128: u128,
        as_i8 => I8: i8,
        as_i16 => I16: i16,
        as_i32 => I32: i32,
        as_i64 => I64: i64,
        as_i128 => I128: i128,
        as_f32 => F32: f32,
        as_f64 => F64: f64,
        as_char => Char: char,
    }

    impl_as_ref! {
        as_string, as_string_mut => String: String,
        as_seq, as_seq_mut => Seq: Vec<Value>,
        as_map, as_map_mut => Map: crate::Map<Value, Value>,
        as_bytes, as_bytes_mut => Bytes: Vec<u8>,
    }

    /// Returns `Some(())` if this is a [`Value::Unit`], otherwise [`None`].
    pub fn as_unit(&self) -> Option<()> {
        match self {
            Value::Unit => Some(()),
            _ => None,
        }
    }

    /// Returns the inner string slice if this is a [`Value::String`], otherwise [`None`].
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(v) => Some(v),
            _ => None,
        }
    }

    /// Returns a reference to the inner value if this is a [`Value::Option`], otherwise [`None`].
    ///
    /// The outer [`Option`] distinguishes "not a [`Value::Option`]" ([`None`]) from a
    /// [`Value::Option`] holding a value (`Some(Some(_))`) or holding nothing (`Some(None)`).
    #[allow(clippy::type_complexity)]
    pub fn as_option(&self) -> Option<Option<&Value>> {
        match self {
            Value::Option(v) => Some(v.as_deref()),
            _ => None,
        }
    }

    /// Returns a mutable reference to the inner value if this is a [`Value::Option`], otherwise [`None`].
    #[allow(clippy::type_complexity)]
    pub fn as_option_mut(&mut self) -> Option<Option<&mut Value>> {
        match self {
            Value::Option(v) => Some(v.as_deref_mut()),
            _ => None,
        }
    }

    /// Returns a reference to the wrapped value if this is a [`Value::Newtype`], otherwise [`None`].
    pub fn as_newtype(&self) -> Option<&Value> {
        match self {
            Value::Newtype(v) => Some(v),
            _ => None,
        }
    }

    /// Returns a mutable reference to the wrapped value if this is a [`Value::Newtype`], otherwise [`None`].
    pub fn as_newtype_mut(&mut self) -> Option<&mut Value> {
        match self {
            Value::Newtype(v) => Some(v),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn from_scalars() {
        assert_eq!(Value::from(1u8), Value::U8(1));
        assert_eq!(Value::from(true), Value::Bool(true));
        assert_eq!(Value::from('a'), Value::Char('a'));
    }

    #[test]
    fn from_tuple_single() {
        assert_eq!(Value::from((Value::U8(1),)), Value::Seq(vec![Value::U8(1)]));
    }

    #[test]
    fn from_tuple_mixed() {
        assert_eq!(
            Value::from((Value::U8(1), Value::Char('a'), Value::Bool(true))),
            Value::Seq(vec![Value::U8(1), Value::Char('a'), Value::Bool(true)])
        );
    }

    #[test]
    fn from_tuple_max() {
        let tuple = (
            Value::U8(0),
            Value::U8(1),
            Value::U8(2),
            Value::U8(3),
            Value::U8(4),
            Value::U8(5),
            Value::U8(6),
            Value::U8(7),
        );
        assert_eq!(
            Value::from(tuple),
            Value::Seq(vec![
                Value::U8(0),
                Value::U8(1),
                Value::U8(2),
                Value::U8(3),
                Value::U8(4),
                Value::U8(5),
                Value::U8(6),
                Value::U8(7),
            ])
        );
    }

    #[test]
    fn as_copy_scalars() {
        assert_eq!(Value::U8(1).as_u8(), Some(1));
        assert_eq!(Value::Bool(true).as_bool(), Some(true));
        assert_eq!(Value::Char('a').as_char(), Some('a'));
        // Wrong variant returns None.
        assert_eq!(Value::U8(1).as_bool(), None);
        assert_eq!(Value::Unit.as_u8(), None);
    }

    #[test]
    fn u128_i128_roundtrip() {
        use serde::Deserialize;

        let big_u = u128::MAX;
        let big_i = i128::MIN;
        assert_eq!(Value::from(big_u), Value::U128(big_u));
        assert_eq!(Value::from(big_i), Value::I128(big_i));
        assert_eq!(Value::U128(big_u).as_u128(), Some(big_u));
        assert_eq!(Value::I128(big_i).as_i128(), Some(big_i));
        assert_eq!(Value::U128(big_u).as_u64(), None);

        // serde roundtrip through to_value / deserialize
        let v = crate::to_value(big_u).unwrap();
        assert_eq!(v, Value::U128(big_u));
        assert_eq!(u128::deserialize(v).unwrap(), big_u);

        let v = crate::to_value(big_i).unwrap();
        assert_eq!(v, Value::I128(big_i));
        assert_eq!(i128::deserialize(v).unwrap(), big_i);
    }

    #[test]
    fn as_str_and_string_mut() {
        let mut v = Value::String("hi".into());
        assert_eq!(v.as_str(), Some("hi"));
        assert_eq!(v.as_string().map(String::as_str), Some("hi"));
        v.as_string_mut().unwrap().push_str(" there");
        assert_eq!(v.as_str(), Some("hi there"));
        assert_eq!(Value::Unit.as_str(), None);
    }

    #[test]
    fn as_seq_mut() {
        let mut v = Value::Seq(vec![Value::U8(1)]);
        v.as_seq_mut().unwrap().push(Value::U8(2));
        assert_eq!(v.as_seq(), Some(&vec![Value::U8(1), Value::U8(2)]));
        assert_eq!(Value::Unit.as_seq(), None);
    }

    #[test]
    fn as_bytes_mut() {
        let mut v = Value::Bytes(vec![1, 2]);
        v.as_bytes_mut().unwrap().push(3);
        assert_eq!(v.as_bytes(), Some(&vec![1u8, 2, 3]));
    }

    #[test]
    fn as_unit() {
        assert_eq!(Value::Unit.as_unit(), Some(()));
        assert_eq!(Value::U8(1).as_unit(), None);
    }

    #[test]
    fn as_option() {
        let v = Value::Option(Some(Box::new(Value::U8(1))));
        assert_eq!(v.as_option(), Some(Some(&Value::U8(1))));

        let empty = Value::Option(None);
        assert_eq!(empty.as_option(), Some(None));

        assert_eq!(Value::Unit.as_option(), None);

        let mut v = Value::Option(Some(Box::new(Value::U8(1))));
        if let Some(Some(inner)) = v.as_option_mut() {
            *inner = Value::U8(2);
        }
        assert_eq!(v.as_option(), Some(Some(&Value::U8(2))));
    }

    #[test]
    fn as_newtype() {
        let mut v = Value::newtype(Value::U8(1));
        assert_eq!(v.as_newtype(), Some(&Value::U8(1)));
        *v.as_newtype_mut().unwrap() = Value::U8(2);
        assert_eq!(v.as_newtype(), Some(&Value::U8(2)));
        assert_eq!(Value::Unit.as_newtype(), None);
    }
}
