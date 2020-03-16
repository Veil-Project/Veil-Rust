use std::collections::HashMap;

use veil_core_rpc::{client::Client, request::Request, veild_structs::*};

use serde_json;

// NOTE would be neat if I incorporated the parse to type, like serde_json.
// Might solve some issues...?
// i.e `let chain_tips: Vec<ChainTip> = serde_json::from_value(value).unwrap();`

// #[test]
// fn request_block_header() {
//     let hash =
// "8f465702cd6e847ec86a4c901a551e17ea619f38b8447c55120fe38093ada104";

//     let mut rpc = Client::new().unwrap();
//     let req = Request::block_header(hash, None);

//     println!("Header: {:?}", req);

//     let res = rpc.request(req).unwrap();

//     println!("Res: {:?}", res);
// }

// #[test]
// fn request_best_block_hash() {
//     let req_kind = RequestKind::BestBlockHash;
//     let req = Request::new(req_kind).unwrap();
//     let mut rpc = Client::new().unwrap();
//     let value = rpc.request(req).unwrap();

//     println!("RES: {:?}", value.as_u64());
// }

// #[test]
// fn request_block() {
//     let req_kind = RequestKind::Block(
//         "8f465702cd6e847ec86a4c901a551e17ea619f38b8447c55120fe38093ada104".
// to_owned(),     );
//     let req = Request::new(req_kind).unwrap();
//     let mut rpc = Client::new().unwrap();
//     let value = rpc.request(req).unwrap();
//     let block = Block::new_from_value(value).unwrap();
//     // let header = block.header();
//     // println!("VALUE: {:?}", value);
//     println!("RES: {:#?}", block);
//     // println!("{:#?}", header);
// }

// #[test]
// fn request_blockchain_info() {
//     let req_kind = RequestKind::BlockchainInfo;
//     let req = Request::new(req_kind).unwrap();
//     let mut rpc = Client::new().unwrap();
//     let value = rpc.request(req).unwrap();
//     // println!("value: {:?}", value);
//     let blockchain_info = BlockchainInfo::new_from_value(value);

//     // println!("RES: {:?}", value);
//     println!("{:?}", blockchain_info);
// }

// #[test]
// fn request_block_count() {
//     let req_kind = RequestKind::BlockCount;
//     let req = Request::new(req_kind).unwrap();
//     let mut rpc = Client::new().unwrap();
//     let value = rpc.request(req).unwrap();
//     // println!("value: {:?}", value);
//     let block_count = value.as_u64().unwrap();

//     println!("{}", block_count);
// }

// #[test]
// fn request_block_hash() {
//     let req_kind = RequestKind::BlockHash(393132);
//     let req = Request::new(req_kind).unwrap();
//     let mut rpc = Client::new().unwrap();
//     let value = rpc.request(req).unwrap();
//     // println!("value: {:?}", value);
//     let block_hash = value.to_string();

//     println!("{}", block_hash);
// }

// #[test]
// fn request_chain_tips() {
//     let req_kind = RequestKind::ChainTips;
//     let req = Request::new(req_kind).unwrap();
//     let mut rpc = Client::new().unwrap();
//     let value = rpc.request(req).unwrap();
//     // println!("value: {:#?}", value);
//     let chain_tips: Vec<ChainTip> = serde_json::from_value(value).unwrap();

//     println!("{:?}", chain_tips);
// }

// #[test]
// fn request_chain_tx_stats() {
//     let mut rpc = Client::new().unwrap();

//     let window_size = 1000;
//     let block_hash =
// "8f465702cd6e847ec86a4c901a551e17ea619f38b8447c55120fe38093ada104".
// to_owned();

//     let kind_no_options = RequestKind::ChainTxStats(None, None);
//     let kind_w_both = RequestKind::ChainTxStats(Some(window_size),
// Some(block_hash.to_owned()));     let kind_w_window_size =
// RequestKind::ChainTxStats(Some(window_size), None);     let kind_w_block_hash
// = RequestKind::ChainTxStats(None, Some(block_hash));

//     let req_no_options = Request::new(kind_no_options).unwrap();
//     let req_w_both = Request::new(kind_w_both).unwrap();
//     let req_w_window_size = Request::new(kind_w_window_size).unwrap();
//     let req_w_block_hash = Request::new(kind_w_block_hash).unwrap();

//     let value_no_options = rpc.request(req_no_options).unwrap();
//     let value_w_both = rpc.request(req_w_both).unwrap();
//     let value_w_window_size = rpc.request(req_w_window_size).unwrap();
//     let value_w_block_hash = rpc.request(req_w_block_hash).unwrap();

//     let ctxs_no_options = ChainTxStats::new_from_value(value_no_options);
//     let ctxs_w_both = ChainTxStats::new_from_value(value_w_both);
//     let ctxs_w_window_size =
// ChainTxStats::new_from_value(value_w_window_size);     let ctxs_w_block_hash
// = ChainTxStats::new_from_value(value_w_block_hash);

//     println!("No Options: {:#?}", ctxs_no_options);
//     println!("With Both: {:#?}", ctxs_w_both);
//     println!("With Window Size: {:#?}", ctxs_w_window_size);
//     println!("With Block Hash: {:#?}", ctxs_w_block_hash);
// }

// #[test]
// fn request_mempool_info() {
//     let mut rpc = Client::new().unwrap();
//     let kind = RequestKind::MempoolInfo;
//     let req = Request::new(kind).unwrap();
//     let value = rpc.request(req).unwrap();
//     let mempool_info = MempoolInfo::new_from_value(value);

//     println!("{:#?}", mempool_info);
// }

// #[test]
// fn request_raw_mempool() {
//     let mut rpc = Client::new().unwrap();
//     let kind = RequestKind::RawMempool;
//     let req = Request::new(kind).unwrap();
//     let value = rpc.request(req).unwrap();
//     // println!("{:#?}", value);
//     let raw_mempool: HashMap<String, MempoolTx> =
// serde_json::from_value(value).unwrap();

//     println!("{:#?}", raw_mempool);
// }

// // TODO test other mempool stuff when I make the fake Client.

// #[test]
// fn request_tx_out() {
//     let mut rpc = Client::new().unwrap();

//     let tx_id =
// "dbe7ee2c86a2ca0bc23b68efaa7ef266605437400c41cb34992afddc782bbb76".
// to_owned();     let n = 0;

//     let kind = RequestKind::TxOut(tx_id, n);
//     let req = Request::new(kind).unwrap();
//     // println!("Request: {:?}", req);
//     let value = rpc.request(req).unwrap();

//     println!("{:?}", value);
// }

// #[test]
// fn request_tx_out_proof() {
//     let mut rpc = Client::new().unwrap();

//     let tx_ids = vec![
//         "370458c5672c2b383a4a9cc8f6953c4ab22f13ab3113ae1106582c2f78e90066".
// to_owned(),
//         "ffca5176ae8d8c48a6a952ad88d857121efd995043a0aee476beaa2f84f08122".
// to_owned(),
//         "dbe7ee2c86a2ca0bc23b68efaa7ef266605437400c41cb34992afddc782bbb76".
// to_owned(),     ];
//     let block_height =
//         "ff66f3eb651bf0c61261ab2c583952a4b1522b2fc72eed7ba98881d3db0c9c3f".
// to_owned();

//     let kind = RequestKind::TxOutProof(tx_ids, Some(block_height));
//     let req = Request::new(kind).unwrap();
//     let value = rpc.request(req).unwrap();
//     let proof = value.to_string();

//     println!("{}", proof);
// }

// #[test]
// fn request_txout_set_info() {
//     let mut rpc = Client::new().unwrap();
//     let kind = RequestKind::TxOutSetInfo;
//     let req = Request::new(kind).unwrap();
//     let value = rpc.request(req).unwrap();
//     let txout_set_info = TxOutSetInfo::new_from_value(value);

//     println!("{:#?}", txout_set_info);
// }

// #[test]
// fn request_zerocoin_supply() {
//     let mut rpc = Client::new().unwrap();
//     let height = 393132;
//     let kind = RequestKind::ZerocoinSupply(Some(height));
//     let req = Request::new(kind).unwrap();
//     let value = rpc.request(req).unwrap();
//     let zerocoin_supply: Vec<DenomInfo> =
// serde_json::from_value(value).unwrap();

//     println!("{:#?}", zerocoin_supply);
// }

// #[test]
// fn request_memory_info() {
//     let mut rpc = Client::new().unwrap();
//     let kind = RequestKind::MemoryInfo;
//     let req = Request::new(kind).unwrap();
//     let value = rpc.request(req).unwrap();
//     let memory_info = MemoryInfo::new_from_value(value);

//     println!("{:#?}", memory_info);
// }

// #[test]
// fn request_network_info() {
//     let mut rpc = Client::new().unwrap();
//     let kind = RequestKind::NetworkInfo;
//     let req = Request::new(kind).unwrap();
//     let value = rpc.request(req).unwrap();
//     // println!("{:#?}", value);
//     let network_info = NetworkInfo::new_from_value(value).unwrap();

//     println!("{:#?}", network_info);
// }

// #[test]
// fn peer_info() {
//     let mut rpc = Client::new().unwrap();
//     let kind = RequestKind::PeerInfo;
//     let req = Request::new(kind).unwrap();
//     let value = rpc.request(req).unwrap();
//     // println!("{:?}", value);
//     let peer_info: Vec<PeerInfo> = serde_json::from_value(value).unwrap();

//     println!("{:#?}", peer_info);
// }

// #[test]
// fn banned() {
//     let mut rpc = Client::new().unwrap();
//     let kind = RequestKind::Banned;
//     let req = Request::new(kind).unwrap();
//     let value = rpc.request(req).unwrap();
//     let banned: Vec<PeerInfo> = serde_json::from_value(value).unwrap();

//     println!("{:#?}", banned)
// }

// #[test]
// fn raw_transaction() {
//     let tx =
// "da6ebf52aacbb3247f442525baac97323dda87b127f838d83c823d62b51ea557".
// to_owned();

//     let mut rpc = Client::new().unwrap();
//     let kind = RequestKind::RawTransaction(tx, None);
//     let req = Request::new(kind).unwrap();
//     let value = rpc.request(req).unwrap();
//     let raw_transaction = Tx::new_from_value(value);

//     println!("{:#?}", raw_transaction);
// }

// // #[test]
// // fn addresses_by_label() {
// //     let label = "".to_owned();
// //     let mut rpc = Client::new().unwrap();
// //     let kind = RequestKind::AddressesByLabel(label);
// //     let req = Request::new(kind).unwrap();
// //     let value = rpc.request(req).unwrap();
// //     println!("{:#?}", value);
// //     // let addresses: Vec<HashMap<String, HashMap<String, String>>> =
// //     //     serde_json::from_value(value).unwrap();

// //     // println!("{:#?}", addresses);
// // }

// #[test]
// fn address_info() {
//     let address = "bv1q7kpd4qkrye9nrjszntaychlcvhuqmcymcnxtca".to_owned();

//     let mut rpc = Client::new().unwrap();
//     let kind = RequestKind::AddressInfo(address);
//     let req = Request::new(kind).unwrap();
//     let value = rpc.request(req).unwrap();
//     let address_info = AddressInfo::new_from_value(value);

//     println!("{:#?}", address_info);
// }
