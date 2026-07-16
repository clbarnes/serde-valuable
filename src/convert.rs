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
    i8 => I8,
    i16 => I16,
    i32 => I32,
    i64 => I64,
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

impl Value {
    pub fn newtype(value: Value) -> Self {
        Value::Newtype(Box::new(value))
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
}
