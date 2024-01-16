use crate::syn::{ReadableType, Reader, WritableType, Writer};
use asn1rs_model::asn::Tag;
use core::marker::PhantomData;

pub struct SequenceOf<T, C: Constraint = NoConstraint>(PhantomData<T>, PhantomData<C>);

pub trait Constraint: super::common::Constraint {
    const MIN: Option<u64> = None;
    const MAX: Option<u64> = None;
    const EXTENSIBLE: bool = false;
}

#[derive(Default)]
pub struct NoConstraint;
impl super::common::Constraint for NoConstraint {
    const TAG: Tag = Tag::DEFAULT_SEQUENCE_OF;
}
impl Constraint for NoConstraint {}

impl<T: WritableType, C: Constraint> WritableType for SequenceOf<T, C> {
    type Type = Vec<T::Type>;

    #[inline]
    fn write_value<W: Writer>(writer: &mut W, value: &Self::Type) -> Result<(), W::Error> {
        writer.write_sequence_of::<C, T>(value.as_slice())
    }
}

impl<T: ReadableType, C: Constraint> ReadableType for SequenceOf<T, C> {
    type Type = Vec<T::Type>;

    #[inline]
    fn read_value<R: Reader>(reader: &mut R) -> Result<Self::Type, <R as Reader>::Error> {
        reader.read_sequence_of::<C, T>()
    }
}
