# gbc-claim

Super Portable singular withdraw claim for Gnosis Chain validator withdrawals

## Run Claimer

### Docker Remote

```shell
docker run --rm -e PK=$PK ghcr.io/bh2smith/gbc-claim --account=$ETH1_WITHDRAWAL_ADDRESS --threshold=$MIN_WITHDRAW 
```

where

- `PK` is any private key with at least 0.01 (i.e. sufficient) xdai.
- `ETH1_WITHDRAWAL_ADDRESS` is the wallet you want to withdraw rewards for
- [optional] `MIN_WITHDRAW` (in WEI) is the minimum amount worth withdrawing (default is 1 GNO)


### Locally
```shell
cp .env.sample .env <-- fill out your credentials!
cargo run --account=$ETH1_WITHDRAWAL_ADDRESS --threshold=$MIN_WITHDRAW
```

### Local Docker

```shell
docker build -t gbc-claim
docker run --rm -e PK=$PK gbc-claim --account=$ETH1_WITHDRWAW_ADDRESS --threshold=$MIN_WITHDRAW
```

