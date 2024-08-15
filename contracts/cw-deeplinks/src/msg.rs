use cosmwasm_std::Uint64;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cyber_std::Link;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
    pub executers: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NamedDeeplink {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub from: Option<String>,
    pub to: Option<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct  Deeplink {
    #[serde(rename = "type")]
    pub type_: String,
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreatedNamedDeeplink {
        name: String,
        deeplink: Deeplink,
    },
    CreateDeeplink {
        deeplink: Deeplink,
    },
    CreateDeeplinks {
        deeplinks: Vec<Deeplink>,
    },
    // TODO update UpdateDeeplink
    UpdateDeeplink {
        #[serde(rename = "type")]
        type_: String,
        from: Option<String>,
        to: Option<String>,
    },
    DeleteDeeplink {
        id: Uint64,
    },
    Cyberlink {
        links: Vec<Link>
    },
    UpdateAdmins {
        new_admins: Vec<String>
    },
    UpdateExecutors {
        new_executors: Vec<String>
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    LastId {},
    DebugState {},
    Deeplink {
        id: Uint64,
    },
    Config {},
}
