use crate::state::Pool;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Decimal};

use crate::state::AssetOrdering;

/// Instantiates the contract with an admin address and oracle contract address
#[cw_serde]
pub struct InstantiateMsg {
    pub admin_address: String,
    pub oracle_contract_address: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Updates the admin or oracle contract address from the config
    UpdateConfig {
        admin_address: String,
        oracle_contract_address: String,
    },
    /// Adds a new stToken stable swap pool
    /// Only the admin can add pool
    AddPool {
        /// Pool ID of the Osmosis pool (e.g. 833)
        pool_id: u64,
        /// The denom of the stToken as it lives on Osmosis (e.g. ibc/{hash(transfer/channel-0/stuosmo)})
        /// This is the same denom that's in the oracle contract, and will line up with the denom in the
        /// Osmosis pool
        sttoken_denom: String,
        /// The ordering of the stToken vs nativeToken assets in the Osmosis pool,
        /// specifically with respect to the scaling factors
        /// This will determine the ordering of elements in the scaling factor array
        ///   e.g. If AssetOrdering::StTokenFirst, that means the assets in the pool are
        ///        ordered as [stToken, nativeToken], and the native token must be scaled up
        ///        So a redemption rate of 1.2 would imply a scaling factors array of [10000, 12000]
        asset_ordering: AssetOrdering,
    },
    /// Removes an stToken stable swap pool, preventing the pool from having it's scaling factors adjusted
    /// Only the admin can remove pools
    RemovePool { pool_id: u64 },
    /// Updates the scaling factors for a pool by querying the redemption rate of the stToken
    /// from the ICA Oracle and submitting an `adjust-scaling-factor` transaction on Osmosis
    /// This message is permissionless
    UpdateScalingFactor { pool_id: u64 },
    /// Allows the admin to bypass the query and adjust the scaling factor directly
    /// This is meant as a safety mechanism after the contract is first deployed and
    /// should eventually be removed
    SudoAdjustScalingFactors {
        pool_id: u64,
        scaling_factors: Vec<u64>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Returns the contract's config
    #[returns(crate::state::Config)]
    Config {},

    /// Returns a the configuration for a specific stToken stableswap pool
    #[returns(crate::state::Pool)]
    Pool { pool_id: u64 },

    /// Returns all pools controlled by the contract
    #[returns(Pools)]
    AllPools {},
}

#[cw_serde]
pub struct Pools {
    pub pools: Vec<Pool>,
}

/// Price query as defined in the ICA Oracle contract
#[cw_serde]
#[derive(QueryResponses)]
pub enum OracleQueryMsg {
    #[returns(PriceResponse)]
    Price {
        denom: String,
        params: Option<Binary>,
    },
}

/// Response from ICA Oracle price query
#[cw_serde]
pub struct PriceResponse {
    pub exchange_rate: Decimal,
    pub update_time: u64,
}
