use alloy::{
    primitives::{Address, U256},
    providers::{DynProvider, Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
    sol,
};
use anyhow::{Context, Result, bail};
use clap::Parser;
use std::{env, path::PathBuf};

const CLAIM_PROXY_ADDRESS: &str = "0x0B98057eA310F4d31F2a452B414647007d1645d9";
const DEFAULT_NODE_URL: &str = "https://rpc.gnosischain.com";

/// Claims Gnosis Chain validator withdrawals for a withdrawal address.
///
/// The transaction signer only pays gas; claimed GNO always goes to the withdrawal address.
/// Provide a signer with the PK env var or --keystore. Without one, the tool only reports
/// the claimable balance.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Args {
    /// Withdrawal address to claim for
    #[arg(short, long, env = "ETH1_WITHDRAWAL_ADDRESS")]
    account: Address,

    /// Minimum amount to claim, in wei (default is 1 GNO)
    #[arg(
        short,
        long,
        env = "MIN_WITHDRAW",
        default_value = "1000000000000000000"
    )]
    threshold: U256,

    /// Gnosis Chain RPC URL
    #[arg(long, env = "ETH_RPC", default_value = DEFAULT_NODE_URL)]
    rpc_url: String,

    /// Encrypted keystore to sign with: a file path, or a name in ~/.foundry/keystores
    /// (as created by `cast wallet import`). Reads the password from KEYSTORE_PASSWORD or prompts.
    #[arg(long, env = "KEYSTORE")]
    keystore: Option<String>,
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

fn gno(value: U256) -> String {
    format!("{} GNO", pretty_u256(value, 18))
}

fn load_signer(keystore: Option<&str>) -> Result<Option<PrivateKeySigner>> {
    let pk = env::var("PK").ok().filter(|pk| !pk.is_empty());
    match (pk, keystore) {
        (Some(_), Some(_)) => bail!("both PK and --keystore are set; use only one"),
        (Some(pk), None) => Ok(Some(pk.parse().context("PK is not a valid private key")?)),
        (None, Some(keystore)) => {
            let mut path = PathBuf::from(keystore);
            if !path.exists() && !keystore.contains('/') {
                let home = env::var("HOME").context("HOME is not set")?;
                path = PathBuf::from(home)
                    .join(".foundry/keystores")
                    .join(keystore);
            }
            if !path.exists() {
                bail!("keystore {path:?} not found");
            }
            let password = match env::var("KEYSTORE_PASSWORD") {
                Ok(password) => password,
                Err(_) => rpassword::prompt_password("Keystore password: ")
                    .context("could not read the keystore password; set KEYSTORE_PASSWORD")?,
            };
            let signer = PrivateKeySigner::decrypt_keystore(&path, password)
                .with_context(|| format!("could not decrypt keystore {path:?}"))?;
            Ok(Some(signer))
        }
        (None, None) => Ok(None),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let signer = load_signer(args.keystore.as_deref())?;
    let rpc_url = args.rpc_url.parse().context("invalid RPC URL")?;
    let provider: DynProvider = match signer {
        Some(ref signer) => ProviderBuilder::new()
            .wallet(signer.clone())
            .connect_http(rpc_url)
            .erased(),
        None => ProviderBuilder::new().connect_http(rpc_url).erased(),
    };
    let contract = DepositContract::new(CLAIM_PROXY_ADDRESS.parse::<Address>()?, provider);

    let reward = contract.withdrawableAmount(args.account).call().await?;
    if reward.is_zero() {
        println!("nothing to claim for {}", args.account);
        return Ok(());
    }
    if reward < args.threshold {
        println!(
            "reward balance {} below threshold of {}",
            gno(reward),
            gno(args.threshold)
        );
        return Ok(());
    }

    if signer.is_none() {
        eprintln!(
            "⚠️  Reward ({}) is at or above the threshold, but no signer is configured.\n\
             🔒 Read-only mode: set PK or --keystore to send the claim transaction.",
            gno(reward),
        );
        return Ok(());
    }

    let pending = contract.claimWithdrawal(args.account).send().await?;
    let tx_url = format!("https://gnosisscan.io/tx/{}", pending.tx_hash());
    println!("sent claim for {}: {tx_url}", gno(reward));
    let receipt = pending
        .get_receipt()
        .await
        .context("claim transaction was sent but its receipt could not be fetched")?;
    if !receipt.status() {
        bail!("claim transaction reverted: {tx_url}");
    }
    println!(
        "claimed {} in block {}",
        gno(reward),
        receipt.block_number.unwrap_or_default()
    );

    Ok(())
}
