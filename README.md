# 1️⃣ Sudi Keyboard V1

<img style="align: center; width: 50vw;" src="./pcb-design/output/banner.png">
<!-- <img style="align: right; width: 50vw;" src="./pcb-design/output/sudi-redox-clone-RIGHT.png"> -->
Sudi keyboard is custom split keyboard based on QMK, its use MCP23017 for scanning key matrix on right side, use promicro for scanning left side. right now sudi is not come with rgb led but its planned.
each side consist 42 standard mechanical key, 84 in total. 

## how to compile 
to build firmware you can follow qmk documentation on: https://docs.qmk.fm/#/newbs_building_firmware, create new keyboard then copy replace `sudi/firmware` to newly created qmk keyboard folder.
source code is copied and modified from mark story article: http://mark-story.com/posts/view/building-a-split-keyboard-part-3.  

## hardware 
hardware is designed with kicad, you can find out hardware design in `pcb-design` folder. 
- schematic: [sudi schematic](pcb-design/output/sudi-redox-keyboard-Schematic.pdf)

casing is designed with PTC CREO, and use cutting acrylic (not uploaded/complete yet but planned)  

<p align="center">
  <a href="https://youtu.be/LN7CI2rUKP8">
    <img src="http://i3.ytimg.com/vi/LN7CI2rUKP8/hqdefault.jpg" width="50%">
  </a>
</p>

<img style="align: center; width: 50vw;" src="./casing-design/Export/keyboarddrawing_img_1.png">

## Reference 
- [Redox-Keyboard](https://github.com/mattdibi/redox-keyboard) i use redox schematic for initial design  
- [MCP23017 datasheet](reference/MCP23017-20001952c.pdf) and add my *initial redox sch* with MCP23017

<sup>🔥 make it V2 - 19 juni 2024 10:49</sup>
