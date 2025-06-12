# dfx canister create ic_siwe_provider

# dfx build ic_siwe_provider 

# dfx deps pull

# dfx deps init evm_rpc --argument '(record {})' 

# dfx deps deploy

# dfx generate ic_siwe_provider 

# dfx generate backend 

# dfx generate frontend 

dfx canister uninstall-code 7cdn3-haaaa-aaaad-aaoja-cai --ic

dfx canister uninstall-code 7fclp-kyaaa-aaaad-aaojq-cai --ic

dfx canister uninstall-code 7qf2c-lqaaa-aaaad-aaoka-cai --ic

cargo build --release --target wasm32-unknown-unknown --package backend

candid-extractor target/wasm32-unknown-unknown/release/backend.wasm >src/backend/backend.did

dfx generate backend  

dfx deploy ic_siwe_provider --argument='(
  record {
    uri = "https://example.com";
    runtime_features = null;
    domain = "www.example.com";
    statement = null;
    scheme = null;
    salt = "nnFSskjBKhoh097slkfn";
    session_expires_in = null;
    targets = null;
    chain_id = null;
    sign_in_expires_in = null;
  }
)' --ic

dfx deploy backend --ic

dfx deploy frontend --ic