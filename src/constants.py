"""Project static constants"""
from pathlib import Path

PROJECT_ROOT = Path(__file__).parent.parent
LOG_CONFIG_FILE = Path(__file__).parent.parent / Path("logging.conf")
ABI_PATH = PROJECT_ROOT / Path("abis")

CLAIM_PROXY_ADDRESS = "0x0B98057eA310F4d31F2a452B414647007d1645d9"

NODE_URL = " https://rpc.gnosischain.com"
