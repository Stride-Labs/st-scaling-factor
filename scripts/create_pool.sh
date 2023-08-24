set -eu
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
source ${SCRIPT_DIR}/vars.sh

echo "Creating stableswap pool..."

contract_address=$(cat ${SCRIPT_DIR}/metadata/contract_address.txt)
cat << EOF > ${SCRIPT_DIR}/metadata/pool.json
{
    "initial-deposit": "100000000${STOSMO_IBC_DENOM},100000000uosmo",
    "swap-fee": "0.01",
    "exit-fee": "0.0",
    "future-governor": "",
    "scaling-factors": "100000,100000",
    "scaling-factor-controller": "$contract_address"
}
EOF

echo ">>> osmosisd tx gamm create-pool --pool-type stableswap --pool-file pool.json --from oval1"
$OSMOSISD tx gamm create-pool --pool-type stableswap --pool-file ${SCRIPT_DIR}/metadata/pool.json --from oval1 -y $GAS | TRIM_TX
sleep 5

echo ">>> osmosisd q gamm pools"
$OSMOSISD q gamm pools
