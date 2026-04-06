use std::{collections::HashMap, fs::File, io::BufReader, process::Command};

use poedata::{
    data::Data,
    keys::{HasKey, Key},
    structs::{
        BaseItemType, Essence, ItemClass, ItemClassCategory, Mod, ModFamily, ModType, Stat, Tag,
    },
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_aux::prelude::serde_introspect;

use which::which;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaColumn {
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaTable {
    pub name: String,
    pub columns: Vec<SchemaColumn>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatSchema {
    pub version: u32,
    pub created_at: u32,
    pub tables: Vec<SchemaTable>,
}

#[derive(Serialize, Deserialize)]
pub struct ViewerTable {
    pub name: String,
    pub columns: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ViewerConfig {
    pub patch: String,
    pub files: Vec<String>,
    pub translations: Vec<String>,
    pub tables: Vec<ViewerTable>,
}

pub struct PoEDatViewer {
    pub data_dir: String,
    pub patch: String,
}

const LATEST_PATCH_URL: &str =
    "https://raw.githubusercontent.com/poe-tool-dev/latest-patch-version/main/latest.txt";
const SCHEMA_URL: &str =
    "https://github.com/poe-tool-dev/dat-schema/releases/download/latest/schema.min.json";

pub fn read_from_json<T: DeserializeOwned>(path: &str) -> anyhow::Result<T> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let u = serde_json::from_reader(reader)?;

    Ok(u)
}

#[allow(unused)]
fn write_to_json_pretty<T: Serialize>(path: &str, data: &T) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(data)?;
    Ok(std::fs::write(path, json)?)
}

#[allow(unused)]
fn read_table_vec<T: DeserializeOwned + HasKey<K>, K>(data_dir: &str, name: &str) -> Vec<T> {
    println!("Loading {}", name);
    read_from_json::<Vec<T>>(&format!("{}/tables/English/{}.json", data_dir, name)).unwrap()
}

#[allow(unused)]
fn read_table_hashmap<T: DeserializeOwned + HasKey<K>, K>(
    data_dir: &str,
    name: &str,
) -> HashMap<Key<K>, T> {
    println!("Loading {}", name);
    read_from_json::<Vec<T>>(&format!("{}/tables/English/{}.json", data_dir, name))
        .unwrap()
        .into_iter()
        .map(|a| (*a.key(), a))
        .collect()
}

#[allow(unused)]
fn fields<T: DeserializeOwned>() -> Vec<String> {
    serde_introspect::<T>()
        .into_iter()
        .copied()
        .map(ToOwned::to_owned)
        .filter(|k| *k != "_index")
        .collect()
}

impl PoEDatViewer {
    pub fn new(data_dir: &str) -> Self {
        Self {
            data_dir: data_dir.to_owned(),
            patch: Self::get_latest_version(),
        }
    }

    pub fn fetch_game_data(&self) {
        self.write_config_for_dat_viewer();

        let mut cmd = Command::new(which("pathofexile-dat").unwrap());

        cmd.current_dir(format!("{}/", self.data_dir));

        println!("Running pathofexile-dat...");

        cmd.output().expect("Failed");
    }

    #[allow(unused)]
    pub fn get_latest_version() -> String {
        reqwest::blocking::get(LATEST_PATCH_URL)
            .unwrap()
            .text()
            .unwrap()
    }

    #[allow(unused)]
    pub fn get_files_schema() -> DatSchema {
        reqwest::blocking::get(SCHEMA_URL).unwrap().json().unwrap()
    }

    fn write_config_for_dat_viewer(&self) {
        let mut tables = HashMap::new();
        tables.extend(
            [
                ("BaseItemTypes", fields::<BaseItemType>()),
                ("ItemClasses", fields::<ItemClass>()),
                ("ItemClassCategories", fields::<ItemClassCategory>()),
                ("Essences", fields::<Essence>()),
                ("Mods", fields::<Mod>()),
                ("ModType", fields::<ModType>()),
                ("ModFamily", fields::<ModFamily>()),
                ("Stats", fields::<Stat>()),
                ("Tags", fields::<Tag>()),
            ]
            .into_iter(),
        );

        let fetch_tables: Vec<_> = tables
            .into_iter()
            .map(|(name, v)| ViewerTable {
                name: name.to_owned(),
                columns: v,
            })
            .collect();

        write_to_json_pretty(
            &format!("{}/config.json", self.data_dir),
            &ViewerConfig {
                patch: self.patch.clone(),
                files: vec![
                    "Metadata/StatDescriptions/advanced_mod_stat_descriptions.txt".to_owned(),
                    "Metadata/StatDescriptions/map_stat_descriptions.txt".to_owned(),
                    "Metadata/StatDescriptions/stat_descriptions.txt".to_owned(),
                    "Metadata/StatDescriptions/atlas_stat_descriptions.txt".to_owned(),
                ],
                translations: vec!["English".to_owned()],
                tables: fetch_tables,
            },
        )
        .expect("File written");
    }

    fn load(&self, patch: &str) -> Data {
        let base_item_types = read_table_hashmap(&self.data_dir, "BaseItemTypes");
        let essences = read_table_hashmap(&self.data_dir, "Essences");
        let mods = read_table_vec(&self.data_dir, "Mods").as_slice().into();
        let mod_types = read_table_hashmap(&self.data_dir, "ModType");
        let mod_families = read_table_vec(&self.data_dir, "ModFamily")
            .as_slice()
            .into();
        let item_classes = read_table_hashmap(&self.data_dir, "ItemClasses");
        let item_class_categories = read_table_hashmap(&self.data_dir, "ItemClassCategories");
        let stats: HashMap<Key<Stat>, Stat> = read_table_hashmap(&self.data_dir, "Stats");
        let tags = read_table_vec(&self.data_dir, "Tags").as_slice().into();

        Data {
            version: patch.to_owned(),
            base_item_types: base_item_types,
            essences: essences,
            item_classes: item_classes,
            item_class_categories: item_class_categories,
            mods: mods,
            mod_types: mod_types,
            mod_families: mod_families,
            stats: stats,
            tags: tags,
        }
    }

    pub fn write_cbor(&self, path: &str) {
        let patch = read_from_json::<ViewerConfig>(&format!("{}/config.json", self.data_dir))
            .unwrap()
            .patch;
        let data = self.load(&patch);
        let file_path = format!("{}/poedata.cbor", path);
        let file = File::create(&file_path).unwrap();
        println!("Writing {}", file_path);
        serde_cbor_2::to_writer(file, &data).unwrap();
    }
}
