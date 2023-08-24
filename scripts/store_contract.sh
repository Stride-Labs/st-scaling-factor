set -eu
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
source ${SCRIPT_DIR}/vars.sh

CONTRACT=./artifacts/st_scaling_factor.wasm

echo "Storing contract..."

echo ">>> osmosisd tx wasm store $CONTRACT"
tx_hash=$($OSMOSISD tx wasm store $CONTRACT $GAS --from oval1 -y | grep -E "txhash:" | awk '{print $2}') 

echo "Tx Hash: $tx_hash"
echo $tx_hash > $METADATA/store_tx_hash.txt

sleep 3

code_id=$($OSMOSISD q tx $tx_hash | grep code_id -m 1 -A 1 | tail -1 | awk '{print $2}' | tr -d '"')
echo "Code ID: $code_id"
echo $code_id > $METADATA/code_id.txt