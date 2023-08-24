use cosmwasm_std::Decimal;

use crate::{state::AssetOrdering, ContractError};
use osmosis_std::types::osmosis::gamm::poolmodels::stableswap::v1beta1::Pool as StableswapPool;

/// Converts an stToken redemption rate (i.e. exchange rate) into a scaling factors array
///
/// stTokens trade above their corresponding native tokens since they have rewards associated with them
/// As a result, the stToken pool composition consists of a larger volume of stTokens and
/// the native tokens should be scaled up accordingly
///
/// The scaling factors array consists of integers that give a ratio of the two assets
/// For instance, a ratio of 1.2 is defined as the array [120000, 100000]
/// The ordering of the elements in the array correspond with the ordering of the two assets in the pool
/// which is configured at the time that the pool is registered
///
/// Ex1: If the redemption rate is 1.2 and the pool has the native asset listed first,
///      the scaling factor should be [120000, 100000]
/// Ex2: If the redemption rate is 1.2345 and the pool has the stToken listed first,
///      the scaling factor should be [100000, 123450]
pub fn convert_redemption_rate_to_scaling_factors(
    redemption_rate: Decimal,
    asset_ordering: AssetOrdering,
) -> Vec<u64> {
    let multiplier_int: u64 = 100_000;
    let multiplier_dec = Decimal::from_ratio(multiplier_int, 1u64);
    let scaling_factor = (redemption_rate * multiplier_dec).to_uint_floor().u128() as u64;

    match asset_ordering {
        AssetOrdering::StTokenFirst => vec![multiplier_int, scaling_factor],
        AssetOrdering::NativeTokenFirst => vec![scaling_factor, multiplier_int],
    }
}

/// Validates the the specified pool configuration matches the actual pool returned from the query
/// (specifically with respect to the ordering of assets)
pub fn validate_pool_configuration(
    stableswap_pool: StableswapPool,
    pool_id: u64,
    sttoken_denom: String,
    asset_ordering: AssetOrdering,
) -> Result<(), ContractError> {
    // Confirm the pool ID matches and there are only two assets in the pool
    if pool_id != stableswap_pool.id {
        return Err(ContractError::PoolNotFoundOsmosis { pool_id });
    }
    if stableswap_pool.pool_liquidity.len() != 2 {
        return Err(ContractError::InvalidNumberOfPoolAssets {
            number: stableswap_pool.pool_liquidity.len() as u64,
        });
    }

    // Confirm the ordering of stToken and native token assets matches
    let expected_sttoken_index: usize = match asset_ordering {
        AssetOrdering::StTokenFirst => 0,
        _ => 1,
    };
    if sttoken_denom != stableswap_pool.pool_liquidity[expected_sttoken_index].denom {
        return Err(ContractError::InvalidPoolAssetOrdering {});
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::vec;

    use cosmwasm_std::Decimal;
    use osmosis_std::types::cosmos::base::v1beta1::Coin;
    use osmosis_std::types::osmosis::gamm::poolmodels::stableswap::v1beta1::Pool as StableswapPool;

    use crate::{
        helpers::convert_redemption_rate_to_scaling_factors, state::AssetOrdering, ContractError,
    };

    use super::validate_pool_configuration;

    // Helper function to build a stableswap pool from an array of denoms
    // E.g. ["sttoken", "native_token"], builds a pool with liquidity
    //      [Coin{"sttoken", 100000}, Coin{"native_token", 100000}]
    fn get_test_stableswap_pool(pool_id: u64, liquidity_denoms: Vec<&str>) -> StableswapPool {
        let pool_liquidity = liquidity_denoms
            .into_iter()
            .map(|denom| Coin {
                denom: denom.to_string(),
                amount: "100000".to_string(),
            })
            .collect();

        StableswapPool {
            id: pool_id,
            pool_liquidity,
            ..Default::default()
        }
    }

    #[test]
    fn test_convert_to_scaling_factor_integer() {
        let redemption_rate = Decimal::from_str("1.0").unwrap();
        let asset_ordering = AssetOrdering::StTokenFirst;
        assert_eq!(
            convert_redemption_rate_to_scaling_factors(redemption_rate, asset_ordering),
            vec![100000, 100000],
        );
    }

    #[test]
    fn test_convert_to_scaling_factor_one_decimal() {
        let redemption_rate = Decimal::from_str("1.2").unwrap();
        let asset_ordering = AssetOrdering::NativeTokenFirst;
        assert_eq!(
            convert_redemption_rate_to_scaling_factors(redemption_rate, asset_ordering),
            vec![120000, 100000],
        );
    }

    #[test]
    fn test_convert_to_scaling_factor_two_decimals() {
        let redemption_rate = Decimal::from_str("1.25").unwrap();
        let asset_ordering = AssetOrdering::StTokenFirst;
        assert_eq!(
            convert_redemption_rate_to_scaling_factors(redemption_rate, asset_ordering),
            vec![100000, 125000],
        );
    }

    #[test]
    fn test_convert_to_scaling_factor_four_decimals() {
        let redemption_rate = Decimal::from_str("1.25236").unwrap();
        let asset_ordering = AssetOrdering::NativeTokenFirst;
        assert_eq!(
            convert_redemption_rate_to_scaling_factors(redemption_rate, asset_ordering),
            vec![125236, 100000],
        );
    }

    #[test]
    fn test_convert_to_scaling_factor_decimal_truncation() {
        let redemption_rate = Decimal::from_str("1.252369923948298234").unwrap();
        let asset_ordering = AssetOrdering::StTokenFirst;
        assert_eq!(
            convert_redemption_rate_to_scaling_factors(redemption_rate, asset_ordering),
            vec![100000, 125236],
        );
    }

    #[test]
    fn test_convert_to_scaling_factor_lt_one() {
        let redemption_rate = Decimal::from_str("0.9837").unwrap();
        let asset_ordering = AssetOrdering::NativeTokenFirst;
        assert_eq!(
            convert_redemption_rate_to_scaling_factors(redemption_rate, asset_ordering),
            vec![98370, 100000],
        );
    }

    #[test]
    fn test_convert_to_scaling_factor_zero() {
        let redemption_rate = Decimal::from_str("0.0").unwrap();
        let asset_ordering = AssetOrdering::StTokenFirst;
        assert_eq!(
            convert_redemption_rate_to_scaling_factors(redemption_rate, asset_ordering),
            vec![100000, 0],
        );
    }

    #[test]
    fn test_validate_pool_configuration_valid_sttoken_first() {
        let pool_id = 2;
        let sttoken_denom = "ibc/sttoken";
        let native_denom = "native";
        let asset_ordering = AssetOrdering::StTokenFirst;

        let actual_pool = get_test_stableswap_pool(pool_id, vec![sttoken_denom, native_denom]);

        assert_eq!(
            validate_pool_configuration(
                actual_pool,
                pool_id,
                sttoken_denom.to_string(),
                asset_ordering
            ),
            Ok(())
        );
    }

    #[test]
    fn test_validate_pool_configuration_valid_native_token_first() {
        let pool_id = 2;
        let sttoken_denom = "ibc/sttoken";
        let native_denom = "native";
        let asset_ordering = AssetOrdering::NativeTokenFirst;

        let actual_pool = get_test_stableswap_pool(pool_id, vec![native_denom, sttoken_denom]);

        assert_eq!(
            validate_pool_configuration(
                actual_pool,
                pool_id,
                sttoken_denom.to_string(),
                asset_ordering
            ),
            Ok(())
        );
    }

    #[test]
    fn test_validate_pool_configuration_mismatch_pool_id() {
        let configured_pool_id = 2;
        let queried_pool_id = 3;
        let sttoken_denom = "ibc/sttoken";
        let native_denom = "native";
        let asset_ordering = AssetOrdering::StTokenFirst;

        let actual_pool =
            get_test_stableswap_pool(queried_pool_id, vec![sttoken_denom, native_denom]);

        assert_eq!(
            validate_pool_configuration(
                actual_pool,
                configured_pool_id,
                sttoken_denom.to_string(),
                asset_ordering
            ),
            Err(ContractError::PoolNotFoundOsmosis {
                pool_id: configured_pool_id
            })
        );
    }

    #[test]
    fn test_validate_pool_configuration_invalid_asset_ordering() {
        let pool_id = 2;
        let sttoken_denom = "ibc/sttoken";
        let native_denom = "native";

        // Actual pool has native first, configured pool specifies stToken first
        let configured_ordering = AssetOrdering::StTokenFirst;
        let actual_pool = get_test_stableswap_pool(pool_id, vec![native_denom, sttoken_denom]);

        assert_eq!(
            validate_pool_configuration(
                actual_pool,
                pool_id,
                sttoken_denom.to_string(),
                configured_ordering
            ),
            Err(ContractError::InvalidPoolAssetOrdering {})
        );

        // Actual pool has stToken first, configured pool specifies native first
        let configured_ordering = AssetOrdering::NativeTokenFirst;
        let actual_pool = get_test_stableswap_pool(pool_id, vec![sttoken_denom, native_denom]);

        assert_eq!(
            validate_pool_configuration(
                actual_pool,
                pool_id,
                sttoken_denom.to_string(),
                configured_ordering
            ),
            Err(ContractError::InvalidPoolAssetOrdering {})
        );
    }
}
