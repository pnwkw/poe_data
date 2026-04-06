use std::sync::LazyLock;

use poedata::data::Data;

pub static DATA: LazyLock<Data> = LazyLock::new(|| {
    let bytes = include_bytes!("./poedata.cbor");
    serde_cbor_2::from_reader(&bytes[..]).unwrap()
});

#[cfg(test)]
mod tests {
    use crate::DATA;

    #[test]
    fn game_version() {
        assert_eq!(DATA.version, "3.28.0.7".to_owned());
    }
}
