use cosmwasm_std::{Deps, StdResult, Uint64};
use crate::state::{CONFIG, DEEPLINKS};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

pub fn query_id(deps: Deps, id: Uint64) -> StdResult<IdResponse> {
    let deeplink = DEEPLINKS.load(deps.storage, id.u64())?;
    Ok(IdResponse {
        id,
        type_: deeplink.type_,
        from: deeplink.from,
        to: deeplink.to,
    })
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct IdResponse {
    pub id: Uint64,
    #[serde(rename = "type")]
    pub type_: String,
    pub from: String,
    pub to: String,
}

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let cfg = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        admins: cfg.admins.into_iter().map(|a| a.into()).collect(),
        executors: cfg.executors.into_iter().map(|a| a.into()).collect()
    })
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ConfigResponse {
    pub admins: Vec<String>,
    pub executors: Vec<String>,
}