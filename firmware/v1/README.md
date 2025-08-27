# How to compile and flash 

- follow qmk instruction to setup env https://docs.qmk.fm/newbs_getting_started 
- in arch linux we can install with `sudo pacman -S qmk --noconfirm`
- then make new keyboard with `qmk new-keyboard -n sudi-keyboard -u ahsanu123 -kb sudiv1`
- delete `sudiv1` content
- copy this `v1` content into `sudiv1` folder inside `qmk_fimware/keyboards/sudiv1`
- try to compile with `qmk compile -kb sudiv1 -km default`
- and flash it with `qmk flash -kb sudiv1 -km default`

