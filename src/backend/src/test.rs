
// eth_sendRawTransaction : (RpcServices, opt RpcConfig, rawSignedTransactionHex : text) -> (MultiSendRawTransactionResult);

// type ProviderId = nat64;

// type EthSepoliaService = variant {
//   Alchemy;
//   Ankr;
//   BlockPi;
//   PublicNode;
//   Sepolia;
// };

// type RpcService = variant {
//   Provider : ProviderId;
//   Custom : RpcApi;
//   EthSepolia : EthSepoliaService;
//   EthMainnet : EthMainnetService;
//   ArbitrumOne : L2MainnetService;
//   BaseMainnet : L2MainnetService;
//   OptimismMainnet : L2MainnetService;
// };

// type SendRawTransactionStatus = variant {
//   Ok : opt text;
//   NonceTooLow;
//   NonceTooHigh;
//   InsufficientFunds;
// };

// type SendRawTransactionResult = variant {
//   Ok : SendRawTransactionStatus;
//   Err : RpcError;
// };

// type HttpHeader = record { value : text; name : text };

// type ChainId = nat64;

// type MultiSendRawTransactionResult = variant {
//   Consistent : SendRawTransactionResult;
//   Inconsistent : vec record { RpcService; SendRawTransactionResult };
// };

// type RpcApi = record { url : text; headers : opt vec HttpHeader };

// type RpcServices = variant {
//   Custom : record {
//     chainId : ChainId;
//     services : vec RpcApi;
//   };
//   EthSepolia : opt vec EthSepoliaService;
//   EthMainnet : opt vec EthMainnetService;
//   ArbitrumOne : opt vec L2MainnetService;
//   BaseMainnet : opt vec L2MainnetService;
//   OptimismMainnet : opt vec L2MainnetService;
// };

// use candid::{CandidType, Principal};
// use ic_cdk::api::call::call_with_payment128;
// use ic_cdk::{export_candid, update};
// use serde::{Deserialize, Serialize};
// use std::cell::RefCell;

// const EVM_CANISTER_ID: &str = "7hfb6-caaaa-aaaar-qadga-cai";
// const GAS_FEE: u128 = 1_000_000_000_000;

// #[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
// enum EthSepoliaService {
//     Alchemy,
//     Ankr,
//     BlockPi,
//     PublicNode,
//     Sepolia,
// }

// #[derive(CandidType, Serialize, Deserialize, Debug, Clone)]
// enum RpcServices {
//     EthSepolia(Option<Vec<EthSepoliaService>>),
// }

// #[update]
// async fn send_raw_transaction(raw_signed_tx : String) {
//     // Define the RPC services (e.g., using Sepolia testnet)
//     let rpc_services = RpcServices::EthSepolia(Some(vec![EthSepoliaService::Sepolia]));

//     // Call the Ethereum canister to send the raw transaction
//     let tx_result: Result<(MultiSendRawTransactionResult,), _> = call_with_payment128(
//         Principal::from_text(EVM_CANISTER_ID).unwrap(),
//         "eth_sendRawTransaction",
//         (rpc_services, None::<RpcConfig>, raw_signed_tx.to_string()),
//         GAS_FEE,
//     )
//     .await;

//     match tx_result {
//         Ok((MultiSendRawTransactionResult::Consistent(result),)) => match result {
//             SendRawTransactionResult::Ok(status) => match status {
//                 SendRawTransactionStatus::Ok(Some(tx_hash)) => {
//                     ic_cdk::println!("Transaction sent successfully. Hash: {}", tx_hash)
//                 }
//                 SendRawTransactionStatus::Ok(None) => {
//                     ic_cdk::println!("Transaction sent, but no hash returned.")
//                 }
//                 SendRawTransactionStatus::NonceTooLow => {
//                     ic_cdk::println!("Transaction failed: Nonce too low.")
//                 }
//                 SendRawTransactionStatus::NonceTooHigh => {
//                     ic_cdk::println!("Transaction failed: Nonce too high.")
//                 }
//                 SendRawTransactionStatus::InsufficientFunds => {
//                     ic_cdk::println!("Transaction failed: Insufficient funds.")
//                 }
//             },
//             SendRawTransactionResult::Err(err) => {
//                 ic_cdk::println!("Transaction error: {:?}", err)
//             }
//         },
//         Ok((MultiSendRawTransactionResult::Inconsistent(results),)) => {
//             ic_cdk::println!("Inconsistent results from RPC services:");
//             for (service, result) in results {
//                 ic_cdk::println!("Service: {:?}, Result: {:?}", service, result);
//             }
//         }
//         Err(e) => ic_cdk::println!("Call error: {:?}", e),
//     }
// }

// export_candid!();
