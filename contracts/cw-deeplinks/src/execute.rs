use cosmwasm_std::{attr, Deps, DepsMut, Env, MessageInfo, StdResult, SubMsg, Uint64};
use cosmwasm_std::Order::Ascending;
use crate::error::ContractError;
use crate::state::{CONFIG, DeeplinkState, DEEPLINKS, ID, DELETED_IDS, NAMED_DEEPLINKS};
use cyber_std::{create_cyberlink_msg, Link, CyberMsgWrapper};
use crate::contract::map_validate;
use crate::msg::Deeplink;

type Response = cosmwasm_std::Response<CyberMsgWrapper>;
pub const CYBERLINK_ID_MSG: u64 = 42;

fn validate_deeplink(
    deps: Deps,
    id: Option<String>,
    deeplink: Deeplink
) -> Result<(), ContractError> {
    // Validation
    if deeplink.from != deeplink.to && (deeplink.from.is_none() || deeplink.to.is_none()) {
        return Err(ContractError::InvalidDeeplink {
            id: Uint64::zero(),
            from: deeplink.from.unwrap_or_else(|| "_".to_string()),
            to: deeplink.to.unwrap_or_else(|| "_".to_string()),
            type_: deeplink.type_.clone(),
        });
    }

    let (mut dtype_, mut dfrom, mut dto): (Option<DeeplinkState>, Option<DeeplinkState>, Option<DeeplinkState>) = (None, None, None);

    dtype_ = NAMED_DEEPLINKS.may_load(deps.storage, deeplink.type_.as_str())?;
    if dtype_.is_none() {
        return Err(ContractError::TypeNotExists { type_: deeplink.type_.clone() });
    }
    if deeplink.from.is_some() {
        dfrom = NAMED_DEEPLINKS.may_load(deps.storage, deeplink.clone().from.unwrap().as_str())?;
        if dfrom.is_none() {
            return Err(ContractError::FromNotExists { from: deeplink.from.unwrap_or_else(|| "_".to_string()) });
        }
    }
    if deeplink.to.is_some() {
        dto = NAMED_DEEPLINKS.may_load(deps.storage, deeplink.clone().to.unwrap().as_str())?;
        if dto.is_none() {
            return Err(ContractError::ToNotExists { to: deeplink.to.unwrap_or_else(|| "_".to_string()) });
        }
    }

    // Additional validation for type conflicts
    if let (Some(ref from), Some(ref to)) = (&deeplink.from, &deeplink.to) {
        if dtype_.clone().unwrap().from.ne(&"Any") && dtype_.clone().unwrap().from.ne(&dfrom.clone().unwrap().type_) {
            return Err(ContractError::TypeConflict {
                id: id.unwrap_or_else(|| "_".to_string()),
                type_: deeplink.clone().type_,
                from: deeplink.clone().from.unwrap_or_else(|| "_".to_string()),
                to: deeplink.clone().to.unwrap_or_else(|| "_".to_string()),
                expected_type: deeplink.clone().type_,
                expected_from: dtype_.clone().unwrap().from,
                expected_to: dtype_.clone().unwrap().to,
                received_type: deeplink.clone().type_,
                received_from: dfrom.clone().unwrap().type_,
                received_to: dto.clone().unwrap().type_,
            });
        }

        if dtype_.clone().unwrap().to.ne(&"Any") && dtype_.clone().unwrap().to.ne(&dto.clone().unwrap().type_) {
            return Err(ContractError::TypeConflict {
                id: id.unwrap_or_else(|| "_".to_string()),
                type_: deeplink.clone().type_,
                from: deeplink.clone().from.unwrap_or_else(|| "_".to_string()),
                to: deeplink.clone().to.unwrap_or_else(|| "_".to_string()),
                expected_type: deeplink.clone().type_,
                expected_from: dtype_.clone().unwrap().from,
                expected_to: dtype_.clone().unwrap().to,
                received_type: deeplink.clone().type_,
                received_from: dfrom.clone().unwrap().type_,
                received_to: dto.clone().unwrap().type_,
            });
        }
    }

    Ok(())
}

fn create_deeplink(
    deps: DepsMut,
    deeplink: Deeplink
) -> Result<u64, ContractError> {
    validate_deeplink(deps.as_ref(), None, deeplink.clone())?;

    // Generate new ID
    let id = ID.load(deps.storage)? + 1;
    ID.save(deps.storage, &id)?;

    // Save new Deeplink
    let type_ = deeplink.clone().type_;
    let deeplink_state = DeeplinkState {
        type_: deeplink.type_.clone(),
        from: deeplink.from.unwrap_or_else(|| "Any".to_string()),
        to: deeplink.to.unwrap_or_else(|| "Any".to_string()),
    };
    DEEPLINKS.save(deps.storage, id, &deeplink_state)?;

    Ok(id)
}

pub fn execute_create_named_deeplink(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    name: String,
    deeplink: Deeplink,
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(deps.storage)?;
    if !cfg.can_modify(info.sender.as_ref()) {
        return Err(ContractError::Unauthorized {});
    }

    validate_deeplink(deps.as_ref(), Some(name.clone()), deeplink.clone())?;

    // Generate new ID
    let id = ID.load(deps.storage)? + 1;
    ID.save(deps.storage, &id)?;

    // Save new Deeplink
    // let type_ = deeplink.clone().type_;
    let deeplink_state = DeeplinkState {
        type_: deeplink.type_.clone(),
        from: deeplink.from.unwrap_or_else(|| "Any".to_string()),
        to: deeplink.to.unwrap_or_else(|| "Any".to_string()),
    };
    DEEPLINKS.save(deps.storage, id, &deeplink_state)?;

    NAMED_DEEPLINKS.save(deps.storage, name.as_str(), &deeplink_state)?;

    Ok(Response::new().add_attributes(vec![attr("action", "create_named_deeplink")]))
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
    let cfg = CONFIG.load(deps.storage)?;
    if !cfg.can_modify(info.sender.as_ref()) {
        return Err(ContractError::Unauthorized {});
    }

    // Mark the deeplink as deleted
    DELETED_IDS.save(deps.storage, id.u64(), &true)?;

    Ok(Response::new()
        .add_attributes(vec![
            attr("action", "delete_deeplink"),
            attr("id", id.to_string())
        ])
    )
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

