set -eu
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
source ${SCRIPT_DIR}/vars.sh

contract_address=$(cat ${SCRIPT_DIR}/metadata/contract_address.txt)

msg='{ "sudo_adjust_scaling_factors": { "pool_id": 1, "scaling_factors": [100000,120000] }}'
echo ">>> osmosisd tx wasm execute $contract_address $msg --from oval1"
$OSMOSISD tx wasm execute $contract_address "$msg" --from oval1 -y $GAS | TRIM_TX
sleep 6

echo ">>> osmosisd q gamm pools"
$OSMOSISD q gamm pools
