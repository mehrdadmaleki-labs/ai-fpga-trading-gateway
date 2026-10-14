# Implementation Milestones

## M1 — Core Risk Engine
Complete:
- order model
- risk policy
- current positions
- order-value check
- position check
- price collar
- kill switch
- unit tests

## M2 — Configurable Gateway
Implement:
- TOML policy loading
- policy validation
- JSONL input
- decision output
- structured logs

## M3 — Stateful Trading Simulation
Implement:
- fills
- partial fills
- open orders
- cancellations
- position reservation
- multiple symbols

## M4 — Rate Limiting
Implement:
- per-strategy counters
- fixed time window first
- then compare with token-bucket/sliding-window approaches

### Hint
Do not call `Instant::now()` many times blindly in the critical path before measuring its cost.

## M5 — Benchmarks
Measure:
- check latency
- full gateway latency
- throughput
- p50 / p95 / p99 / p99.9
- effect of allocations
- effect of logging

## M6 — AI Risk Intelligence
Implement:
- feature windows
- scaler + IsolationForest
- anomaly score
- regime label
- policy recommendation
- policy validation

## M7 — Low-Latency Refactor
Investigate:
- preallocation
- fixed-capacity structures
- symbol IDs instead of String in hot path
- SPSC ring buffer
- CPU affinity
- eliminating avoidable allocations

## M8 — FPGA Boundary
Define an exact hardware interface:

Input:
- symbol_id
- side
- quantity
- price
- current_position
- configured limits

Output:
- approve/reject
- reject reason
- timestamp

Start with ONE rule:
`quantity * price <= max_order_notional`.

## M9 — Rust/FPGA Equivalence Tests
Generate thousands/millions of test vectors and require identical decisions.
