BUILDDIR ?= $(CURDIR)/build
STRIDE_HOME ?= $(CURDIR)/../stride

format: 
	cargo fmt

lint:
	cargo clippy

.PHONY: build
build:
	cargo wasm

build-debug:
	cargo wasm-debug

build-optimized:
	docker run --rm -v "$(CURDIR)":/code \
		--mount type=volume,source="$(notdir $(CURDIR))_cache",target=/code/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		cosmwasm/rust-optimizer:0.12.12

validate:
	cosmwasm-check ./artifacts/st_scaling_factor.wasm

# Uploads the contract to osmosis
store-contract: 
	@STRIDE_HOME=$(STRIDE_HOME) bash scripts/store_contract.sh

# Instantiates the contract directly with the osmosis dockernet validator as the admin
instantiate-contract: 
	@STRIDE_HOME=$(STRIDE_HOME) bash scripts/instantiate_contract.sh

# Mint stuosmo from Stride
mint-sttoken:
	@STRIDE_HOME=$(STRIDE_HOME) bash scripts/mint_sttoken.sh

# Create stuosmo/uosmo pool on Osmosis
create-pool:
	@STRIDE_HOME=$(STRIDE_HOME) bash scripts/create_pool.sh

# Registers the stableswap pool in the oracle
register-pool:
	@STRIDE_HOME=$(STRIDE_HOME) bash scripts/register_pool.sh

# Registers the stableswap pool in the oracle
query-pools:
	@STRIDE_HOME=$(STRIDE_HOME) bash scripts/query_pools.sh

# Executes the update-scaling-factor tx in the contract 
update-scaling-factor:
	@STRIDE_HOME=$(STRIDE_HOME) bash scripts/update_scaling_factor.sh

# Adjust's the scaling factor directly by bypassing the query
sudo-adjust-scaling-factors:
	@STRIDE_HOME=$(STRIDE_HOME) bash scripts/sudo_adjust_scaling_factors.sh

# Initializes the contract and creates the stToken pool
setup-dockernet: 
	@$(MAKE) store-contract && sleep 5
	@$(MAKE) instantiate-contract && sleep 5
	@$(MAKE) mint-sttoken && sleep 5 
	@$(MAKE) create-pool
