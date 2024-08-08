
<h1 align="center">🌿 <i>SUDI</i> Keyboard V2</h1>
<p style="align: right;">
<img style="align: center; " src="./pcb-design/output/RedoxV2.png"> 
</p>

<p >
  <a href="">
    <img alt="npm version" src="https://badgen.net/github/commits/ahsanu123/sudi-keyboard/">
  </a>
  <a href="">
    <img alt="npm" src="https://badgen.net/github/contributors/ahsanu123/sudi-keyboard/">
  </a>
  <a href="">
    <img alt="npm" src="https://badgen.net/github/branches/ahsanu123/sudi-keyboard/">
  </a>
  <a href="https://github.com/ahsanu123/sudi-keyboard/blob/main/LICENSE">
    <img alt="licence" src="https://badgen.net/github/license/ahsanu123/sudi-keyboard/">
  </a>
</p>


Sudi V2 is custom wireless split keyboard based on NRF51822 Nordic Semiconductor Microcontroller series, each side consist 18 keys 36 in totals, use MCP23017 I2C I/O expansion to scan one side of keyboard, and use wire to trasfer data to NRF51.
Sudi Have interactive RGB LED and OLED 0.91 inch for graphic display. Planned to create 3D print enclosure.

## 🖨️ Compile And Flash

🧁 TODO: add compile step

There is several option to Flash binary file into NRF51, 

### use openocd + jlink
first connect into ic with this command, then you can use `program` command to load flash, [read more about flashing](https://openocd.org/doc/html/Flash-Programming.html)
```shell
sudo openocd -f interface/jlink.cfg  -c "transport select swd; adapter speed 1000" -f target/nrf51.cfg C
```

### use Jlink 
next, you can use Jlink cli tool, you can download tool from Segger Jlink webpage. 
run this command 
```shell
 
$ sudo JLinkExe
SEGGER J-Link Commander V7.92o (Compiled Nov  8 2023 15:46:41)
DLL version V7.92o, compiled Nov  8 2023 15:46:18

Connecting to J-Link via USB...O.K.
Firmware: J-Link ARM V8 compiled Nov 28 2014 13:44:46
Hardware version: V8.00
J-Link uptime (since boot): N/A (Not supported by this model)
S/N: 805251123
License(s): RDI,FlashDL,FlashBP,JFlash,GDB
VTref=3.358V


Type "connect" to establish a target connection, '?' for help
J-Link>connect
Please specify device / core. <Default>: NRF51822_XXAA
Type '?' for selection dialog
Device>
Please specify target interface:
  J) JTAG (Default)
  S) SWD
  T) cJTAG
TIF>S
Specify target interface speed [kHz]. <Default>: 4000 kHz
Speed>
Device "NRF51822_XXAA" selected.

Connecting to target via SWD
Found SW-DP with ID 0x0BB11477
DPv0 detected
CoreSight SoC-400 or earlier
Scanning AP map to find all available APs
AP[1]: Stopped AP scan as end of AP map has been reached
AP[0]: AHB-AP (IDR: 0x04770021)
Iterating through AP map to find AHB-AP to use
AP[0]: Core found
AP[0]: AHB-AP ROM base: 0xF0000000
CPUID register: 0x410CC200. Implementer code: 0x41 (ARM)
Found Cortex-M0 r0p0, Little endian.
FPUnit: 4 code (BP) slots and 0 literal slots
CoreSight components:
ROMTbl[0] @ F0000000
[0][0]: E00FF000 CID B105100D PID 000BB471 ROM Table
ROMTbl[1] @ E00FF000
[1][0]: E000E000 CID B105E00D PID 000BB008 SCS
[1][1]: E0001000 CID B105E00D PID 000BB00A DWT
[1][2]: E0002000 CID B105E00D PID 000BB00B FPB
[0][1]: F0002000 CID B105900D PID 000BB9A3 ???
Memory zones:
  Zone: "Default" Description: Default access mode
Cortex-M0 identified.
J-Link>halt
T-bit of XPSR is 0 but should be 1. Changed to 1.
PC = FFFFFFFE, CycleCnt = 00000000
R0 = FFFFFFFF, R1 = FFFFFFFF, R2 = FFFFFFFF, R3 = FFFFFFFF
R4 = FFFFFFFF, R5 = FFFFFFFF, R6 = FFFFFFFF, R7 = FFFFFFFF
R8 = FFFFFFFF, R9 = FFFFFFFF, R10= FFFFFFFF, R11= FFFFFFFF
R12= FFFFFFFF
SP(R13)= E92D3FF0, MSP= E92D3FF0, PSP= FFFFFFFC, R14(LR) = FFFFFFF9
XPSR = C1000003: APSR = NZcvq, EPSR = 01000000, IPSR = 003 (HardFault)
CFBP = 00000000, CONTROL = 00, FAULTMASK = 00, BASEPRI = 00, PRIMASK = 00
FPU regs: FPU not enabled / not implemented on connected CPU.
J-Link>
```

then you can load/flashing into ic with `LoadFile`

## 💾 hardware 
hardware is designed with kicad, you can find out hardware design in `pcb-design` folder. 
- schematic: [sudi schematic](pcb-design/output/sudi-redox-keyboard-Schematic.pdf)

  

casing is designed with PTC CREO, and use cutting acrylic (not uploaded/complete yet but planned)  
<details>
  <summary>Expand Me </summary>
  <p align="center">
  <a href="https://youtu.be/LN7CI2rUKP8">
    <img src="http://i3.ytimg.com/vi/LN7CI2rUKP8/hqdefault.jpg" width="50%">
  </a>
</p>

<img style="align: center; width: 50vw;" src="./casing-design/Export/keyboarddrawing_img_1.png">
</details>

## 💳 Reference 
- [NRF51 Series Documentation](https://www.nordicsemi.com/Products/nRF51822/GetStarted)
- [Redox-Keyboard](https://github.com/mattdibi/redox-keyboard) i use redox schematic for initial design  
- [MCP23017 datasheet](reference/MCP23017-20001952c.pdf) and add my *initial redox sch* with MCP23017

<sup>🔥 make it V2 - 19 juni 2024 10:49</sup>
