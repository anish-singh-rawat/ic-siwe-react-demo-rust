use serde::{Serialize, Deserialize};
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

#[derive(Serialize, Deserialize)]
pub enum EthMainnetService { Alchemy, Ankr, BlockPi, Cloudflare, PublicNode, Llama }
#[derive(Serialize, Deserialize)]
pub enum EthSepoliaService { Alchemy, Ankr, BlockPi, PublicNode, Sepolia }
#[derive(Serialize, Deserialize)]
pub enum L2MainnetService { Alchemy, Ankr, BlockPi, PublicNode, Llama }

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

#[derive(Serialize, Deserialize)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub enum HttpOutcallError {
    IcError { code: RejectionCode, message: String },
    InvalidHttpJsonRpcResponse { status: Nat16, body: String, parsing_error: Option<String> },
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

#[derive(Serialize, Deserialize)]
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
    pub requests: Vec<( (String, String), u64 )>,
    pub responses: Vec<( (String, String, String), u64 )>,
    pub inconsistent_responses: Vec<( (String, String), u64 )>,
    pub cycles_charged: Vec<( (String, String), Nat )>,
    pub err_http_outcall: Vec<( (String, String, RejectionCode), u64 )>,
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

#[derive(Serialize, Deserialize)]
pub enum MultiSendRawTransactionResult {
    Consistent(SendRawTransactionResult),
    Inconsistent(Vec<(RpcService, SendRawTransactionResult)>),
}

#[derive(Serialize, Deserialize)]
pub enum MultiCallResult {
    Consistent(CallResult),
    Inconsistent(Vec<(RpcService, CallResult)>),
}

#[derive(Serialize, Deserialize)]
pub enum ProviderError {
    TooFewCycles { expected: Nat, received: Nat },
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
    Authenticated { auth: RpcAuth, public_url: Option<String> },
    Unauthenticated { public_url: String },
}

#[derive(Serialize, Deserialize)]
pub enum RpcAuth {
    BearerToken { url: String },
    UrlParameter { url_pattern: String },
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub enum RpcError {
    JsonRpcError(JsonRpcError),
    ProviderError(ProviderError),
    ValidationError(ValidationError),
    HttpOutcallError(HttpOutcallError),
}

#[derive(Serialize, Deserialize)]
pub struct RpcApi {
    pub url: String,
    pub headers: Option<Vec<HttpHeader>>,
}

#[derive(Serialize, Deserialize)]
pub enum RpcService {
    Provider(ProviderId),
    Custom(RpcApi),
    EthSepolia(EthSepoliaService),
    EthMainnet(EthMainnetService),
    ArbitrumOne(L2MainnetService),
    BaseMainnet(L2MainnetService),
    OptimismMainnet(L2MainnetService),
}

#[derive(Serialize, Deserialize)]
pub enum RpcServices {
    Custom { chain_id: ChainId, services: Vec<RpcApi> },
    EthSepolia(Option<Vec<EthSepoliaService>>),
    EthMainnet(Option<Vec<EthMainnetService>>),
    ArbitrumOne(Option<Vec<L2MainnetService>>),
    BaseMainnet(Option<Vec<L2MainnetService>>),
    OptimismMainnet(Option<Vec<L2MainnetService>>),
}

#[derive(Serialize, Deserialize)]
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

// Finally: the service trait defining all methods
#[async_trait::async_trait]
pub trait EthService {
    async fn eth_fee_history(
        &self,
        services: RpcServices,
        config: Option<RpcConfig>,
        args: FeeHistoryArgs,
    ) -> MultiFeeHistoryResult;

    async fn eth_get_block_by_number(
        &self,
        services: RpcServices,
        config: Option<RpcConfig>,
        tag: BlockTag,
    ) -> MultiGetBlockByNumberResult;

    async fn eth_get_logs(
        &self,
        services: RpcServices,
        config: Option<RpcConfig>,
        args: GetLogsArgs,
    ) -> MultiGetLogsResult;

    async fn eth_get_transaction_count(
        &self,
        services: RpcServices,
        config: Option<RpcConfig>,
        args: GetTransactionCountArgs,
    ) -> MultiGetTransactionCountResult;

    async fn eth_get_transaction_receipt(
        &self,
        services: RpcServices,
        config: Option<RpcConfig>,
        hash: String,
    ) -> MultiGetTransactionReceiptResult;

    async fn eth_send_raw_transaction(
        &self,
        services: RpcServices,
        config: Option<RpcConfig>,
        raw_signed_tx: String,
    ) -> MultiSendRawTransactionResult;

    async fn eth_call(
        &self,
        services: RpcServices,
        config: Option<RpcConfig>,
        args: CallArgs,
    ) -> MultiCallResult;

    async fn request(
        &self,
        service: RpcService,
        json: String,
        max_response_bytes: Nat64,
    ) -> RequestResult;

    async fn request_cost(
        &self,
        service: RpcService,
        json: String,
        max_response_bytes: Nat64,
    ) -> RequestCostResult;

    async fn get_metrics(&self) -> Metrics;
    async fn get_nodes_in_subnet(&self) -> Nat32;
    async fn get_providers(&self) -> Vec<Provider>;
    async fn get_service_provider_map(&self) -> Vec<(RpcService, ProviderId)>;
    async fn update_api_keys(&self, keys: Vec<(ProviderId, Option<String>)>);
}
