# RP2040 JST-GH Modular Breakout Board

An open-source, modular breakout and distribution carrier for the **Raspberry Pi Pico / Pico 2 (RP2040)** featuring hybrid high-voltage battery power, direct 5V bench power, dedicated dual UART telemetry ports, and a comprehensive parallel **JST GH** connector grid.

Designed in accordance with **Tangency Labs Clean-Room Engineering By-Laws** (100% clean-room derivation, zero GPL code, dual-licensed under **MIT OR Apache-2.0**).

---

## ⚡ 1. Power Architecture (Hybrid No-SMT Design)

The board provides clean, high-current power distribution using pure through-hole connectors and drop-in daughterboards:

1. **High-Voltage Battery In (XT30 & XT60)**:
   - Dedicated through-hole landing footprints for both **XT30** and **XT60** connectors.
   - Feeds a drop-in **Pololu Step-Down Voltage Regulator** daughterboard socket (standard $2.54\text{ mm}$ SIP header: `VIN`, `GND`, `VOUT`, `SHDN`).
   - Accepts $2\text{S} - 6\text{S}+$ LiPo battery packs ($7.4\text{ V} - 36\text{ V}$), stepping down to a high-efficiency $5.0\text{ V}$ system rail.
2. **Direct 5V Benchtop In (XT30 & XT60)**:
   - Secondary dedicated footprints for both **XT30** and **XT60** directly wired to the $5.0\text{ V}$ distribution bus and common ground for bench testing without the regulator module.
3. **Pico Power Integration**:
   - The $5.0\text{ V}$ bus feeds directly into Pico **VSYS** (Pin 39).
   - The Pico's on-board buck-boost/LDO supplies $3.3\text{ V}$ for RP2040 silicon and IO logic.

---

## 📐 2. Global Pinout Invariant

Across every single JST GH connector on the board, power polarity is **strictly uniform**:

$$\text{Pin 1} = \mathbf{5.0\text{ V} \text{ (VCC)}} \quad \big| \quad \text{Pins } 2 \dots (N-1) = \mathbf{\text{Signals / IOs}} \quad \big| \quad \text{Pin } N = \mathbf{\text{GND}}$$

This eliminates inverted-power plugging hazards across all harness sizes.

---

## 🔌 3. Connector Matrix & Parallel IO Grid

All Raspberry Pi Pico GPIO lines are bused in parallel across multiple connector form factors, allowing single 2-signal sensor cables, 4-signal quads, 8-signal octals, or wide multi-line wiring harnesses to plug into the same signals without wiring conflicts.

### Dedicated Dual UART Ports (2x Duplicate)
- **`J_UART1` (4-Pin JST GH)**: `[1: 5V, 2: GP0/UART0_TX, 3: GP1/UART0_RX, 4: GND]`
- **`J_UART2` (4-Pin JST GH)**: `[1: 5V, 2: GP0/UART0_TX, 3: GP1/UART0_RX, 4: GND]` (Duplicate telemetry/debug port)

### 4-Pin JST GH Grid (2x IO per connector)
| Connector | Pin 1 | Pin 2 | Pin 3 | Pin 4 | Primary Function |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **`J_A_PAIR0`** | 5V | **ADC0 (GP26)** | **ADC1 (GP27)** | GND | Analog Sensor Pair 1 |
| **`J_A_PAIR1`** | 5V | **ADC2 (GP28)** | **ADC_VREF (Pin 35)** | GND | Analog Sensor Pair 2 |
| **`J_D_PAIR0`** | 5V | **GP0** | **GP1** | GND | Digital Pair 0 (UART0) |
| **`J_D_PAIR1`** | 5V | **GP2** | **GP3** | GND | Digital Pair 1 (I2C1) |
| **`J_D_PAIR2`** | 5V | **GP4** | **GP5** | GND | Digital Pair 2 (UART1 / I2C0) |
| **`J_D_PAIR3`** | 5V | **GP6** | **GP7** | GND | Digital Pair 3 |
| **`J_D_PAIR4`** | 5V | **GP8** | **GP9** | GND | Digital Pair 4 |
| **`J_D_PAIR5`** | 5V | **GP10** | **GP11** | GND | Digital Pair 5 |
| **`J_D_PAIR6`** | 5V | **GP12** | **GP13** | GND | Digital Pair 6 |
| **`J_D_PAIR7`** | 5V | **GP14** | **GP15** | GND | Digital Pair 7 |
| **`J_D_PAIR8`** | 5V | **GP16** | **GP17** | GND | Digital Pair 8 (SPI0) |
| **`J_D_PAIR9`** | 5V | **GP18** | **GP19** | GND | Digital Pair 9 (SPI0) |
| **`J_D_PAIR10`** | 5V | **GP20** | **GP21** | GND | Digital Pair 10 |
| **`J_D_PAIR11`** | 5V | **GP22** | **RUN (Reset)** | GND | Digital Pair 11 & Hardware Reset |

### 6-Pin JST GH Quads (4x IO per connector)
- **`J_A_QUAD0`**: `[1: 5V, 2: ADC0, 3: ADC1, 4: ADC2, 5: ADC_VREF, 6: GND]`
- **`J_D_QUAD1`**: `[1: 5V, 2: GP0, 3: GP1, 4: GP2, 5: GP3, 6: GND]`
- **`J_D_QUAD2`**: `[1: 5V, 2: GP4, 3: GP5, 4: GP6, 5: GP7, 6: GND]`
- **`J_D_QUAD3`**: `[1: 5V, 2: GP8, 3: GP9, 4: GP10, 5: GP11, 6: GND]`
- **`J_D_QUAD4`**: `[1: 5V, 2: GP12, 3: GP13, 4: GP14, 5: GP15, 6: GND]`
- **`J_D_QUAD5`**: `[1: 5V, 2: GP16, 3: GP17, 4: GP18, 5: GP19, 6: GND]`
- **`J_D_QUAD6`**: `[1: 5V, 2: GP20, 3: GP21, 4: GP22, 5: RUN, 6: GND]`

### 10-Pin JST GH Octals (8x IO per connector)
- **`J_D_OCT1`**: `[1: 5V, 2: GP0, 3: GP1, 4: GP2, 5: GP3, 6: GP4, 7: GP5, 8: GP6, 9: GP7, 10: GND]`
- **`J_D_OCT2`**: `[1: 5V, 2: GP8, 3: GP9, 4: GP10, 5: GP11, 6: GP12, 7: GP13, 8: GP14, 9: GP15, 10: GND]`
- **`J_D_OCT3`**: `[1: 5V, 2: GP16, 3: GP17, 4: GP18, 5: GP19, 6: GP20, 7: GP21, 8: GP22, 9: RUN, 10: GND]`

### 13-Pin JST GH Wide Connectors (11x IO per connector)
- **`J_D_WIDE1`**: `[1: 5V, 2..12: GP0 .. GP10, 13: GND]`
- **`J_D_WIDE2`**: `[1: 5V, 2..12: GP11 .. GP21, 13: GND]`

---

## 📦 4. Bill of Materials (BOM)

| Designator | Description | Package / Mounting | Quantity |
| :--- | :--- | :--- | :--- |
| **`U_PICO`** | Raspberry Pi Pico / Pico 2 Socket | $2\times 20\text{ pin } 2.54\text{ mm}$ Female Header | 1 |
| **`U_POLOLU`** | Pololu D24Vxx / D36Vxx Step-Down Regulator | $1\times 4\text{ pin } 2.54\text{ mm}$ Female Header | 1 |
| **`J_XT30_HV`**, **`J_XT30_5V`** | Amass XT30PW / XT30 Through-Hole Male Plug | $5.0\text{ mm}$ Pitch Through-Hole | 2 |
| **`J_XT60_HV`**, **`J_XT60_5V`** | Amass XT60PW / XT60 Heavy Through-Hole Male Plug | $7.2\text{ mm}$ Pitch Through-Hole | 2 |
| **`J_UART1`**, **`J_UART2`** | JST GH 4-Pin Shrouded Header | $1.25\text{ mm}$ SMT Positive Lock | 2 |
| **`J_A_PAIR0..1`**, **`J_D_PAIR0..11`** | JST GH 4-Pin Shrouded Header | $1.25\text{ mm}$ SMT Positive Lock | 14 |
| **`J_A_QUAD0`**, **`J_D_QUAD1..6`** | JST GH 6-Pin Shrouded Header | $1.25\text{ mm}$ SMT Positive Lock | 7 |
| **`J_D_OCT1..3`** | JST GH 10-Pin Shrouded Header | $1.25\text{ mm}$ SMT Positive Lock | 3 |
| **`J_D_WIDE1..2`** | JST GH 13-Pin Shrouded Header | $1.25\text{ mm}$ SMT Positive Lock | 2 |

---

## ⚖️ License
Dual-licensed under **MIT OR Apache-2.0**.
