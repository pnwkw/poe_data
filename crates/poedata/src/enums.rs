use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::Debug;
use std::hash::Hash;
use strum::VariantArray;

#[derive(Clone, Copy, Debug, Deserialize_repr, Serialize_repr, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Domain {
    Item = 1,
    Flask = 2,
    Monster = 3,
    Strongbox = 4,
    Area = 5,
    // Unused6 = 6,
    Relic = 7,
    // Unused8 = 8,
    Crafted = 9,
    Jewel = 10,
    Atlas = 11,
    Leaguestone = 12,
    AbyssJewel = 13,
    MapDevice = 14,
    Unknown15 = 15,
    Delve = 16,
    DelveArea = 17,
    SynthesisArea = 18,
    SynthesisAreaGlobal = 19,
    SynthesisImplicit = 20,
    ClusterJewel = 21,
    HeistArea = 22,
    HeistItem = 23,
    HeistTrinket = 24,
    Watchstone = 25,
    Veiled = 26,
    ExpeditionRemnant = 27,
    Unveiled = 28,
    EldritchAltar = 29,
    Sentinel = 30,
    Memory = 31,
    SanctifiedRelic = 32,
    CrucibleArea = 33,
    Tincture = 34,
    Charm = 35,
    NecropolisMonster = 36,
    UberMapArea = 37,
    Quest = 38,
    Graft = 39,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize_repr,
    Serialize_repr,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    VariantArray,
)]
#[repr(u8)]
pub enum EssenceType {
    Hatred = 0,
    Woe = 1,
    Greed = 2,
    Contempt = 3,
    Sorrow = 4,
    Anger = 5,
    Torment = 6,
    Fear = 7,
    Suffering = 8,
    Rage = 9,
    Wrath = 10,
    Doubt = 11,
    Anguish = 12,
    Loathing = 13,
    Spite = 14,
    Zeal = 15,
    Misery = 16,
    Dread = 17,
    Scorn = 18,
    Envy = 19,
    Hysteria = 20,
    Insanity = 21,
    Horror = 22,
    Delirium = 23,
    #[strum(disabled)]
    RemnantOfCorruption = 24,
    Desolation = 25,
}

#[derive(Clone, Copy, Debug, Deserialize_repr, Serialize_repr, PartialEq)]
#[repr(u8)]
pub enum GenerationType {
    Prefix = 1,
    Suffix = 2,
    Intrinsic = 3,
    Nemesis = 4,
    Corrupted = 5,
    Bloodlines = 6,
    Torment = 7,
    Tempest = 8,
    Talisman = 9,
    Enchantment = 10,
    Essence = 11,
    // Unused12 = 12,
    Bestiary = 13,
    Delve = 14,
    SynthesisUnknown = 15,
    SynthesisGlobals = 16,
    SynthesisBonus = 17,
    Blight = 18,
    BlightTower = 19,
    MonsterAffliciton = 20,
    EnkindlingOrb = 21,
    InstillingOrb = 22,
    ExpeditionLogbook = 23,
    ScourgeBenefit = 24,
    ScourgeDetriment = 25,
    ScourgeGimmick = 26,
    // Unused27 = 27,
    SearingExarch = 28,
    EaterOfWorlds = 29,
    Archnemesis = 30,
    CruciblePassiveTree = 31,
    CruciblePassiveTreeMutation = 32,
    AfflicitonWisps = 33,
    NecropolisDownside = 34,
    NecropolisUpside = 35,
    MemoryAltar = 36,
}

#[derive(Clone, Copy, Debug, Deserialize_repr, Serialize_repr, PartialEq)]
#[repr(u8)]
pub enum InfluenceType {
    Shaper = 0,
    Elder = 1,
    Crusader = 2,
    Redeemer = 3,
    Hunter = 4,
    Warlord = 5,
    Normal = 6,
}

#[derive(Deserialize_repr, Serialize_repr, PartialEq, Eq, Hash, Clone, Copy, Debug)]
#[repr(u8)]
pub enum CraftingBenchCustomAction {
    RemoveCraftedMods = 0,
    RemoveEnchantMods = 1,
    // Unknown2 = 2,
    // Unknown3 = 3,
    // Unknown4 = 4,
    // Unknown5 = 5,
    // Unknown6 = 6,
    // Unknown7 = 7,
    // Unknown8 = 8,
}
