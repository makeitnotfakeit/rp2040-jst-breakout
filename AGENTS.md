# Agent Directives & Engineering Rules

This repository maintains the hardware design for the **RP2040 JST-GH Breakout Board**. Any autonomous agent or human developer contributing to this repository must follow these rules:

1. **Clean-Room Enforcement**:
   - Zero GPL / LGPL / AGPL data or code.
   - Never import footprints or symbols from KiCad standard libraries without complete re-authoring from raw manufacturer datasheets under MIT/Apache-2.0.
2. **Unified Pinout Rules**:
   - Every JST GH connector must adhere to:
     - **Pin 1**: `5V` (VCC)
     - **Pins 2 to N-1**: Signals (IOs / UART / ADC)
     - **Pin N**: `GND`
   - Exception: None. The polarity is invariant across all multi-pin ports.
3. **Parallel Connectivity**:
   - Every GPIO net on the RP2040 is connected in parallel across 4-pin pairs, 6-pin quads, 10-pin octals, and 13-pin wide connectors.
   - When routing or netlisting, verify continuity so that no net is orphaned.
4. **Git-Friendly Storage**:
   - All symbol and footprint definitions follow standard JSON format (`pcbcraft-symbol/1` and `pcbcraft-footprint/1`) with deterministic formatting.
