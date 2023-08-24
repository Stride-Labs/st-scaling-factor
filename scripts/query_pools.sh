set -eu
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
source ${SCRIPT_DIR}/vars.sh

contract_address=$(cat $METADATA/contract_address.txt)

echo "Querying registered pools..."

msg='{ "all_pools" : { } }'
echo ">>> osmosisd q wasm contract-state smart $contract_address $msg"
$OSMOSISD q wasm contract-state smart $contract_address "$msg"
sleep 1
