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

<h1 align="center">🌿 <i>Sudi</i> Keyboard V2</h1>

<img style="align: center; width: 50vw;" src="./pcb-design/output/RedoxV2.png"> 

Sudi V2 is custom wireless split keyboard based on NRF51822 Nordic Semiconductor Microcontroller series, each side consist 18 keys 36 in totals, use MCP23017 I2C I/O expansion to scan one side of keyboard, and use wire to trasfer data to NRF51.
Sudi Have interactive RGB LED and OLED 0.91 inch for graphic display. Planned to create 3D print casing.

## 🔋 how to compile 
to build firmware you can follow qmk documentation on: https://docs.qmk.fm/#/newbs_building_firmware, create new keyboard then copy replace `sudi/firmware` to newly created qmk keyboard folder.
source code is copied and modified from mark story article: http://mark-story.com/posts/view/building-a-split-keyboard-part-3.  

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
