use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DeeplinkState {
    #[serde(rename = "type")]
    pub type_: String,
    pub from: String,
    pub to: String,
}

pub const DEEPLINKS_KEY: &str = "deeplinks";
pub const DEEPLINKS: Map<u64, DeeplinkState> = Map::new(DEEPLINKS_KEY);

pub const ID_KEY: &str = "id";
pub const ID: Item<u64> = Item::new("ID_KEY");

pub const TYPE_KEY: &str = "type";
pub const FROM_KEY: &str = "from";
pub const TO_KEY: &str = "to";

pub const TYPE_MAP: Map<&str, u64> = Map::new(TYPE_KEY);
pub const FROM_MAP: Map<&str, u64> = Map::new(FROM_KEY);
pub const TO_MAP: Map<&str, u64> = Map::new(TO_KEY);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admins: Vec<Addr>,
    pub executors : Vec<Addr>
}

impl Config {
    pub fn is_admin(&self, addr: impl AsRef<str>) -> bool {
        let addr = addr.as_ref();
        self.admins.iter().any(|a| a.as_ref() == addr)
    }

    pub fn is_executor(&self, addr: impl AsRef<str>) -> bool {
        let addr = addr.as_ref();
        self.executors.iter().any(|a| a.as_ref() == addr)
    }

    pub fn can_modify(&self, addr: &str) -> bool {
        self.is_admin(addr)
    }

    pub fn can_execute(&self, addr: &str) -> bool {
        self.is_executor(addr)
    }
}

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);


