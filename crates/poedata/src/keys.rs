use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::marker::PhantomData;

#[derive(Deserialize, Serialize)]
#[serde(transparent)]
pub struct Key<T> {
    pub key: u32,
    #[serde(skip_deserializing)]
    pub key_type: PhantomData<T>,
}

pub type Id = String;

impl<T> Clone for Key<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Key<T> {}

impl<T> Hash for Key<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

impl<T> PartialEq<Self> for Key<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<T> Eq for Key<T> {}

impl<T> PartialOrd for Key<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl<T> Ord for Key<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

impl<T> Display for Key<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.key))
    }
}

impl<T> Debug for Key<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.key))
    }
}

impl<T> From<u32> for Key<T> {
    fn from(key: u32) -> Self {
        Self {
            key: key,
            key_type: PhantomData,
        }
    }
}

impl<T> Key<T> {
    pub fn new(key: u32) -> Self {
        Self {
            key: key,
            key_type: PhantomData,
        }
    }
}

pub trait HasKey<T> {
    fn key(&self) -> &Key<T>;
}

pub trait HasId {
    fn id(&self) -> &Id;
}

macro_rules! impl_key {
    ($struct_name:ident) => {
        impl HasKey<$struct_name> for $struct_name {
            fn key(&self) -> &Key<$struct_name> {
                &self.key
            }
        }
    };
}

macro_rules! impl_id {
    ($struct_name:ident) => {
        impl HasId for $struct_name {
            fn id(&self) -> &Id {
                &self.id
            }
        }
    };
}

pub(crate) use impl_id;
pub(crate) use impl_key;
