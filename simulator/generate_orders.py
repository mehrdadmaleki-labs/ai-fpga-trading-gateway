"""
Generate synthetic orders for testing.

Version 1 prints CSV-like rows.
Later:
- output JSONL,
- send UDP packets,
- model bursts,
- model multiple symbols,
- model cancellations and fills.
"""

import random
import time


SYMBOLS = ["AAPL", "MSFT", "NVDA"]


def generate_order(order_id: int) -> dict:
    symbol = random.choice(SYMBOLS)
    side = random.choice(["Buy", "Sell"])

    base_price = {
        "AAPL": 20_000,
        "MSFT": 40_000,
        "NVDA": 13_000,
    }[symbol]

    price = base_price + random.randint(-50, 50)
    quantity = random.choice([10, 50, 100, 500, 1_000])

    return {
        "id": order_id,
        "symbol": symbol,
        "side": side,
        "quantity": quantity,
        "price_cents": price,
        "timestamp_ns": time.time_ns(),
    }


if __name__ == "__main__":
    for i in range(20):
        print(generate_order(i + 1))

# TODO(HINT):
# Your next integration task is to make this output JSONL
# and make gateway-rs read one Order per line from stdin.
