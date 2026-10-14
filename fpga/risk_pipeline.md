# Proposed FPGA Risk Pipeline

```text
Order Input
    |
    v
Field Validation
    |
    v
Notional Check
    |
    v
Position Check
    |
    v
Price Collar
    |
    v
Rate Limit
    |
    v
Kill-Switch Gate
    |
    v
Approve / Reject
```

## Important design question

Should checks run sequentially or in parallel?

### Sequential
Simpler, but latency can grow with each stage.

### Parallel
Several checks can evaluate at the same time, then combine:

```text
             +--> Notional ----+
             +--> Position ----+
Order ------>+--> Price -------+--> AND --> decision
             +--> Kill switch -+
```

This is one reason FPGA is interesting: independent checks can often be implemented as parallel hardware.

## TODO(HINT)

Do not optimize this yet.

First define:
- exact input widths,
- fixed-point/integer representation,
- overflow behaviour,
- rejection precedence,
- configuration-register interface,
- software/FPGA test vectors.
