# gbc-claim

Portable singular withdraw claim for Gnosis Chain validator withdrawals

Run Locally:

```shell
cp .env.sample .env <-- fill out your credentials!
python -m src.claim_withdrawal --account=$ETH1_WITHDRAWAL_ADDRESS --threshold=100000000000000000
```
