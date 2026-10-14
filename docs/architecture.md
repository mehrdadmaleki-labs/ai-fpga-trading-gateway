# Architecture

## Principle

The project separates:

1. **Trading/order flow**
2. **Deterministic safety enforcement**
3. **AI risk intelligence**
4. **Future hardware acceleration**

```text
                  +--------------------+
                  |   AI Risk Model    |
                  | anomaly / regime   |
                  +---------+----------+
                            |
                    recommendation
                            |
                            v
                  +--------------------+
                  | Policy Validator   |
                  +---------+----------+
                            |
                    approved settings
                            |
                            v
Orders ----------> +--------------------+ ----------> Exchange simulator
                   | Rust Risk Gateway  |
                   +--------------------+
                            |
                            v
                     metrics / audit

Future:
selected Rust checks -> FPGA pipeline
```

## Why AI is not the final safety authority

AI is probabilistic. Hard risk limits should remain deterministic.

Example:

```text
AI says:
"Volatility looks abnormal; reduce max order value by 50%."

Policy validator says:
"That recommendation is within allowed configuration bounds."

Gateway/FPGA then enforces:
order_notional <= new_limit
```

## Software reference model

The Rust implementation should remain the behavioural reference even after FPGA work begins.

For every test vector:

```text
same order + same policy + same state
             |
       +-----+------+
       |            |
      Rust         FPGA
       |            |
       +-----+------+
             |
       same decision
```

This becomes one of the strongest engineering stories in the project.
