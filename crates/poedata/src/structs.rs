use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{
    enums::{Domain, EssenceType, GenerationType, InfluenceType},
    keys::{HasId, HasKey, Id, Key, impl_id, impl_key},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BaseItemType {
    #[serde(rename = "_index")]
    pub key: Key<BaseItemType>,
    pub id: Id,
    pub item_classes_key: Key<ItemClass>,
    pub width: u32,
    pub height: u32,
    pub name: String,
    pub inherits_from: String,
    pub drop_level: u32,
    pub flavour_text_key: Option<Key<u32>>,
    #[serde(rename = "Implicit_ModsKeys")]
    pub implicit_mods_keys: HashSet<Key<Mod>>,
    pub size_on_ground: u32,
    pub sound_effect: Option<u32>,
    pub tags_keys: Vec<Key<Tag>>,
    pub mod_domain: Domain,
    pub item_visual_identity: u32,
    pub inflection: String,
    pub is_corrupted: bool,
    pub fragment_base_item_types_key: Option<Key<BaseItemType>>,
    pub unmodifiable: bool,
}

impl_key!(BaseItemType);
impl_id!(BaseItemType);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModType {
    #[serde(rename = "_index")]
    pub key: Key<ModType>,
    pub name: String,
    pub mod_sell_price_types_keys: HashSet<Key<u32>>,
}

impl_key!(ModType);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModFamily {
    #[serde(rename = "_index")]
    pub key: Key<ModFamily>,
    pub id: Id,
}

impl_key!(ModFamily);
impl_id!(ModFamily);

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Essence {
    #[serde(rename = "_index")]
    pub key: Key<Essence>,
    pub base_item_types_key: Key<BaseItemType>,
    pub drop_level: Vec<i32>,
    pub essence_type_key: EssenceType,
    pub level: u8,
    pub level2: Vec<u8>,
    #[serde(rename = "Helmet_ModsKey")]
    pub helmet_mods_key: Option<Key<Mod>>,
    #[serde(rename = "BodyArmour_ModsKey")]
    pub body_armour_mods_key: Option<Key<Mod>>,
    #[serde(rename = "Boots_ModsKey")]
    pub boots_mods_key: Option<Key<Mod>>,
    #[serde(rename = "Gloves_ModsKey")]
    pub gloves_mods_key: Option<Key<Mod>>,
    #[serde(rename = "Bow_ModsKey")]
    pub bow_mods_key: Option<Key<Mod>>,
    #[serde(rename = "Wand_ModsKey")]
    pub wand_mods_key: Option<Key<Mod>>,
    #[serde(rename = "Staff_ModsKey")]
    pub staff_mods_key: Option<Key<Mod>>,
    #[serde(rename = "TwoHandSword_ModsKey")]
    pub two_hand_sword_mods_key: Option<Key<Mod>>,
    #[serde(rename = "TwoHandAxe_ModsKey")]
    pub two_hand_axe_mods_key: Option<Key<Mod>>,
    #[serde(rename = "TwoHandMace_ModsKey")]
    pub two_hand_mace_mods_key: Option<Key<Mod>>,
    #[serde(rename = "Claw_ModsKey")]
    pub claw_mods_key: Option<Key<Mod>>,
    #[serde(rename = "Dagger_ModsKey")]
    pub dagger_mods_key: Option<Key<Mod>>,
    #[serde(rename = "OneHandSword_ModsKey")]
    pub one_hand_sword_mods_key: Option<Key<Mod>>,
    #[serde(rename = "OneHandThrustingSword_ModsKey")]
    pub one_hand_thrusting_sword_mods_key: Option<Key<Mod>>,
    #[serde(rename = "OneHandAxe_ModsKey")]
    pub one_hand_axe_mods_key: Option<Key<Mod>>,
    #[serde(rename = "OneHandMace_ModsKey")]
    pub one_hand_mace_mods_key: Option<Key<Mod>>,
    #[serde(rename = "Sceptre_ModsKey")]
    pub sceptre_mods_key: Option<Key<Mod>>,
    pub item_level_restriction: u32,
    #[serde(rename = "Belt_ModsKey")]
    pub belt_mods_key: Option<Key<Mod>>,
    #[serde(rename = "Amulet_ModsKey")]
    pub amulet_mods_key: Option<Key<Mod>>,
    #[serde(rename = "Ring_ModsKey")]
    pub ring_mods_key: Option<Key<Mod>>,
    #[serde(rename = "Shield_ModsKey")]
    pub shield_mods_key: Option<Key<Mod>>,
    #[serde(rename = "Display_Quiver_ModsKey")]
    pub display_quiver_mods_key: Option<Key<Mod>>,
    pub is_screaming_essence: bool,
}

impl_key!(Essence);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ModStatEntry {
    pub key: Option<Key<Stat>>,
    pub min_value: i32,
    pub max_value: i32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RawMod {
    #[serde(rename = "_index")]
    pub key: Key<RawMod>,
    pub id: Id,
    pub mod_type_key: Key<ModType>,
    pub level: u16,
    pub stats_key_1: Option<Key<Stat>>,
    pub stats_key_2: Option<Key<Stat>>,
    pub stats_key_3: Option<Key<Stat>>,
    pub stats_key_4: Option<Key<Stat>>,
    pub domain: Domain,
    pub name: String,
    pub generation_type: GenerationType,
    pub families: HashSet<Key<ModFamily>>,
    pub stat_1_min: i32,
    pub stat_1_max: i32,
    pub stat_2_min: i32,
    pub stat_2_max: i32,
    pub stat_3_min: i32,
    pub stat_3_max: i32,
    pub stat_4_min: i32,
    pub stat_4_max: i32,
    #[serde(rename = "SpawnWeight_TagsKeys")]
    pub spawn_weight_tags_keys: Vec<Key<Tag>>,
    #[serde(rename = "SpawnWeight_Values")]
    pub spawn_weight_values: Vec<u32>,
    pub tags_keys: HashSet<Key<Tag>>,
    pub granted_effects_per_level_keys: HashSet<Key<u32>>,
    pub stat_5_min: i32,
    pub stat_5_max: i32,
    pub stats_key_5: Option<Key<Stat>>,
    #[serde(rename = "GenerationWeight_TagsKeys")]
    pub generation_weight_tags_keys: Vec<Key<Tag>>,
    #[serde(rename = "GenerationWeight_Values")]
    pub generation_weight_values: Vec<u32>,
    pub is_essence_only_modifier: bool,
    pub stat_6_min: i32,
    pub stat_6_max: i32,
    pub stats_key_6: Option<Key<Stat>>,
    pub max_level: u32,
    pub crafting_item_class_restrictions: Vec<Key<ItemClass>>,
    #[serde(rename = "Heist_SubStatValue1")]
    pub heist_sub_stat_value_1: i32,
    #[serde(rename = "Heist_SubStatValue2")]
    pub heist_sub_stat_value_2: i32,
    #[serde(rename = "Heist_StatsKey0")]
    pub heist_stats_key_0: Option<Key<Stat>>,
    #[serde(rename = "Heist_StatsKey1")]
    pub heist_stats_key_1: Option<Key<Stat>>,
    #[serde(rename = "Heist_AddStatValue1")]
    pub heist_add_stat_value_1: i32,
    #[serde(rename = "Heist_AddStatValue2")]
    pub heist_add_stat_value_2: i32,
    pub influence_types: InfluenceType,
    pub implicit_tags_keys: HashSet<Key<Tag>>,
    pub game_mode: i32,
}

impl_key!(RawMod);
impl_id!(RawMod);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Mod {
    pub key: Key<Mod>,
    pub id: Id,
    pub mod_type_key: Key<ModType>,
    pub level: u16,
    pub domain: Domain,
    pub name: String,
    pub generation_type: GenerationType,
    pub families: HashSet<Key<ModFamily>>,
    pub mod_stats: [ModStatEntry; 6],
    pub heist_stats: [ModStatEntry; 2],
    pub spawn_weight_tags_keys: Vec<Key<Tag>>,
    pub spawn_weight_values: Vec<u32>,
    pub tags_keys: HashSet<Key<Tag>>,
    pub granted_effects_per_level_keys: HashSet<Key<u32>>,
    pub generation_weight_tags_keys: Vec<Key<Tag>>,
    pub generation_weight_values: Vec<u32>,
    pub is_essence_only_modifier: bool,
    pub max_level: u32,
    pub crafting_item_class_restrictions: Vec<Key<ItemClass>>,
    pub influence_types: InfluenceType,
    pub implicit_tags_keys: HashSet<Key<Tag>>,
    pub game_mode: i32,
}

impl From<RawMod> for Mod {
    fn from(rm: RawMod) -> Self {
        let mod_stats = [
            ModStatEntry {
                key: rm.stats_key_1,
                min_value: rm.stat_1_min,
                max_value: rm.stat_1_max,
            },
            ModStatEntry {
                key: rm.stats_key_2,
                min_value: rm.stat_2_min,
                max_value: rm.stat_2_max,
            },
            ModStatEntry {
                key: rm.stats_key_3,
                min_value: rm.stat_3_min,
                max_value: rm.stat_3_max,
            },
            ModStatEntry {
                key: rm.stats_key_4,
                min_value: rm.stat_4_min,
                max_value: rm.stat_4_max,
            },
            ModStatEntry {
                key: rm.stats_key_5,
                min_value: rm.stat_5_min,
                max_value: rm.stat_5_max,
            },
            ModStatEntry {
                key: rm.stats_key_6,
                min_value: rm.stat_6_min,
                max_value: rm.stat_6_max,
            },
        ];

        let heist_stats = [
            ModStatEntry {
                key: rm.heist_stats_key_0,
                min_value: rm.heist_sub_stat_value_1,
                max_value: rm.heist_add_stat_value_1,
            },
            ModStatEntry {
                key: rm.heist_stats_key_1,
                min_value: rm.heist_sub_stat_value_2,
                max_value: rm.heist_add_stat_value_2,
            },
        ];

        Self {
            key: rm.key.key.into(),
            id: rm.id,
            mod_type_key: rm.mod_type_key,
            level: rm.level,
            domain: rm.domain,
            name: rm.name,
            generation_type: rm.generation_type,
            families: rm.families,
            mod_stats: mod_stats,
            heist_stats: heist_stats,
            spawn_weight_tags_keys: rm.spawn_weight_tags_keys,
            spawn_weight_values: rm.spawn_weight_values,
            tags_keys: rm.tags_keys,
            granted_effects_per_level_keys: rm.granted_effects_per_level_keys,
            generation_weight_tags_keys: rm.generation_weight_tags_keys,
            generation_weight_values: rm.generation_weight_values,
            is_essence_only_modifier: rm.is_essence_only_modifier,
            max_level: rm.max_level,
            crafting_item_class_restrictions: rm.crafting_item_class_restrictions,
            influence_types: rm.influence_types,
            implicit_tags_keys: rm.implicit_tags_keys,
            game_mode: rm.game_mode,
        }
    }
}

impl_key!(Mod);
impl_id!(Mod);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemClassCategory {
    #[serde(rename = "_index")]
    pub key: Key<ItemClassCategory>,
    pub id: Id,
    pub text: String,
}

impl_key!(ItemClassCategory);
impl_id!(ItemClassCategory);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemClass {
    #[serde(rename = "_index")]
    pub key: Key<ItemClass>,
    pub id: Id,
    pub name: String,
    pub item_class_category: Option<Key<ItemClassCategory>>,
    pub removed_if_leaves_area: bool,
    pub allocate_to_map_owner: bool,
    pub always_allocate: bool,
    pub can_have_veiled_mods: bool,
    pub always_show: bool,
    pub can_be_corrupted: bool,
    pub can_have_incubators: bool,
    pub can_have_influence: bool,
    pub can_be_double_corrupted: bool,
    pub can_have_aspects: bool,
    pub can_transfer_skin: bool,
    pub item_stance: Option<u32>,
    pub can_scourge: bool,
    pub can_upgrade_rarity: bool,
    pub max_inventory_dimensions: Vec<u32>,
    pub unmodifiable: bool,
    pub can_be_fractured: bool,
    pub used_in_map_device: bool,
}

impl_key!(ItemClass);
impl_id!(ItemClass);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tag {
    #[serde(rename = "_index")]
    pub key: Key<Tag>,
    pub id: Id,
    pub display_string: String,
    pub name: String,
}

impl_key!(Tag);
impl_id!(Tag);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Stat {
    #[serde(rename = "_index")]
    pub key: Key<Stat>,
    pub id: Id,
    pub is_local: bool,
    pub is_weapon_local: bool,
    pub semantics: u32,
    pub is_virtual: bool,
    #[serde(rename = "MainHandAlias_StatsKey")]
    pub main_hand_alias_stats_key: Option<Key<Stat>>,
    #[serde(rename = "OffHandAlias_StatsKey")]
    pub off_hand_alias_stats_key: Option<Key<Stat>>,
    pub belongs_active_skills_key: Vec<String>,
    pub category: Option<u32>,
    pub is_scalable: bool,
    pub context_flags: Vec<u32>,
}

impl_key!(Stat);
impl_id!(Stat);
