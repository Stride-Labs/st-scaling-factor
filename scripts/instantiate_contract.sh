set -eu
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
source ${SCRIPT_DIR}/vars.sh

osmo_val=$($OSMOSISD keys show oval1 -a)
code_id=$(cat $METADATA/code_id.txt)

# This assumes the ica-oracle repo is sitting at the same level as this project
oracle_contract_address=$(cat ${SCRIPT_DIR}/../../ica-oracle/scripts/metadata/contract_address.txt)

echo "Instantiating contract..."
init_msg="{ \"admin_address\": \"$osmo_val\", \"oracle_contract_address\": \"$oracle_contract_address\" }"

echo ">>> osmosisd tx wasm instantiate $code_id "$init_msg""
tx_hash=$($OSMOSISD tx wasm instantiate $code_id "$init_msg" --from oval1 --label "st-scaling-factor" --no-admin $GAS -y | grep -E "txhash:" | awk '{print $2}') 

echo "Tx Hash: $tx_hash"
echo $tx_hash > $METADATA/instantiate_tx_hash.txt

sleep 3

contract_address=$($OSMOSISD q tx $tx_hash | grep contract_address -m 1 -A 1 | tail -1 | awk '{print $2}')
echo "Contract Address: $contract_address"
echo $contract_address > $METADATA/contract_address.txt