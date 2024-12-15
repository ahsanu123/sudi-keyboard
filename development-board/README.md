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

## Note 

- [NRF51822 bluetooth module pin](https://quartzcomponents.com/cdn/shop/products/NRF51822-BLE-Module_1200x1200.jpg?v=1653290384)
  ![](https://quartzcomponents.com/cdn/shop/products/NRF51822-BLE-Module_1200x1200.jpg?v=1653290384)
```schell
left (from top perspectife, count from bottom)      Right (from top perspectife, count from bottom)  
PO3                                                 PO105                                            
PO4                                                 PO106                                            
PO1                                                 PO107                                            
PO2                                                 PO108                                            
PO30                                                PO109                                            
PO0                                                 PO110                                            
VDD                                                 PO111                                            
GND                                                 PO112                                            
PO28                                                PO003                                            
PO29                                                PO114                                            
NC                                                  PO015                                            
NC                                                  PO116                                            
PO24                                                SDA                                              
PO25                                                SCLK                                             
PO22                                                PO117                                            
PO23                                                PO119                                            
GND                                                 PO118                                            
PO21                                                PO200                                            

```

## Reference 

- [ebastler/zmk-designguide](https://github.com/ebastler/zmk-designguide)
