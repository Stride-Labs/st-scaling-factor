use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use std::fmt;

// The config defines the admin and oracle contract addresses
#[cw_serde]
pub struct Config {
    /// The admin address is able to add and remove pools
    pub admin_address: Addr,
    /// The oracle contract address represents the address of the ICA Oracle contract
    /// that contains the stToken redemption rates
    pub oracle_contract_address: Addr,
}

/// Pool represents a stableswap pool that should have it's scaling factors adjusted
#[cw_serde]
pub struct Pool {
    /// Pool ID of the Osmosis pool (e.g. 833)
    pub pool_id: u64,
    /// The denom of the stToken as it lives on Osmosis (e.g. ibc/{hash(transfer/channel-0/stuosmo)})
    /// This is the same denom that's in the oracle contract, and will line up with the denom in the
    /// Osmosis pool
    pub sttoken_denom: String,
    /// The ordering of the stToken vs nativeToken assets in the Osmosis pool,
    /// specifically with respect to the scaling factors
    /// This will determine the ordering of elements in the scaling factor array
    ///   e.g. If AssetOrdering::StTokenFirst, that means the assets in the pool are
    ///        ordered as [stToken, nativeToken], and the native token must be scaled up
    ///        So a redemption rate of 1.2 would imply a scaling factors array of [10000, 12000]
    pub asset_ordering: AssetOrdering,
    /// The last time (in unix timestamp) that the scaling factors were updated
    pub last_updated: u64,
}

/// Defines the ordering of the two assets (stToken and native token) in a stable swap pool
/// The scaling factors are an array where the index of each factor maps back to the two assets
/// Redemption rate changes should modify the scaling factor that's tied to the native token
/// meaning if the ordering is NativeTokenFirst, then the scaling factor array is [RedemptionRate, 1]
/// Whereas, if the ordering is StTokenFirst, then the scaling factor array is [1, RedemptionRate]
#[cw_serde]
pub enum AssetOrdering {
    /// NativeTokenFirst means the native token is the first asset in the pool, meaning
    /// the redemption rate change should be reflected in the first asset
    NativeTokenFirst,
    /// StTokenFirst means the stToken is the first asset in the pool, and thus, the
    /// native token is the second asset in the pool. This means the redemption rate
    /// change should be reflected in the *second* asset
    StTokenFirst,
}

impl fmt::Display for AssetOrdering {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AssetOrdering::NativeTokenFirst => write!(f, "native_token_first"),
            AssetOrdering::StTokenFirst => write!(f, "st_token_first"),
        }
    }
}

/// The CONFIG store stores contract configuration
pub const CONFIG: Item<Config> = Item::new("config");

/// The POOLS store stores each Osmosis stableswap pool, key'd by the pool ID
pub const POOLS: Map<u64, Pool> = Map::new("pools");
