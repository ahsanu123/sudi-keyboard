
#include "SEGGER_RTT.h"
#include "boards.h"
#include "nrf.h"
#include "nrf_delay.h"
#include "nrf_drv_gpiote.h"
#include "nrf_drv_ppi.h"
#include "nrf_drv_timer.h"
#include "nrf_error.h"
#include "nrf_gpio.h"
#include "nrf_gpiote.h"
#include "nrf_log.h"
#include "nrf_log_backend.h"
#include "nrf_log_ctrl.h"
#include <stdbool.h>
#include <stdint.h>

int main(void) {

  NRF_LOG_INIT(NULL);
  nrf_log_backend_init(false);
  bsp_board_leds_init();

  /* Toggle LEDs. */

  while (true) {
    NRF_LOG_INFO("%s", nrf_log_push("sending data\r\n"));
    nrf_delay_ms(500);
    NRF_LOG_PROCESS();
  }
}

/** @} */
