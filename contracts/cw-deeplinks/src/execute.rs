use cosmwasm_std::{attr, Deps, DepsMut, Env, MessageInfo, StdResult, SubMsg, Uint64};

use crate::error::ContractError;
use crate::state::{CONFIG, DeeplinkState, DEEPLINKS, FROM_MAP, ID, TO_MAP, TYPE_MAP};
use cyber_std::{create_cyberlink_msg, Link, CyberMsgWrapper};
use crate::contract::map_validate;
use crate::msg::Deeplink;

type Response = cosmwasm_std::Response<CyberMsgWrapper>;
pub const CYBERLINK_ID_MSG: u64 = 42;

fn validate_deeplink(
    deps: Deps,
    type_: &String,
    from: &Option<String>,
    to: &Option<String>,
) -> Result<(), ContractError> {

    // Validation
    if let (Some(ref from), Some(ref to)) = (&from, &to) {
        if from != to && (from == "Null" || to == "Null") {
            return Err(ContractError::InvalidDeeplink {
                id: Uint64::zero(),
                from: from.clone(),
                to: to.clone(),
                type_: type_.clone(),
            });
        }
    }

    // Check if type exists
    if !TYPE_MAP.has(deps.storage, type_.as_str()) {
        return Err(ContractError::TypeNotExists { type_: type_.clone() });
    }

    // Check if from exists
    if let Some(ref from) = from {
        if !FROM_MAP.has(deps.storage, from.as_str()) {
            return Err(ContractError::FromNotExists { from: from.clone() });
        }
    }

    // Check if to exists
    if let Some(ref to) = to {
        if !TO_MAP.has(deps.storage, to.as_str()) {
            return Err(ContractError::ToNotExists { to: to.clone() });
        }
    }

    // Additional validation for type conflicts
    if let (Some(ref from), Some(ref to)) = (&from, &to) {
        let type_deeplink = DEEPLINKS.load(deps.storage,
        TYPE_MAP.load(deps.storage, type_.as_str())?
        )?;
        let from_deeplink = DEEPLINKS.load(deps.storage,
        FROM_MAP.load(deps.storage, from.as_str())?
        )?;
        let to_deeplink = DEEPLINKS.load(deps.storage,
        TO_MAP.load(deps.storage, to.as_str())?
        )?;

        if type_deeplink.from != "Any" && type_deeplink.from != from_deeplink.type_ {
            return Err(ContractError::TypeConflict {
                id: Uint64::zero(),
                type_: type_.clone(),
                from: from.clone(),
                to: to.clone(),
                expected_from: type_deeplink.from,
                expected_to: type_deeplink.to,
                received_from: from_deeplink.type_,
                received_to: to_deeplink.type_,
            });
        }

        if type_deeplink.to != "Any" && type_deeplink.to != to_deeplink.type_ {
            return Err(ContractError::TypeConflict {
                id: Uint64::zero(),
                type_: type_.clone(),
                from: from.clone(),
                to: to.clone(),
                expected_from: type_deeplink.from,
                expected_to: type_deeplink.to,
                received_from: from_deeplink.type_,
                received_to: to_deeplink.type_,
            });
        }
    }

    Ok(())
}

fn create_deeplink(
    deps: DepsMut,
    deeplink: Deeplink
) -> Result<u64, ContractError> {
    validate_deeplink(deps.as_ref(), &deeplink.type_, &deeplink.from, &deeplink.to)?;

    // Generate new ID
    let id = ID.load(deps.storage)? + 1;
    ID.save(deps.storage, &id)?;

    // Save new Deeplink
    let deeplink_state = DeeplinkState {
        type_: deeplink.type_.clone(),
        from: deeplink.from.unwrap_or_else(|| "Any".to_string()),
        to: deeplink.to.unwrap_or_else(|| "Any".to_string()),
    };
    DEEPLINKS.save(deps.storage, id, &deeplink_state)?;

    // Save to new maps
    // Save to new maps
    TYPE_MAP.save(deps.storage, deeplink_state.type_.as_str(), &id)?;
    FROM_MAP.save(deps.storage, deeplink_state.from.as_str(), &id)?;
    TO_MAP.save(deps.storage, deeplink_state.to.as_str(), &id)?;

    Ok(id)
}
pub fn execute_create_deeplink(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    deeplink: Deeplink
) -> Result<Response, ContractError> {
    create_deeplink(deps, deeplink)?;
    Ok(Response::new().add_attributes(vec![attr("action", "create_deeplink")]))
}

pub fn execute_create_deeplinks(
    mut deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    deeplinks: Vec<Deeplink>
) -> Result<Response, ContractError> {
    for deeplink in deeplinks {
        create_deeplink(deps.branch(), deeplink)?;
    }
    Ok(Response::new().add_attributes(vec![attr("action", "create_deeplinks")]))
}

pub fn execute_update_deeplink(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    type_: String,
    from: Option<String>,
    to: Option<String>,
) -> Result<Response, ContractError> {

    Ok(Response::new().add_attributes(vec![attr("action", "update")]))
}

pub fn execute_delete_deeplink(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    id: Uint64
) -> Result<Response, ContractError> {

    Ok(Response::new().add_attributes(vec![attr("action", "delete")]))
}

pub fn execute_update_admins(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_admins: Vec<String>,
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;
    if !cfg.can_modify(info.sender.as_ref()) {
        return Err(ContractError::Unauthorized {});
    }

    let admins = map_validate(deps.api, &new_admins)?;
    CONFIG.update(deps.storage, |mut cfg| -> StdResult<_> {
        cfg.admins = admins;
        Ok(cfg)
    })?;

    Ok(Response::new().add_attributes(vec![attr("action", "update_admins")]))
}

pub fn execute_update_executors(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_executors: Vec<String>,
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;
    if !cfg.can_modify(info.sender.as_ref()) {
        return Err(ContractError::Unauthorized {});
    }

    let executors = map_validate(deps.api, &new_executors)?;
    CONFIG.update(deps.storage, |mut cfg| -> StdResult<_> {
        cfg.executors = executors;
        Ok(cfg)
    })?;

    Ok(Response::new().add_attributes(vec![attr("action", "update_executors")]))
}

pub fn execute_cyberlink(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cyberlink: Vec<Link>,
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;
    if !cfg.can_execute(info.sender.as_ref()) {
        return Err(ContractError::Unauthorized {});
    }

    let msg = create_cyberlink_msg(env.contract.address.to_string(), cyberlink);
    Ok(Response::new().add_submessage(SubMsg::reply_on_error(msg, CYBERLINK_ID_MSG)))
}

