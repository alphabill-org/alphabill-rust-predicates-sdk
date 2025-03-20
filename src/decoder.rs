/*!
Helper to decode data sent from host.
*/

extern crate alloc;
use alloc::{string::String, vec::Vec};

use crate::{
    error::{self, Error},
    evaluation_ctx::{self, ABHandle},
    memory,
};

#[derive(Copy, Clone)]
pub struct Decoder<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Decoder<'_> {
    /// create new decoder with given byte slice as input data.
    pub fn new(data: &'a [u8]) -> Decoder<'a> {
        Decoder { data, pos: 0 }
    }

    pub fn from_handle(h: ABHandle) -> Decoder<'a> {
        let ptr = evaluation_ctx::serialize_obj(h, 0);
        let data = memory::byte_array(ptr);
        Decoder::new(data)
    }

    /// read u32 value from data stream.
    pub fn uint32(&mut self) -> u32 {
        let sp = self.pos;
        self.pos += 4;
        u32::from_le_bytes(self.data[sp..self.pos].try_into().unwrap())
    }

    /// read u64 value from data stream.
    pub fn uint64(&mut self) -> u64 {
        let sp = self.pos;
        self.pos += 8;
        u64::from_le_bytes(self.data[sp..self.pos].try_into().unwrap())
    }

    /// read string value from data stream.
    pub fn string(&mut self) -> String {
        let len = self.uint32();
        if len == 0 {
            return String::new();
        }
        let sp = self.pos;
        self.pos += len as usize;
        String::from_utf8(self.data[sp..self.pos].to_vec()).unwrap()
    }

    pub fn bytes(&mut self) -> Vec<u8> {
        let len = self.uint32();
        if len == 0 {
            return Vec::new();
        }
        let sp = self.pos;
        self.pos += len as usize;
        let mut b = Vec::new();
        b.extend_from_slice(&self.data[sp..self.pos]);
        b
    }

    pub fn value(&mut self) -> Value {
        let type_id = self.data[self.pos];
        self.pos += 1;
        match type_id {
            1 => Value::Bytes(self.bytes()),
            2 => Value::U64(self.uint64()),
            3 => Value::U32(self.uint32()),
            4 => Value::String(self.string()),
            5 => {
                let cnt = self.uint32();
                let mut a = Vec::<Value>::with_capacity(cnt as usize);
                for _ in 0..cnt {
                    let v = self.value();
                    a.push(v);
                }
                Value::Array(a)
            }
            _ => Value::Error(1),
        }
    }

    /// read current position in the data as "tag" value
    fn tag(&mut self) -> u8 {
        let tag = self.data[self.pos];
        self.pos += 1;
        tag
    }
}

pub struct TagValueIter<'a> {
    dec: Decoder<'a>,
}

impl<'a> TagValueIter<'_> {
    pub fn new(data: &'a [u8]) -> TagValueIter<'a> {
        TagValueIter {
            dec: Decoder::new(data),
        }
    }
}

impl<'a> Iterator for TagValueIter<'a> {
    type Item = (u8, Value);

    fn next(&mut self) -> Option<Self::Item> {
        if self.dec.pos >= self.dec.data.len() {
            None
        } else {
            Some((self.dec.tag(), self.dec.value()))
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Value {
    U64(u64),
    U32(u32),
    Bytes(Vec<u8>),
    String(String),
    Array(Vec<Value>),
    Error(u8), // error code
}

impl TryFrom<Value> for u32 {
    type Error = error::Error;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::U32(n) => Ok(n),
            _ => Err(Error::new(1)),
        }
    }
}

impl TryFrom<Value> for u64 {
    type Error = error::Error;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::U32(n) => Ok(n.into()),
            Value::U64(n) => Ok(n),
            _ => Err(Error::new(1)),
        }
    }
}
impl TryFrom<Value> for Option<u64> {
    type Error = error::Error;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match <Value as TryInto<u64>>::try_into(v) {
            Ok(n) => Ok(Some(n)),
            Err(err) => Err(err.chain(1)),
        }
    }
}

impl TryFrom<Value> for String {
    type Error = error::Error;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::String(s) => Ok(s),
            Value::Bytes(b) => Ok(unsafe { String::from_utf8_unchecked(b.to_vec()) }),
            _ => Err(Error::new(1)),
        }
    }
}
impl TryFrom<Value> for Option<String> {
    type Error = error::Error;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::String(s) => Ok(Some(s)),
            Value::Bytes(b) => Ok(Some(unsafe { String::from_utf8_unchecked(b.to_vec()) })),
            _ => Err(Error::new(1)),
        }
    }
}

impl<'a> TryFrom<Value> for Vec<u8> {
    type Error = error::Error;

    fn try_from<'b>(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::String(s) => Ok(s.into_bytes()),
            Value::Bytes(b) => Ok(b),
            _ => Err(Error::new(1)),
        }
    }
}
impl<'a> TryFrom<Value> for Option<Vec<u8>> {
    type Error = error::Error;

    fn try_from<'b>(v: Value) -> Result<Self, Self::Error> {
        match v {
            Value::String(s) => Ok(Some(s.into_bytes())),
            Value::Bytes(b) => Ok(Some(b)),
            _ => Err(Error::new(1)),
        }
    }
}

#[cfg(test)]
mod tests {

    use alloc::{string::ToString, vec};

    use super::*;

    #[test]
    fn decode_value() {
        // test generated by Go backend!
        // Go value uint16 = 0x201
        let data: &mut [u8] = &mut [0x3, 0x1, 0x2, 0x0, 0x0];
        let mut dec = Decoder::new(data);
        assert_eq!(dec.value(), Value::U32(0x201));

        // Go value uint32 = 0x4030201
        let data: &mut [u8] = &mut [0x3, 0x1, 0x2, 0x3, 0x4];
        let mut dec = Decoder::new(data);
        assert_eq!(dec.value(), Value::U32(0x4030201));

        // Go value uint64 = 0x807060504030201
        let data: &mut [u8] = &mut [0x2, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8];
        let mut dec = Decoder::new(data);
        assert_eq!(dec.value(), Value::U64(0x807060504030201));

        // Go value []uint8 = []byte{0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa}
        let data: &mut [u8] = &mut [
            0x1, 0xb, 0x0, 0x0, 0x0, 0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa,
        ];
        let mut dec = Decoder::new(data);
        assert_eq!(
            dec.value(),
            Value::Bytes(vec![0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa])
        );

        // Go value string = "this is string"
        let data: &mut [u8] = &mut [
            0x4, 0xe, 0x0, 0x0, 0x0, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x73, 0x74,
            0x72, 0x69, 0x6e, 0x67,
        ];
        let mut dec = Decoder::new(data);
        assert_eq!(dec.value(), Value::String("this is string".to_string()));

        // Go value []interface {} = []interface {}{0x20, 0x40, "AB"}
        let data: &mut [u8] = &mut [
            0x5, 0x3, 0x0, 0x0, 0x0, 0x3, 0x20, 0x0, 0x0, 0x0, 0x2, 0x40, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x4, 0x2, 0x0, 0x0, 0x0, 0x41, 0x42,
        ];
        let mut dec = Decoder::new(data);
        assert_eq!(
            dec.value(),
            Value::Array(vec![
                Value::U32(0x20),
                Value::U64(0x40),
                Value::String("AB".to_string())
            ])
        );

        // Go value string = ""
        let data: &mut [u8] = &mut [0x4, 0x0, 0x0, 0x0, 0x0];
        let mut dec = Decoder::new(data);
        assert_eq!(dec.value(), Value::String("".to_string()));

        // Go value []uint8 = []byte(nil)
        let data: &mut [u8] = &mut [0x1, 0x0, 0x0, 0x0, 0x0];
        let mut dec = Decoder::new(data);
        assert_eq!(dec.value(), Value::Bytes(vec![]));

        // Go value []interface {} = []interface {}{}
        let data: &mut [u8] = &mut [0x5, 0x0, 0x0, 0x0, 0x0];
        let mut dec = Decoder::new(data);
        assert_eq!(dec.value(), Value::Array(vec![]));
    }

    #[test]
    fn iterator() {
        // test generated by Go backend!
        let data: &mut [u8] = &mut [
            0x1, 0x3, 0x0, 0xff, 0x0, 0xff, 0x4, 0x2, 0x0, 0xff, 0x0, 0xff, 0x0, 0x0, 0x0, 0x0,
            0x2, 0x4, 0x5, 0x0, 0x0, 0x0, 0x74, 0x6f, 0x6b, 0x65, 0x6e,
        ];
        let dec = TagValueIter::new(&data);
        let items: Vec<(u8, Value)> = dec.collect();
        assert_eq!(
            items,
            vec![
                (1, Value::U32(0xff00ff00)),
                (4, Value::U64(0xff00ff00)),
                (2, Value::String("token".to_string()))
            ]
        );
    }
}
