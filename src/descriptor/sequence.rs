use crate::descriptor::{ReadableType, Reader, WritableType, Writer};
use core::marker::PhantomData;

pub struct Sequence<T: Constraint>(PhantomData<T>);

pub trait Constraint: super::common::Constraint {
    const NAME: &'static str;
    const STD_OPTIONAL_FIELDS: u64;
    const FIELD_COUNT: u64;
    const EXTENDED_AFTER_FIELD: Option<u64>;

    fn read_seq<R: Reader>(reader: &mut R) -> Result<Self, R::Error>
    where
        Self: Sized;

    fn write_seq<W: Writer>(&self, writer: &mut W) -> Result<(), W::Error>;
}

impl<C: Constraint> WritableType for Sequence<C> {
    type Type = C;

    #[inline]
    fn write_value<W: Writer>(
        writer: &mut W,
        value: &Self::Type,
    ) -> Result<(), <W as Writer>::Error> {
        writer.write_sequence::<C, _>(|w| value.write_seq::<W>(w))
    }
}

impl<C: Constraint> ReadableType for Sequence<C>
where
    C: Sized,
{
    type Type = C;

    #[inline]
    fn read_value<R: Reader>(reader: &mut R) -> Result<Self::Type, <R as Reader>::Error> {
        reader.read_sequence::<C, Self::Type, _>(C::read_seq)
    }
}
