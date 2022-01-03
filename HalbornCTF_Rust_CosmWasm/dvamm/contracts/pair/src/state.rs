use dvamm::asset::PairInfo;
use cosmwasm_std::{Addr};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub pair_info: PairInfo,
    pub factory_addr: Addr,
    pub block_time_last: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");
