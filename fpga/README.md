# FPGA Roadmap

No FPGA implementation is claimed in Version 1.

This directory defines the future hardware boundary and test strategy.

## First FPGA target

Implement only:

```text
notional = price * quantity
approve if notional <= MAX_ORDER_NOTIONAL
```

Why start here?

- simple arithmetic
- easy to verify against Rust
- easy to benchmark
- teaches the complete FPGA workflow without pretending to have a full HFT stack

## Suggested future files

```text
fpga/
├── rtl/
│   └── notional_check.sv
├── tb/
│   └── notional_check_tb.sv
├── vectors/
│   └── notional_cases.csv
└── benchmark/
```

## TODO(HINT)

Before buying hardware, learn the tool flow using simulation:

1. SystemVerilog basics
2. RTL module
3. testbench
4. simulator
5. synthesis
6. timing report
7. only then target a development board
