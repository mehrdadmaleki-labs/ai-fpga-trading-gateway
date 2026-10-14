# AI-Assisted Low-Latency Trading Gateway

A learning and portfolio project that demonstrates a path from:

**Rust software risk gateway → AI-assisted risk intelligence → future FPGA acceleration**

> Important: this repository is infrastructure research and is **not** a profitable trading strategy or production trading system.

## Core idea

```text
Synthetic Orders / Strategy
          |
          v
+-------------------------+
| Rust Pre-Trade Gateway  |
|-------------------------|
| Position limit          |
| Order value limit       |
| Price collar            |
| Rate limit              |
| Kill switch             |
+------------+------------+
             |
        Approve / Reject
             |
             v
     Exchange Simulator

AI side-channel:
market/order metrics -> anomaly model -> policy recommendation

Future:
replace selected Rust checks with FPGA implementations.
```

## Repository structure

```text
ai-fpga-trading-gateway/
├── Cargo.toml
├── gateway-rs/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── order.rs
│   │   ├── policy.rs
│   │   ├── state.rs
│   │   ├── engine.rs
│   │   └── risk/
│   │       ├── mod.rs
│   │       └── checks.rs
│   └── tests/
│       └── risk_engine.rs
├── ai-risk/
│   ├── requirements.txt
│   ├── anomaly.py
│   └── policy_recommender.py
├── simulator/
│   └── generate_orders.py
├── config/
│   └── policy.example.toml
├── docs/
│   ├── architecture.md
│   └── milestones.md
└── fpga/
    ├── README.md
    └── risk_pipeline.md
```

## First milestone

Get this flow working:

```text
Order
  -> Rust RiskEngine
      -> max order value
      -> max position
      -> price collar
      -> kill switch
  -> Decision
```

Run:

```bash
cargo test
cargo run -p gateway-rs
```

## Second milestone

Generate synthetic events:

```bash
python3 simulator/generate_orders.py
```

Then connect those events to the Rust gateway.

## Third milestone

Run the AI anomaly prototype:

```bash
python3 -m venv .venv
source .venv/bin/activate
pip install -r ai-risk/requirements.txt
python3 ai-risk/anomaly.py
```

## Fourth milestone

Benchmark the Rust critical path and record:

- median latency
- p95
- p99
- p99.9
- throughput
- allocations/order

Only publish numbers you actually measured.

## Fifth milestone

Move one simple check to FPGA first:

```text
order.price * order.quantity <= MAX_ORDER_VALUE
```

Keep the Rust implementation as the reference model and verify that Rust and FPGA produce identical decisions.

## Suggested GitHub description

> AI-assisted low-latency pre-trade risk gateway in Rust, with configurable policy checks, anomaly detection, benchmarking, and an FPGA acceleration roadmap.
