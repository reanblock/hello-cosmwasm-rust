use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use error::ContractError;

pub mod contract;
pub mod error;
pub mod msg;
pub mod state;

#[entry_point]
pub fn instantiate(deps: DepsMut, env: Env, info: MessageInfo, msg: InstantiateMsg)
  -> StdResult<Response>
{
    contract::instantiate(deps, env, info, msg)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg)
  -> StdResult<Binary>
{
    contract::query(deps, env, msg)
}

#[entry_point]
pub fn execute(deps: DepsMut, 
  env: Env, 
  info: MessageInfo,
  msg: ExecuteMsg) -> Result<Response, ContractError> {
    contract::execute(deps, env, info, msg)
}