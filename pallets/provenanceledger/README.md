# Provenance Ledger Pallet

This crate is a FRAME pallet, not an ink! smart contract.

Because your network is already running, this pallet must be installed through a runtime upgrade.
You cannot deploy this crate as a contract.

## What Works With Your Current 2-Node Network

- Dynamic install path: `sudo(system.setCode(...))` runtime upgrade.
- Signer/admin: validator1 controller account (your sudo account).
- RPC endpoint: `ws://127.0.0.1:9944` (or your external endpoint once exposed).

## 1. Integrate Pallet Into Runtime Source

Copy this crate into your chain workspace (on the machine where you build the runtime):

```sh
mkdir -p /path/to/qanetwork-chain/pallets
cp -R /Users/starlord/Documents/aqilliz/qanetwork/provenanceledger /path/to/qanetwork-chain/pallets/provenanceledger
```

Then wire it into runtime:

1. Add dependency in runtime Cargo.toml:

```toml
pallet-provenanceledger = { path = "../pallets/provenanceledger", default-features = false }
```

2. Enable std feature in runtime Cargo.toml std list:

```toml
"pallet-provenanceledger/std",
```

3. Add config in `runtime/src/configs/mod.rs`:

```rust
impl pallet_provenanceledger::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}
```

4. Add to `construct_runtime!` in `runtime/src/lib.rs`:

```rust
pub type ProvenanceLedger = pallet_provenanceledger;
```

Use the next available pallet index in your runtime.

## 2. Build Runtime Wasm

From `qanetwork-chain`:

```sh
cargo build --release -p solochain-template-runtime
```

Expected wasm output (adjust if your runtime crate name differs):

```text
target/release/wbuild/solochain-template-runtime/solochain_template_runtime.compact.compressed.wasm
```

## 3. Runtime Upgrade On Running Network

Use script `scripts/runtime-upgrade.js` from this folder.

Install JS dependencies once:

```sh
npm init -y
npm install @polkadot/api @polkadot/keyring @polkadot/util-crypto
```

Run upgrade:

```sh
WS_URL=ws://127.0.0.1:9944 \
SUDO_SEED_FILE=/home/ec2-user/keys/validator1-controller.txt \
RUNTIME_WASM=/absolute/path/to/solochain_template_runtime.compact.compressed.wasm \
node scripts/runtime-upgrade.js
```

## 4. Verify Pallet Exists In Metadata

```sh
WS_URL=ws://127.0.0.1:9944 node scripts/check-pallet.js
```

Expected output contains `pallet_provenanceledger`.

## 5. Submit Test Call To New Pallet

```sh
WS_URL=ws://127.0.0.1:9944 \
CALLER_SEED_FILE=/home/ec2-user/keys/validator2-controller.txt \
node scripts/test-add-activity.js
```

This sends one `add_activity` transaction.

## Notes

- If validator1 is down, upgrades will fail because sudo signer is unavailable.
- Keep both validators up for stable finality.
- Public RPC exposure should be restricted by security group source IP.
