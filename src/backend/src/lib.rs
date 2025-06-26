mod declarations;
mod service;
mod user_profile;

use candid::{CandidType, Principal};
use ic_cdk::api::call::call_with_payment128;
use ic_cdk::api::time;
use ic_cdk::{export_candid, update};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use ic_xrc_types::{Asset, AssetClass, GetExchangeRateRequest, GetExchangeRateResult};
use num_traits::cast::ToPrimitive;

use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use user_profile::UserProfile;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static USER_PROFILES: RefCell<StableBTreeMap<String, UserProfile, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
}

use num_bigint::BigUint;
pub type Nat = BigUint;
pub type Nat64 = u64;
pub type Nat32 = u32;
pub type Nat8 = u8;
pub type Nat16 = u16;
pub type Int64 = i64;

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub miner: String,
    pub total_difficulty: Option<Nat>,
    pub receipts_root: String,
    pub state_root: String,
    pub hash: String,
    pub difficulty: Option<Nat>,
    pub size: Nat,
    pub uncles: Vec<String>,
    pub base_fee_per_gas: Option<Nat>,
    pub extra_data: String,
    pub transactions_root: Option<String>,
    pub sha3_uncles: String,
    pub nonce: Nat,
    pub number: Nat,
    pub timestamp: Nat,
    pub transactions: Vec<String>,
    pub gas_limit: Nat,
    pub logs_bloom: String,
    pub parent_hash: String,
    pub gas_used: Nat,
    pub mix_hash: String,
}

#[derive(Serialize, Deserialize)]
pub enum BlockTag {
    Earliest,
    Safe,
    Finalized,
    Latest,
    Number(Nat),
    Pending,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum EthMainnetService {
    Alchemy,
    Ankr,
    BlockPi,
    Cloudflare,
    PublicNode,
    Llama,
}
#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum EthSepoliaService {
    Alchemy,
    Ankr,
    BlockPi,
    PublicNode,
    Sepolia,
}
#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum L2MainnetService {
    Alchemy,
    Ankr,
    BlockPi,
    PublicNode,
    Llama,
}

#[derive(Serialize, Deserialize)]
pub struct FeeHistory {
    pub reward: Vec<Vec<Nat>>,
    pub gas_used_ratio: Vec<f64>,
    pub oldest_block: Nat,
    pub base_fee_per_gas: Vec<Nat>,
}

#[derive(Serialize, Deserialize)]
pub struct FeeHistoryArgs {
    pub block_count: Nat,
    pub newest_block: BlockTag,
    pub reward_percentiles: Option<Vec<Nat8>>,
}

#[derive(Serialize, Deserialize)]
pub struct GetLogsArgs {
    pub from_block: Option<BlockTag>,
    pub to_block: Option<BlockTag>,
    pub addresses: Vec<String>,
    pub topics: Option<Vec<Topic>>,
}

#[derive(Serialize, Deserialize)]
pub struct GetTransactionCountArgs {
    pub address: String,
    pub block: BlockTag,
}

#[derive(Serialize, Deserialize)]
pub struct CallArgs {
    pub transaction: TransactionRequest,
    pub block: Option<BlockTag>,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionRequest {
    #[serde(rename = "type")]
    pub tx_type: Option<String>,
    pub nonce: Option<Nat>,
    pub to: Option<String>,
    pub from: Option<String>,
    pub gas: Option<Nat>,
    pub value: Option<Nat>,
    pub input: Option<String>,
    pub gas_price: Option<Nat>,
    pub max_priority_fee_per_gas: Option<Nat>,
    pub max_fee_per_gas: Option<Nat>,
    pub max_fee_per_blob_gas: Option<Nat>,
    pub access_list: Option<Vec<AccessListEntry>>,
    pub blob_versioned_hashes: Option<Vec<String>>,
    pub blobs: Option<Vec<String>>,
    pub chain_id: Option<Nat>,
}

#[derive(Serialize, Deserialize)]
pub struct AccessListEntry {
    pub address: String,
    pub storage_keys: Vec<String>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum HttpOutcallError {
    IcError {
        code: RejectionCode,
        message: String,
    },
    InvalidHttpJsonRpcResponse {
        status: Nat16,
        body: String,
        parsing_error: Option<String>,
    },
}

#[derive(Serialize, Deserialize)]
pub struct InstallArgs {
    pub demo: Option<bool>,
    pub manage_api_keys: Option<Vec<String>>, // principal as String
    pub log_filter: Option<LogFilter>,
    pub override_provider: Option<OverrideProvider>,
    pub nodes_in_subnet: Option<Nat32>,
}

pub type Regex = String;

#[derive(Serialize, Deserialize)]
pub enum LogFilter {
    ShowAll,
    HideAll,
    ShowPattern(Regex),
    HidePattern(Regex),
}

#[derive(Serialize, Deserialize)]
pub struct RegexSubstitution {
    pub pattern: Regex,
    pub replacement: String,
}

#[derive(Serialize, Deserialize)]
pub struct OverrideProvider {
    pub override_url: Option<RegexSubstitution>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct JsonRpcError {
    pub code: Int64,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct LogEntry {
    pub transaction_hash: Option<String>,
    pub block_number: Option<Nat>,
    pub data: String,
    pub block_hash: Option<String>,
    pub transaction_index: Option<Nat>,
    pub topics: Vec<String>,
    pub address: String,
    pub log_index: Option<Nat>,
    pub removed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Metrics {
    pub requests: Vec<((String, String), u64)>,
    pub responses: Vec<((String, String, String), u64)>,
    pub inconsistent_responses: Vec<((String, String), u64)>,
    pub cycles_charged: Vec<((String, String), Nat)>,
    pub err_http_outcall: Vec<((String, String, RejectionCode), u64)>,
}

#[derive(Serialize, Deserialize)]
pub enum MultiFeeHistoryResult {
    Consistent(FeeHistoryResult),
    Inconsistent(Vec<(RpcService, FeeHistoryResult)>),
}

#[derive(Serialize, Deserialize)]
pub enum MultiGetBlockByNumberResult {
    Consistent(GetBlockByNumberResult),
    Inconsistent(Vec<(RpcService, GetBlockByNumberResult)>),
}

#[derive(Serialize, Deserialize)]
pub enum MultiGetLogsResult {
    Consistent(GetLogsResult),
    Inconsistent(Vec<(RpcService, GetLogsResult)>),
}

#[derive(Serialize, Deserialize)]
pub enum MultiGetTransactionCountResult {
    Consistent(GetTransactionCountResult),
    Inconsistent(Vec<(RpcService, GetTransactionCountResult)>),
}

#[derive(Serialize, Deserialize)]
pub enum MultiGetTransactionReceiptResult {
    Consistent(GetTransactionReceiptResult),
    Inconsistent(Vec<(RpcService, GetTransactionReceiptResult)>),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum MultiSendRawTransactionResult {
    Consistent(SendRawTransactionResult),
    Inconsistent(Vec<(RpcService, SendRawTransactionResult)>),
}

#[derive(Serialize, Deserialize)]
pub enum MultiCallResult {
    Consistent(CallResult),
    Inconsistent(Vec<(RpcService, CallResult)>),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum ProviderError {
    TooFewCycles { expected: String, received: String },
    MissingRequiredProvider,
    ProviderNotFound,
    NoPermission,
    InvalidRpcConfig(String),
}

pub type ProviderId = Nat64;
pub type ChainId = Nat64;

#[derive(Serialize, Deserialize)]
pub struct Provider {
    pub provider_id: ProviderId,
    pub chain_id: ChainId,
    pub access: RpcAccess,
    pub alias: Option<RpcService>,
}

#[derive(Serialize, Deserialize)]
pub enum RpcAccess {
    Authenticated {
        auth: RpcAuth,
        public_url: Option<String>,
    },
    Unauthenticated {
        public_url: String,
    },
}

#[derive(Serialize, Deserialize)]
pub enum RpcAuth {
    BearerToken { url: String },
    UrlParameter { url_pattern: String },
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum RejectionCode {
    NoError,
    CanisterError,
    SysTransient,
    DestinationInvalid,
    Unknown,
    SysFatal,
    CanisterReject,
}

#[derive(Serialize, Deserialize)]
pub enum FeeHistoryResult {
    Ok(FeeHistory),
    Err(RpcError),
}

#[derive(Serialize, Deserialize)]
pub enum GetBlockByNumberResult {
    Ok(Block),
    Err(RpcError),
}

#[derive(Serialize, Deserialize)]
pub enum GetLogsResult {
    Ok(Vec<LogEntry>),
    Err(RpcError),
}

#[derive(Serialize, Deserialize)]
pub enum GetTransactionCountResult {
    Ok(Nat),
    Err(RpcError),
}

#[derive(Serialize, Deserialize)]
pub enum GetTransactionReceiptResult {
    Ok(Option<TransactionReceipt>),
    Err(RpcError),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum SendRawTransactionResult {
    Ok(SendRawTransactionStatus),
    Err(RpcError),
}

#[derive(Serialize, Deserialize)]
pub enum CallResult {
    Ok(String),
    Err(RpcError),
}

#[derive(Serialize, Deserialize)]
pub enum RequestResult {
    Ok(String),
    Err(RpcError),
}

#[derive(Serialize, Deserialize)]
pub enum RequestCostResult {
    Ok(Nat),
    Err(RpcError),
}

#[derive(Serialize, Deserialize)]
pub struct RpcConfig {
    pub response_size_estimate: Option<Nat64>,
    pub response_consensus: Option<ConsensusStrategy>,
}

#[derive(Serialize, Deserialize)]
pub enum ConsensusStrategy {
    Equality,
    Threshold { total: Option<Nat8>, min: Nat8 },
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum ValidationError {
    Custom(String),
    InvalidHex(String),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum RpcError {
    JsonRpcError(JsonRpcError),
    ProviderError(ProviderError),
    ValidationError(ValidationError),
    HttpOutcallError(HttpOutcallError),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct RpcApi {
    pub url: String,
    pub headers: Option<Vec<HttpHeader>>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum RpcService {
    Provider(ProviderId),
    Custom(RpcApi),
    EthSepolia(EthSepoliaService),
    EthMainnet(EthMainnetService),
    ArbitrumOne(L2MainnetService),
    BaseMainnet(L2MainnetService),
    OptimismMainnet(L2MainnetService),
}

#[derive(CandidType, Serialize, Deserialize)]
pub enum RpcServices {
    Custom {
        chainId: ChainId,
        services: Vec<RpcApi>,
    },
    EthSepolia(Option<Vec<EthSepoliaService>>),
    EthMainnet(Option<Vec<EthMainnetService>>),
    ArbitrumOne(Option<Vec<L2MainnetService>>),
    BaseMainnet(Option<Vec<L2MainnetService>>),
    OptimismMainnet(Option<Vec<L2MainnetService>>),
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum SendRawTransactionStatus {
    Ok(Option<String>),
    NonceTooLow,
    NonceTooHigh,
    InsufficientFunds,
}

pub type Topic = Vec<String>;

#[derive(Serialize, Deserialize)]
pub struct TransactionReceipt {
    pub to: Option<String>,
    pub status: Option<Nat>,
    pub transaction_hash: String,
    pub block_number: Nat,
    pub from: String,
    pub logs: Vec<LogEntry>,
    pub block_hash: String,
    #[serde(rename = "type")]
    pub tx_type: String,
    pub transaction_index: Nat,
    pub effective_gas_price: Nat,
    pub logs_bloom: String,
    pub contract_address: Option<String>,
    pub gas_used: Nat,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub enum Error {
    EmptyAsset,
    InvalidAssetLength,
    ExchangeRateError(String),
    CallFailed(String),
}

pub const NANOS_PER_SECOND: u128 = 1_000_000_000u128;
pub const XRC_CYCLES_FEE: u128 = 1_000_000_000u128;
pub const SCALING_FACTOR: u128 = 100_000_000;
pub const MAX_ASSET_LENGTH: usize = 7;

pub fn validate_asset_name(asset_name: &str) -> Result<(), Error> {
    if asset_name.trim().is_empty() {
        return Err(Error::EmptyAsset);
    }
    if asset_name.len() > MAX_ASSET_LENGTH {
        return Err(Error::InvalidAssetLength);
    }

    Ok(())
}

pub fn current_timestamp() -> u64 {
    time() / NANOS_PER_SECOND as u64
}

fn normalize_symbol(symbol: &str) -> String {
    match symbol {
        "ckBTC" => "btc".to_string(),
        "ckETH" => "eth".to_string(),
        "ckUSDC" => "usdc".to_string(),
        "ckUSDT" => "usdt".to_string(),
        _ => symbol.to_string(),
    }
}


#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub enum AssetData {
  CustomPriceFeed{ decimals: u64, rate: u64, timestamp: u64, symbol: String },
  CustomNumber{ id: String, decimals: u64, value: u64 },
  DefaultPriceFeed{ decimals: u64, rate: u64, timestamp: u64, symbol: String },
  CustomString{ id: String, value: String },
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub struct AssetDataResult {
  pub signature: Option<String>,
  pub data: AssetData,
}

#[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
pub enum GetAssetDataWithProofResponse { Ok(AssetDataResult), Err(String) }



#[update]
pub async fn send_raw_transaction(raw_signed_transaction_hex: String) -> Result<String, String> {
    let rpc_services: RpcServices = RpcServices::Custom {
        chainId: ChainId::from(11155111u64),
        services: vec![RpcApi {
            url: "https://eth-sepolia.g.alchemy.com/v2/NP2an-FMSHKAB1qV7U0vBZP6w4g5Yiir"
                .to_string(),
            headers: None,
        }],
    };

    let cycles = 2_000_000_000_000;
    let canister_id =
        Principal::from_text("7hfb6-caaaa-aaaar-qadga-cai").expect("principal should be valid");
    let rpc_config: Option<()> = None;
    let args = (rpc_services, rpc_config, raw_signed_transaction_hex);

    let result: (MultiSendRawTransactionResult,) =
        call_with_payment128(canister_id, "eth_sendRawTransaction", args, cycles)
            .await
            .expect("Failed to call eth_sendRawTransaction");

    match result.0 {
        MultiSendRawTransactionResult::Consistent(SendRawTransactionResult::Ok(status)) => {
            match status {
                SendRawTransactionStatus::Ok(Some(tx_hash)) => Ok(tx_hash),
                SendRawTransactionStatus::Ok(None) => {
                    Err("Transaction succeeded but tx_hash missing".to_string())
                }
                SendRawTransactionStatus::NonceTooLow => Err("Nonce too low".to_string()),
                SendRawTransactionStatus::NonceTooHigh => Err("Nonce too high".to_string()),
                SendRawTransactionStatus::InsufficientFunds => {
                    Err("Insufficient funds".to_string())
                }
            }
        }
        MultiSendRawTransactionResult::Consistent(SendRawTransactionResult::Err(e)) => {
            Err(format!("SendRawTransaction failed: {:?}", e))
        }
        MultiSendRawTransactionResult::Inconsistent(_) => {
            Err("Transaction is inconsistent".to_string())
        }
    }
}


#[update]
pub async fn get_exchange_rates(
    base_asset_symbol: String,
    quote_asset_symbol: Option<String>,
    amount: candid::Nat,
) -> Result<(String, u64), Error> {
    validate_asset_name(&base_asset_symbol)?;
    if let Some(ref symbol) = quote_asset_symbol {
        validate_asset_name(symbol)?;
    }

    let base_asset = normalize_symbol(&base_asset_symbol);
    let quote_asset = normalize_symbol(&quote_asset_symbol.unwrap_or_else(|| "USDT".to_string()));

    let args = GetExchangeRateRequest {
        timestamp: None,
        quote_asset: Asset {
            class: if quote_asset.to_uppercase() == "USD" {
                AssetClass::FiatCurrency
            } else {
                AssetClass::Cryptocurrency
            },
            symbol: quote_asset.clone(),
        },
        base_asset: Asset {
            class: if base_asset.to_uppercase() == "USD" {
                AssetClass::FiatCurrency
            } else {
                AssetClass::Cryptocurrency
            },
            symbol: base_asset.clone(),
        },
    };

    let xrc_canister_id: Principal = Principal::from_text("uf6dk-hyaaa-aaaaq-qaaaq-cai").unwrap();

    let res = ic_cdk::api::call::call_with_payment128(
        xrc_canister_id,
        "get_exchange_rate",
        (args,),
        XRC_CYCLES_FEE,
    )
    .await;

    let response = match res {
        Ok((GetExchangeRateResult::Ok(v),)) => v,
        Ok((GetExchangeRateResult::Err(e),)) => {
            return Err(Error::ExchangeRateError(format!(
                "Exchange rate error: {:?}",
                e
            )));
        }
        Err(e) => {
            return Err(Error::CallFailed(format!("Call failed: {:?}", e)));
        }
    };

    // let quote = response.rate;
    // let pow = 10u64.pow(response.metadata.decimals);
    // let exchange_rate = (candid::Nat::from(quote) * candid::Nat::from(SCALING_FACTOR)) / candid::Nat::from(pow);

    // ic_cdk::println!(" exchange_rate : {:?} ",exchange_rate);

    // let total_value: candid::Nat = (candid::Nat::from(quote) * amount) / candid::Nat::from(pow);
    // let time = current_timestamp();

    let quote = response.rate;
    let pow = 10u64.pow(response.metadata.decimals);

    let quote_f64 = quote as f64;
    let pow_f64 = pow as f64;
    let amount_f64 = amount.0.to_f64().unwrap_or(0.0);

    let total_value_f64 = (quote_f64 * amount_f64) / pow_f64;
    let time = current_timestamp();

    Ok((total_value_f64.to_string(), time))
}

export_candid!();
