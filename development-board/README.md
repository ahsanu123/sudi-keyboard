## SUDI Development Board

Development board intended to understanding NRF52 nordic series for wireless keyboard, 
with Battery Management System (BMS), Bluetooth Low Energy Communication, USB, Vsense and 
LED Matrix.

## Description 

initial design decision, 

- NRF52820(5v tolerant, and USB, 256 kB flash, 32 kB RAM, QFN-40-EP(5x5), $2.6)
- NRF52832(**No** 5v tolerant, and **No** USB, 512/256 kB flash, 64/32 kB RAM, QFN-48-EP(6x6), $2.75)
- NRF52833QDXX(5v tolerant, and USB, 512 kB flash, 128 kB RAM, QFN-40-EP(5x5), $3.59)
- NRF52840(5v tolerant, and USB, 1 MB flash, 256 kB RAM, WLCSP-94(3.5x3.6) More difficult to Solder but most complete feature, $5.22)
- Add BMS into board 
- add IO expander, use MCP23017 smd
- board able to drive ws2812b (min voltage 3.5)
- vsense ic MAX17048
- switch to test 
- led for indicator 
- give all pin connector 
- usb with protection 
- uart 
- i2c 


## Reference 

- [ebastler/zmk-designguide](https://github.com/ebastler/zmk-designguide)
