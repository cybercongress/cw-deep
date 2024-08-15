use cosmwasm_std::{Deps, StdError, StdResult, Uint64};
use crate::state::{CONFIG, DEEPLINKS, DeeplinkState, DELETED_IDS, ID, NAMED_DEEPLINKS};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use crate::ContractError;
use crate::msg::Deeplink;

pub fn query_last_id(deps: Deps) -> StdResult<Uint64> {
    let last_id = ID.load(deps.storage)?;
    Ok(Uint64::new(last_id))
}

pub fn query_id(deps: Deps, id: Uint64) -> StdResult<DeeplinkState> {
    // Check if the deeplink is deleted
    if DELETED_IDS.may_load(deps.storage, id.u64())?.unwrap_or(false) {
        return Err(StdError::not_found("deleted deeplink"));
    }

    // Load the deeplink state
    let deeplink = DEEPLINKS.load(deps.storage, id.u64())?;
    Ok(deeplink)
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

pub fn query_state(deps: Deps) -> StdResult<StateResponse> {
    let deeplinks = DEEPLINKS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|i| i.unwrap())
        .collect::<Vec<(u64, DeeplinkState)>>();
    let named_deeplinks = NAMED_DEEPLINKS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|i| i.unwrap())
        .collect::<Vec<(String, DeeplinkState)>>();
    let deleted_deeplinks = DELETED_IDS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|i| i.unwrap().0)
        .collect::<Vec<u64>>();

    Ok(StateResponse {
        deeplinks,
        named_deeplinks,
        deleted_deeplinks,
    })
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct StateResponse {
    pub deeplinks: Vec<(u64, DeeplinkState)>,
    pub named_deeplinks: Vec<(String, DeeplinkState)>,
    pub deleted_deeplinks: Vec<u64>,
}