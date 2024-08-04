# This file intentionally left blank


OLED_ENABLE = yes
OLED_DRIVER = ssd1306

CUSTOM_MATRIX = lite

SRC += matrix.c
QUANTUM_LIB_SRC += i2c_master.c

