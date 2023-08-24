set -eu
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

METADATA=${SCRIPT_DIR}/metadata
mkdir -p $METADATA

OSMOSISD="${STRIDE_HOME}/build/osmosisd --home ${STRIDE_HOME}/dockernet/state/osmo1"
STRIDED="${STRIDE_HOME}/build/strided --home ${STRIDE_HOME}/dockernet/state/stride1"
STOSMO_IBC_DENOM="ibc/4263C1D1EEEA066572F679EF212BDD522ADF0E57C86819AF260C8BC82BD87602"

GAS="--gas-prices 0.1uosmo --gas auto --gas-adjustment 1.3"

TRIM_TX() {
  grep -E "code:|txhash:" | sed 's/^/  /'
  echo ""
}
