use std::sync::LazyLock;

use poedata::data::Data;

pub static DATA: LazyLock<Data> = LazyLock::new(|| {
    let bytes = include_bytes!("./poedata.cbor");
    serde_cbor_2::from_reader(&bytes[..]).unwrap()
});
