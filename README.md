# gbc-claim

Super Portable singular withdraw claim for Gnosis Chain validator withdrawals

Checks how much GNO a withdrawal address can claim and, if it's at least the threshold, sends the
claim transaction and waits for it to be mined.

Anyone can send the claim: the signer only pays gas, and the GNO always goes to the withdrawal
address. Use a throwaway key with a little xDAI (0.01 is plenty), not your withdrawal address's key.

## Configuration

Every option can be passed as a flag or an environment variable, so a `.env` file works for both
Docker and local runs. Run with `--help` for details.

| Flag | Env var | Description |
|------|---------|-------------|
| `--account` | `ETH1_WITHDRAWAL_ADDRESS` | Withdrawal address to claim for (required) |
| `--threshold` | `MIN_WITHDRAW` | Minimum amount to claim, in wei (default 1 GNO) |
| `--rpc-url` | `ETH_RPC` | Gnosis Chain RPC (default `https://rpc.gnosischain.com`) |
| | `PK` | Private key of the signer that pays gas |
| `--keystore` | `KEYSTORE` | Encrypted keystore to sign with instead of `PK`: a file path, or a name in `~/.foundry/keystores` |
| | `KEYSTORE_PASSWORD` | Keystore password; prompted for when unset |

Without `PK` or `--keystore`, the tool runs read-only and only reports the claimable balance.

The tool exits non-zero if anything fails, including a reverted claim, so it is safe to run from
cron or a systemd timer.

## Run Claimer

```shell
cp .env.sample .env  # then fill it out
```

### Docker Remote

```shell
docker run --rm --env-file .env ghcr.io/bh2smith/gbc-claim
```

Flags work too, e.g. `docker run --rm -e PK ghcr.io/bh2smith/gbc-claim --account=$ETH1_WITHDRAWAL_ADDRESS`.
Passing `-e PK` without a value reads it from your shell, so the key doesn't end up in your shell
history.

### Locally

```shell
set -a; source .env; set +a
cargo run --release
```

Pass flags after `--`, e.g. `cargo run --release -- --account=0x... --threshold=500000000000000000`.

### Local Docker

```shell
docker build -t gbc-claim .
docker run --rm --env-file .env gbc-claim
```

## Signing with a keystore

To avoid keeping a raw private key in `.env`, import it into an encrypted keystore with
[Foundry](https://getfoundry.sh)'s `cast` and point the tool at it:

```shell
cast wallet import my-claimer --interactive
cargo run --release -- --account=$ETH1_WITHDRAWAL_ADDRESS --keystore my-claimer
```

The tool asks for the password, or reads it from `KEYSTORE_PASSWORD` for unattended runs. With
Docker, mount your keystores so names resolve the same way, and add `-it` for the password prompt:

```shell
docker run --rm -it --env-file .env -v ~/.foundry/keystores:/keystores:ro \
  ghcr.io/bh2smith/gbc-claim --keystore my-claimer
```
