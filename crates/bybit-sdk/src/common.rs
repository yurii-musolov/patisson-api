use serde::{Deserialize, Serialize};

#[inline]
pub fn deserialize_slice<'a, T>(message: &'a [u8]) -> serde_json::Result<T>
where
    T: ?Sized + Deserialize<'a>,
{
    serde_json::from_slice(message)
}

#[inline]
pub fn serialize<T>(msg: &T) -> serde_json::Result<String>
where
    T: ?Sized + Serialize,
{
    serde_json::to_string(msg)
}
