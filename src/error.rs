use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Unable to query price of {token} from oracle, {error}")]
    UnableToQueryPrice { token: String, error: String },

    #[error("Pool {pool_id} is not configured in the contract")]
    PoolNotFound { pool_id: u64 },

    #[error("Pool {pool_id} is already configured in the contract")]
    PoolAlreadyExists { pool_id: u64 },

    #[error("Pool {pool_id} not found on Osmosis")]
    PoolNotFoundOsmosis { pool_id: u64 },

    #[error("The specified asset ordering does not match the underlying pool")]
    InvalidPoolAssetOrdering {},

    #[error("The underlying pool has {number} of assets, only 2 is allowed")]
    InvalidNumberOfPoolAssets { number: u64 },
}
