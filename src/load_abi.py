"""Basic Contract ABI loader (from json files)"""

import json
import os
from typing import Any

from src.constants import ABI_PATH


def load_contract_abi(abi_name: str) -> Any:
    """Loads a contract abi from json file"""
    with open(
        os.path.join(ABI_PATH, f"{abi_name}.json"), "r", encoding="utf-8"
    ) as file:
        return json.load(file)
