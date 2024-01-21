#![allow(unused)]

use asn1rs::prelude::basic::DER;
pub use asn1rs::prelude::*;

pub fn serialize_uper(to_uper: &impl Writable) -> (usize, Vec<u8>) {
    let mut writer = UperWriter::default();
    writer.write(to_uper).unwrap();
    let bits = writer.bit_len();
    (bits, writer.into_bytes_vec())
}

pub fn deserialize_uper<T: Readable>(data: &[u8], bits: usize) -> T {
    let mut reader = UperReader::from((data, bits));
    let result = reader.read::<T>().unwrap();
    assert_eq!(
        0,
        reader.bits_remaining(),
        "After reading, there are still bits remaining!"
    );
    result
}

pub fn serialize_and_deserialize_uper<T: Readable + Writable + std::fmt::Debug + PartialEq>(
    bits: usize,
    data: &[u8],
    uper: &T,
) {
    let serialized = serialize_uper(uper);
    assert_eq!(
        (bits, data),
        (serialized.0, &serialized.1[..]),
        "Serialized binary data does not match, bad-hex: {:02x?}",
        &serialized.1[..]
    );
    assert_eq!(
        uper,
        &deserialize_uper::<T>(data, bits),
        "Deserialized data struct does not match"
    );
}

pub fn serialize_der(to_der: &impl Writable) -> Vec<u8> {
    let mut writer = DER::writer(Vec::new());
    writer.write(to_der).unwrap();
    writer.into_inner()
}

pub fn deserialize_der<T: Readable>(data: &[u8]) -> T {
    let mut reader = DER::reader(data);
    let result = reader.read::<T>().unwrap();
    assert_eq!(
        0,
        reader.into_inner().len(),
        "After reading, there are still bytes remaining!"
    );
    result
}

pub fn serialize_and_deserialize_der<T: Readable + Writable + std::fmt::Debug + PartialEq>(
    data: &[u8],
    value: &T,
) {
    let serialized = serialize_der(value);
    assert_eq!(
        data,
        &serialized[..],
        "Serialized binary data does not match, bad-hex: {:02x?}",
        &serialized[..]
    );
    assert_eq!(
        value,
        &deserialize_der::<T>(data),
        "Deserialized data struct does not match"
    );
}

#[cfg(feature = "protobuf")]
pub fn serialize_protobuf(to_protobuf: &impl Writable) -> Vec<u8> {
    let mut writer = ProtobufWriter::default();
    writer.write(to_protobuf).unwrap();
    let vec = writer.into_bytes_vec();

    let mut vec2 = vec![0u8; vec.len()];
    let mut writer2 = ProtobufWriter::from(&mut vec2[..]);
    writer2.write(to_protobuf).unwrap();

    let len_written = writer2.len_written();
    let as_bytes_vec = writer2.as_bytes().to_vec();
    let into_bytes_vec = writer2.into_bytes_vec();

    assert_eq!(
        &vec[..],
        &vec2[..],
        "ProtobufWriter output differs between Vec<u8> and &mut [u8] backend"
    );

    assert_eq!(
        &vec[..],
        &as_bytes_vec[..],
        "ProtobufWriter::as_bytes returns wrong byte slice"
    );

    assert_eq!(
        &vec[..],
        &into_bytes_vec[..],
        "ProtobufWriter::into_bytes_vec returns wrong vec"
    );

    assert_eq!(
        vec.len(),
        len_written,
        "ProtobufWriter::len_written returns wrong value"
    );

    vec
}

#[cfg(feature = "protobuf")]
pub fn deserialize_protobuf<T: Readable>(data: &[u8]) -> T {
    let mut reader = ProtobufReader::from(data);
    T::read(&mut reader).unwrap()
}

#[cfg(feature = "protobuf")]
pub fn serialize_and_deserialize_protobuf<T: Readable + Writable + std::fmt::Debug + PartialEq>(
    data: &[u8],
    proto: &T,
) {
    let serialized = serialize_protobuf(proto);
    assert_eq!(
        data,
        &serialized[..],
        "Serialized binary data does not match"
    );

    assert_eq!(
        proto,
        &deserialize_protobuf::<T>(data),
        "Deserialized data struct does not match"
    );
}
