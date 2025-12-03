use alloy::{
    primitives::{Address, U256},
    providers::{DynProvider, Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
    sol,
};
use anyhow::Result;
use clap::Parser;
use std::env;

const CLAIM_PROXY_ADDRESS: &str = "0x0B98057eA310F4d31F2a452B414647007d1645d9";
const DEFAULT_NODE_URL: &str = "https://rpc.gnosischain.com";

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
    let signer: Option<PrivateKeySigner> = match env::var("PK") {
        Ok(val) => match val.parse() {
            Ok(pk) => Some(pk),
            Err(_) => {
                eprintln!(
                    "⚠️  PK env var is set but invalid — running in read-only mode (cannot claim)"
                );
                None
            }
        },
        Err(_) => {
            eprintln!("⚠️  PK not set — running in read-only mode (cannot claim)");
            None
        }
    };
    let rpc_url = env::var("ETH_RPC")
        .unwrap_or(DEFAULT_NODE_URL.to_string())
        .parse()?;
    let provider: DynProvider = if let Some(ref pk) = signer {
        ProviderBuilder::new()
            .wallet(pk.clone())
            .connect_http(rpc_url)
            .erased()
    } else {
        ProviderBuilder::new().connect_http(rpc_url).erased()
    };
    let contract = DepositContract::new(CLAIM_PROXY_ADDRESS.parse::<Address>()?, provider);

    let reward = contract.withdrawableAmount(account).call().await?;
    let min_amount = U256::try_from(args.threshold)?;
    if reward.gt(&min_amount) {
        // Only try to claim if we actually have a signer
        if signer.is_none() {
            eprintln!(
                "⚠️  Reward ({}) is above threshold, but no PK is configured.\n\
                 🔒 Read-only mode: cannot send claim transaction.",
                pretty_u256(reward, 18),
            );
        } else {
            let tx = contract.claimWithdrawal(account).send().await?;
            println!(
                "claimed {} at: https://gnosisscan.io/tx/0x{:x}",
                pretty_u256(reward, 18),
                tx.tx_hash()
            );
        }
    } else {
        println!(
            "reward balance {} below threshold of {} GNO",
            pretty_u256(reward, 18),
            args.threshold as f64 / 1e18
        );
    }

    Ok(())
}
