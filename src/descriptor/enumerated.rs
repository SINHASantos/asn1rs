use crate::descriptor::{ReadableType, Reader, WritableType, Writer};
use core::marker::PhantomData;

pub struct Enumerated<C: Constraint>(PhantomData<C>);

pub trait Constraint: super::common::Constraint + Sized {
    const NAME: &'static str;
    const VARIANT_COUNT: u64;
    const STD_VARIANT_COUNT: u64;
    const EXTENSIBLE: bool = false;

    fn to_choice_index(&self) -> u64;

    fn from_choice_index(index: u64) -> Option<Self>;
}

impl<C: Constraint> WritableType for Enumerated<C> {
    type Type = C;

    #[inline]
    fn write_value<W: Writer>(
        writer: &mut W,
        value: &Self::Type,
    ) -> Result<(), <W as Writer>::Error> {
        writer.write_enumerated(value)
    }
}

impl<C: Constraint> ReadableType for Enumerated<C> {
    type Type = C;

    #[inline]
    fn read_value<R: Reader>(reader: &mut R) -> Result<Self::Type, <R as Reader>::Error> {
        reader.read_enumerated::<Self::Type>()
    }
}
