set -eu
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
source ${SCRIPT_DIR}/vars.sh

echo "Minting stTokens..."

stride_address=$($STRIDED keys show val1 -a)
osmo_address=$($OSMOSISD keys show oval1 -a)

echo ">>> osmosisd tx ibc-transfer transfer transfer channel-0 $stride_address 100000000uosmo --from oval1"
$OSMOSISD tx ibc-transfer transfer transfer channel-0 $stride_address 100000000uosmo --from oval1 -y $GAS | TRIM_TX 
sleep 10

echo ">>> strided tx stakeibc liquid-stake 10000000 uosmo --from oval1"
$STRIDED tx stakeibc liquid-stake 100000000 uosmo --from val1 -y | TRIM_TX 
sleep 5

echo ">>> strided tx ibc-transfer transfer transfer channel-0 $osmo_address 100000000stuosmo --from val1"
$STRIDED tx ibc-transfer transfer transfer channel-0 $osmo_address 100000000stuosmo --from val1 -y | TRIM_TX 
sleep 5

echo ">>> osmosisd q bank balances $osmo_address"
$OSMOSISD q bank balances $osmo_address