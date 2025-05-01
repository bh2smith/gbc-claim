use alloy::{
    primitives::{Address, U256},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    sol,
};
use anyhow::Result;
use clap::Parser;
use std::env;

const CLAIM_PROXY_ADDRESS: &str = "0x0B98057eA310F4d31F2a452B414647007d1645d9";
const NODE_URL: &str = "https://rpc.gnosischain.com";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Ethereum Address to claim withdraw for
    #[arg(short, long)]
    account: String,

    /// Minimum amount to claim (in wei, default is 1 GNO)
    #[arg(short, long, default_value_t = 1_000_000_000_000_000_000)]
    threshold: u128,
}

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    DepositContract,
    "abis/SBCDepositContract.json"
);

fn pretty_u256(value: U256, decimals: usize) -> String {
    let ten = U256::from(10);
    let base = ten.pow(U256::from(decimals));

    let whole = value / base;
    let frac = value % base;

    let mut frac_str = frac.to_string();

    // Pad with leading zeros if necessary
    if frac_str.len() < decimals {
        let pad = "0".repeat(decimals - frac_str.len());
        frac_str = format!("{pad}{frac_str}");
    }

    // Trim trailing zeros for pretty output
    let frac_str_trimmed = frac_str.trim_end_matches('0');

    if frac_str_trimmed.is_empty() {
        whole.to_string()
    } else {
        format!("{}.{}", whole, frac_str_trimmed)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let account = args.account.parse::<Address>()?;
    let signer: PrivateKeySigner = env::var("PK").unwrap().parse().unwrap();
    let provider = ProviderBuilder::new()
        .wallet(signer)
        .connect_http(NODE_URL.try_into()?);
    let contract = DepositContract::new(CLAIM_PROXY_ADDRESS.parse::<Address>()?, provider);

    let reward = contract.withdrawableAmount(account).call().await?;
    let min_amount = U256::try_from(args.threshold)?;
    if reward.gt(&min_amount) {
        // Try claim
        let tx = contract.claimWithdrawal(account).send().await?;
        println!("claimed at txHash: 0x{:x}", tx.tx_hash());
    } else {
        println!(
            "reward balance {} below minimum withdraw of {} GNO",
            pretty_u256(reward, 18),
            args.threshold as f64 / 1e18
        );
    }

    Ok(())
}
