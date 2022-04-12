use serde::de::{Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use serde_bytes::ByteBuf;
use serde_cbor::tags::Tagged;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DateT(pub(crate) String);

impl Serialize for DateT {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        Tagged::new(Some(0), &self.0).serialize(s)
    }
}

impl<'de> Deserialize<'de> for DateT {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let tagged = Tagged::<String>::deserialize(deserializer)?;
        match tagged.tag {
            Some(0) | None => Ok(DateT(tagged.value)),
            Some(_) => Err(serde::de::Error::custom("unexpected tag")),
        }
    }
}

impl fmt::Display for DateT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct UriT(pub(crate) String);

impl Serialize for UriT {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        Tagged::new(Some(32), &self.0).serialize(s)
    }
}

impl<'de> Deserialize<'de> for UriT {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let tagged = Tagged::<String>::deserialize(deserializer)?;
        match tagged.tag {
            // allow deserialization even if there is no tag. Allows roundtrip via other formats such as json
            Some(32) | None => Ok(UriT(tagged.value)),
            Some(_) => Err(serde::de::Error::custom("unexpected tag")),
        }
    }
}

impl fmt::Display for UriT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct BytesT(pub(crate) Vec<u8>);

impl Serialize for BytesT {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        Tagged::new(Some(64), &ByteBuf::from(self.0.clone())).serialize(s)
    }
}

impl<'de> Deserialize<'de> for BytesT {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let tagged = Tagged::<ByteBuf>::deserialize(deserializer)?;
        match tagged.tag {
            Some(64) | None => Ok(BytesT(tagged.value.to_vec())),
            Some(_) => Err(serde::de::Error::custom("unexpected tag")),
        }
    }
}

impl fmt::Display for BytesT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            format!("{:02x?}", &self.0.to_vec()).replace(',', "")
        )
    }
}
