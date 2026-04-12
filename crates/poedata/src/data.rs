use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    keys::{HasId, HasKey, Id, Key},
    structs::{
        BaseItemType, Essence, ItemClass, ItemClassCategory, Mod, ModFamily, ModType, Stat, Tag,
    },
};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct TwoKeyHashMap<T: HasKey<T> + HasId> {
    pub items: Vec<T>,
    key_map: HashMap<Key<T>, usize>,
    id_map: HashMap<Id, usize>,
}

impl<T: Clone + HasKey<T> + HasId> TwoKeyHashMap<T> {
    pub fn get_by_key(&self, k: &Key<T>) -> Option<&T> {
        self.key_map.get(k).and_then(|v| self.items.get(*v))
    }

    pub fn get_by_id(&self, id: &str) -> Option<&T> {
        self.id_map.get(id).and_then(|v| self.items.get(*v))
    }
}

impl<T: Clone + HasKey<T> + HasId> From<Vec<T>> for TwoKeyHashMap<T> {
    fn from(items: Vec<T>) -> Self {
        TwoKeyHashMap {
            key_map: items
                .iter()
                .map(|elm| (*elm.key(), elm.key().key as usize))
                .collect(),
            id_map: items
                .iter()
                .map(|elm| (elm.id().clone(), elm.key().key as usize))
                .collect(),
            items: items.into(),
        }
    }
}

impl<T: Clone + HasKey<T> + HasId> FromIterator<T> for TwoKeyHashMap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let items: Vec<T> = iter.into_iter().collect();
        TwoKeyHashMap {
            key_map: items
                .iter()
                .map(|elm| (*elm.key(), elm.key().key as usize))
                .collect(),
            id_map: items
                .iter()
                .map(|elm| (elm.id().clone(), elm.key().key as usize))
                .collect(),
            items: items,
        }
    }
}

type DataHashMap<T> = HashMap<Key<T>, T>;

#[derive(Deserialize, Debug, Serialize)]
pub struct Data {
    pub version: String,
    pub base_item_types: DataHashMap<BaseItemType>,
    pub item_classes: DataHashMap<ItemClass>,
    pub item_class_categories: DataHashMap<ItemClassCategory>,
    pub essences: DataHashMap<Essence>,
    pub mods: TwoKeyHashMap<Mod>,
    pub mod_types: DataHashMap<ModType>,
    pub mod_families: TwoKeyHashMap<ModFamily>,
    pub tags: TwoKeyHashMap<Tag>,
    pub stats: DataHashMap<Stat>,
}
