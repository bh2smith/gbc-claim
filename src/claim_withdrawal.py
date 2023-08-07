"""Script to Check or Claim GNO for Gnosis Beacon Chain Validators"""
import argparse
import os

from dotenv import load_dotenv
from eth_account.signers.local import LocalAccount
from web3 import Web3

from src.constants import CLAIM_PROXY_ADDRESS, NODE_URL
from src.load_abi import load_contract_abi


class RewardClaimer:
    """Implements elementary Claim Contract interactions (check balance and claim_withdrawal)"""

    def __init__(self, account: str):
        web3 = Web3(Web3.HTTPProvider(NODE_URL))
        self.web3 = web3
        self.claim_contract = web3.eth.contract(
            address=web3.to_checksum_address(CLAIM_PROXY_ADDRESS),
            abi=load_contract_abi("SBCDepositContract"),
        )
        self.account = web3.to_checksum_address(account)

    def claim_withdrawal(self) -> None:
        """ClaimWithdrawal for `self.account`"""
        web3 = self.web3
        load_dotenv()
        private_key = os.environ["PK"]
        caller: LocalAccount = web3.eth.account.from_key(private_key)
        tx_dict = self.claim_contract.functions.claimWithdrawal(
            self.account
        ).build_transaction(
            {
                "chainId": 100,
                "from": caller.address,
                "nonce": web3.eth.get_transaction_count(caller.address),
            }
        )

        signed_tx = web3.eth.account.sign_transaction(tx_dict, private_key)
        send_tx = web3.eth.send_raw_transaction(signed_tx.rawTransaction)

        # Wait for transaction receipt
        tx_receipt = web3.eth.wait_for_transaction_receipt(send_tx)
        print(f"claimed at txHash: 0x{tx_receipt['transactionHash'].hex()}")

    def check_reward(self) -> int:
        """Check Reward balance of `self.account`"""
        amount: int = self.claim_contract.functions.withdrawableAmount(
            self.account
        ).call()
        print(f"withdrawable amount={amount / pow(10, 18)} GNO")
        return amount

    def try_claim(self, min_amount: int) -> None:
        """Claims if balance exceeds `min_amount` threshold"""
        # Read balance and compare with threshold
        if self.check_reward() > min_amount:
            print(f"invoking claimWithdrawal for {self.account}...")
            self.claim_withdrawal()
        else:
            print(f"reward balance below minimum withdraw of {min_amount/ 1e18} GNO")


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
    claimer = RewardClaimer(account=args.account)
    claimer.try_claim(min_amount=args.threshold)
