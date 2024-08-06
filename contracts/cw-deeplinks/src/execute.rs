use cosmwasm_std::{attr, Deps, DepsMut, Env, MessageInfo, StdResult, SubMsg, Uint64};

use crate::error::ContractError;
use crate::state::{CONFIG, Deeplink, DEEPLINKS, ID};
use cyber_std::{create_cyberlink_msg, Link, CyberMsgWrapper};
use crate::contract::map_validate;

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
    let type_exists = DEEPLINKS.range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .any(|item| item.map_or(false, |(_, deeplink)| deeplink.type_ == type_.clone()));
    if !type_exists {
        return Err(ContractError::TypeNotExists { type_: type_.clone() });
    }

    // Check if from exists
    if let Some(ref from) = from {
        let from_exists = DEEPLINKS.range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .any(|item| item.map_or(false, |(_, deeplink)| deeplink.type_ == from.clone()));
        if !from_exists {
            return Err(ContractError::FromNotExists { from: from.clone() });
        }
    }

    // Check if to exists
    if let Some(ref to) = to {
        let to_exists = DEEPLINKS.range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .any(|item| item.map_or(false, |(_, deeplink)| deeplink.type_ == to.clone()));
        if !to_exists {
            return Err(ContractError::ToNotExists { to: to.clone() });
        }
    }

    // Additional validation for type conflicts
    if let (Some(ref from), Some(ref to)) = (&from, &to) {
        let type_deeplink = DEEPLINKS.range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .find(|item| item.as_ref().map_or(false, |(_, deeplink)| deeplink.type_ == type_.clone()))
            .map(|item| item.unwrap().1)
            .unwrap();

        let from_deeplink = DEEPLINKS.range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .find(|item| item.as_ref().map_or(false, |(_, deeplink)| deeplink.type_ == from.clone()))
            .map(|item| item.unwrap().1)
            .unwrap();

        let to_deeplink = DEEPLINKS.range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .find(|item| item.as_ref().map_or(false, |(_, deeplink)| deeplink.type_ == to.clone()))
            .map(|item| item.unwrap().1)
            .unwrap();

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
pub fn execute_create_deeplink(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    type_: String,
    from: Option<String>,
    to: Option<String>,
) -> Result<Response, ContractError> {

    validate_deeplink(deps.as_ref(), &type_, &from, &to)?;

    // Generate new ID
    let id = ID.load(deps.storage)? + 1;
    ID.save(deps.storage, &id)?;

    // Save new Deeplink
    let deeplink = Deeplink {
        type_: type_.clone(),
        from: from.unwrap_or_else(|| "Any".to_string()),
        to: to.unwrap_or_else(|| "Any".to_string()),
    };
    DEEPLINKS.save(deps.storage, id, &deeplink)?;

    Ok(Response::new().add_attributes(vec![attr("action", "create")]))
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

