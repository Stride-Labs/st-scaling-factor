# stToken Scaling Factor Contract
## Context and Purpose
Osmosis stableswap pools have a "scaling factor" which indicates the expected ratio of the two assets in a pool. The scaling factor of stXXX/XXX pools (i.e. stOSMO/OSMO) is derived from Stride's internal stToken exchange rate (i.e redemption rate). This redemption rate updates every 6 hours on Stride, and, as it stands today, the pool's scaling factor is manually adjusted every other day via a multisig.

With the new `icaoracle` module on Stride, each time the redemption rate updates, the new value is submitted to an oracle contract on Osmosis via an ICA. The contract in the repo represents the last piece of the puzzle to help fully automate the scaling factor changes, by querying the redemption rate value from the oracle contract and submitting the corresponding Osmosis transaction to update the scaling factor.

## Mainnet Deployments
| Chain     | Code ID | Contract Address                                                   |
|-----------|---------|--------------------------------------------------------------------|
| Osmosis   | 152     | osmo12yvjuy69ynnts95ensss4q6480wkvkpnq2z2ntxmfa2qp860xsmq9mzlpn    |

## Overview
The contract consists of admin-gated transactions to register a pool and provide the relevant configuration, as well as a permissionless transaction to refresh the scaling factor of a configured pool based on the redemption rate value in the oracle. 

There is also an admin transaction to bypass the oracle query completely and update the scaling factor directly. However, this is only intended as a temporary safety feature to help phase out control of the scaling factor from a multisig to the contract. Once the contract is working as expected, this transaction can be removed. 

## Redemption Rate to Scaling Factor Conversion
The redemption rate on Stride is a decimal (e.g. `1.2`); however, the scaling factor is represented as an array of two integers that define the ratio (e.g. `[100000, 120000]`). The ordering of the values in the array must align with the ordering of the two assets in the pool definition. For instance, in the [stOSMO/OSMO pool](https://osmosis-api.polkachu.com/osmosis/gamm/v1beta1/pools/833), `ibc/stuosmo` is defined as the first asset, and `uosmo` is defined as the second asset. Consequently, the redemption rate value is reflected in the second value in the scaling factors array. To support both stXXX/XXX and XXX/stXXX pools, the relative ordering of the assets is defined in the pool configuration (see `AssetOrdering`). 

## Transactions
* **AddPool** [admin]: Registers a pool so that it's scaling factor can be updated
* **RemovePool** [admin]: Removes a pool so that the contract will no longer adjust the scaling factor
* **UpdateScalingFactor** [permissionless]: Refreshes the scaling factor for a given pool based on the value in the oracle
* **SudoAdjustScalingFactors**[admin]: Bypasses the oracle and updates the scaling factor directly

## Scheduling
The `UpdateScalingFactor` should be triggered every 6 hours after the redemption rate updates. This execution was originally planned to run through croncat, which is a decentralized CW scheduling solution. However, croncat does not appear to be mature enough on Osmosis yet, so in the interim, the contract will be triggered off-chain. However, we'll continue to explore scheduling solutions in the coming days.

## Local Testing
### Setup Dependencies (Dockernet and Oracle Contract)
* Clone this repo so that it sits at the same level as the `stride` and `ica-oracle` repos
* Navigate to the stride repo
```bash
cd ../stride
```
* Update the `HOST_CHAINS` variable in [config.sh](https://github.com/Stride-Labs/stride/blob/4b1c63332452b2772dc1b26b47547975b8cbd8e0/dockernet/config.sh#L19) to run only osmosis
```bash
HOST_CHAINS=(OSMO)
```
* Start dockernet from Stride repo home directory
```bash
git checkout ica-oracle
git submodule update --init --recursive
make start-docker build=sor

# Each ensuing run, you can just run `make start-docker` which will only rebuild the Stride binary
```
* Navigate to the oracle repo and build the oracle contract
```bash
cd ../ica-oracle
make build-optimized
```
* Upload the oracle contract, register the oracle, and add a metric (all in one command)
```bash
make setup-dockernet-manual
```

### Execute st-scaling-factor contract
* Navigate back to this repo
```bash
cd ../st-scaling-factor
```
* Build the st-scaling-factor contract
```bash
make build-optimized
```
* Upload this contract and create an stToken stableswap pool
```bash
make setup-dockernet
```
* Register the pool with the contract
```bash
make register-pool
```
* Finally, update the scaling factor
```bash
make update-scaling-factor
```