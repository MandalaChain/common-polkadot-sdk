use jsonrpsee::http_client::HttpClientBuilder;
use pallet_revive::{
	create1,
	evm::{BlockTag, Bytes, EthInstantiateInput, ReceiptInfo, U256},
};
use pallet_revive_eth_rpc::{
	example::{wait_for_receipt, Account},
	EthRpcClient,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	env_logger::init();
	let account = Account::default();
	let data = vec![];
	let (bytes, _) = pallet_revive_fixtures::compile_module("dummy")?;
	let input = EthInstantiateInput { code: bytes, data: data.clone() };

	println!("Account address: {:?}", account.address());
	let client = HttpClientBuilder::default().build("http://localhost:9090".to_string())?;

	println!("\n\n=== Deploying contract ===\n\n");

	let input = rlp::encode(&input).to_vec();
	let nonce = client.get_transaction_count(account.address(), BlockTag::Latest.into()).await?;
	let hash = account.send_transaction(&client, U256::zero(), input.into(), None).await?;

	println!("Deploy Tx hash: {hash:?}");
	let ReceiptInfo { block_number, gas_used, contract_address, .. } =
		wait_for_receipt(&client, hash).await?;
	println!("Receipt:");
	println!("- Block number: {block_number}");
	println!("- Gas used: {gas_used}");
	println!("- Contract address: {contract_address:?}");

	let contract_address = create1(&account.address(), nonce.try_into().unwrap());
	println!("\n\n=== Calling contract ===\n\n");

	let hash = account
		.send_transaction(&client, U256::zero(), Bytes::default(), Some(contract_address))
		.await?;

	println!("Contract call tx hash: {hash:?}");
	let ReceiptInfo { block_number, gas_used, to, .. } = wait_for_receipt(&client, hash).await?;
	println!("Receipt:");
	println!("- Block number: {block_number}");
	println!("- Gas used: {gas_used}");
	println!("- To: {to:?}");
	Ok(())
}