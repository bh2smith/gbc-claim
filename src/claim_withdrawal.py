import argparse
import os

from dotenv import load_dotenv
from eth_account.signers.local import LocalAccount
from eth_typing import ChecksumAddress
from web3.contract import Contract

from src.constants import CLAIM_PROXY_ADDRESS, NODE_URL
from src.load_abi import load_contract_abi

from web3 import Web3

web3 = Web3(Web3.HTTPProvider(NODE_URL))


def claim_withdrawal(account: ChecksumAddress, claim_contract: Contract):
    load_dotenv()
    pk = os.environ["PK"]
    caller: LocalAccount = web3.eth.account.from_key(pk)
    tx_dict = claim_contract.functions.claimWithdrawal(account).build_transaction(
        {
            "chainId": 100,
            "from": caller.address,
            "nonce": web3.eth.get_transaction_count(caller.address),
        }
    )

    signed_tx = web3.eth.account.sign_transaction(tx_dict, private_key=os.environ["PK"])
    send_tx = web3.eth.send_raw_transaction(signed_tx.rawTransaction)

    # Wait for transaction receipt
    tx_receipt = web3.eth.wait_for_transaction_receipt(send_tx)
    print(f"claimed at txHash: 0x{tx_receipt.transactionHash.hex()}")


def try_claim(account: ChecksumAddress, min_amount: int) -> bool:
    # load claim contract.
    claim_contract = web3.eth.contract(
        address=CLAIM_PROXY_ADDRESS, abi=load_contract_abi("SBCDepositContract")
    )
    # Read balance and compare with threshold
    amount = claim_contract.functions.withdrawableAmount(account).call()
    print(f"withdrawable amount={amount / pow(10, 18)} GNO")

    if amount > min_amount:
        print(f"calling claimWithdrawal({account})...")
        claim_withdrawal(account, claim_contract)
    else:
        print(f"amount does not exceed withdraw threshold, nothing to do.")
    return True


if __name__ == "__main__":
    parser = argparse.ArgumentParser("Script Arguments")
    parser.add_argument(
        "--account",
        type=str,
        required=True,
        help="Ethereum Address to claim withdraw for",
    )
    parser.add_argument(
        "--threshold",
        type=int,
        default=pow(10, 18),
        help="minimum amount to claim (default is 1 GNO)",
    )

    args, _ = parser.parse_known_args()
    try_claim(account=web3.to_checksum_address(args.account), min_amount=args.threshold)
