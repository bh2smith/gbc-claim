# gbc-claim

Portable singular withdraw claim for Gnosis Chain validator withdrawals

## Run Claimer

```shell
usage: Script Arguments [-h] --account ACCOUNT [--threshold THRESHOLD]

options:
  -h, --help            show this help message and exit
  --account ACCOUNT     Ethereum Address to claim withdraw for
  --threshold THRESHOLD
                        minimum amount to claim (default is 1 GNO)
```


### Locally
```shell
cp .env.sample .env <-- fill out your credentials!
python -m src.claim_withdrawal --account=$ETH1_WITHDRAWAL_ADDRESS --threshold=$MIN_WITHDRAW
```

### Docker

```shell
docker build -t gbc-claim
docker run --env-file .env gbc-claim --account=$ETH1_WITHDRWAW_ADDRESS --threshold=$MIN_WITHDRAW
```