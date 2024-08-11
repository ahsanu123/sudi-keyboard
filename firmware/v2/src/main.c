#include "boards.h"
#include "nrf_delay.h"
#include "nrf_gpio.h"
#include <stdbool.h>
#include <stdint.h>

/**
 * @brief Function for application main entry.
 */
int main(void) {
  /* Configure board. */
  // bsp_board_leds_init();
  nrf_gpio_cfg_output(5);

  /* Toggle LEDs. */
  while (true) {
    for (int i = 0; i < LEDS_NUMBER; i++) {
      nrf_gpio_pin_toggle(5);
      // bsp_board_led_invert(i);
      nrf_delay_ms(800);
    }
  }
}

/**
 *@}
 **/
