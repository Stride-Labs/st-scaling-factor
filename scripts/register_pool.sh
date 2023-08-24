set -eu
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
source ${SCRIPT_DIR}/vars.sh

echo "Registering pool in contract..."

contract_address=$(cat scripts/metadata/contract_address.txt)
msg=$(cat << EOF
{
    "add_pool": {
        "pool_id": 1,
        "sttoken_denom": "${STOSMO_IBC_DENOM}",
        "asset_ordering": "st_token_first"
    }
}
EOF
)

echo ">>> osmosisd tx wasm execute $contract_address $msg"
tx_hash=$($OSMOSISD tx wasm execute $contract_address "$msg" --from oval1 -y $GAS | grep -E "txhash:" | awk '{print $2}')

echo "Tx Hash: $tx_hash"
echo $tx_hash > $METADATA/store_tx_hash.txt
