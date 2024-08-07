cmake_minimum_required(VERSION 3.29)
project(Sudi VERSION 1)

set(SDK_ROOT ${CMAKE_CURRENT_SOURCE_DIR}/sdk/nRF5_SDK_12)
set(PROJ_DIR ${CMAKE_CURRENT_SOURCE_DIR})
set(TARGET_DEVICE nrf51822_xxac)

include(./cmake/toolchain.cmake)
message("-- Current Working Directory: ${CMAKE_CURRENT_SOURCE_DIR}")

if(NOT SDK_ROOT)
  message(FATAL_ERROR "Please Set SDK_ROOT")
else()
  message("-- SDK_ROOT: ${SDK_ROOT}")
endif()

if(NOT TARGET_DEVICE)
  message(FATAL_ERROR "Please Set TARGET_DEVICE")
else()
  message("-- TARGET_DEVICE: ${TARGET_DEVICE}")
endif()

if(EXISTS ${SDK_ROOT}/components/toolchain/gcc/gcc_nrf51_common.ld)
  set(LINKER_FILE ${SDK_ROOT}/components/toolchain/gcc/gcc_nrf51_common.ld)
  message("-- Found Common Linker File.")
else()
  message(FATAL_ERROR "Cant Find Linker File")
endif()

set(
  SRCS

  ${SDK_ROOT}/components/libraries/log/src/nrf_log_backend_serial.c
  ${SDK_ROOT}/components/libraries/log/src/nrf_log_frontend.c
  ${SDK_ROOT}/components/libraries/button/app_button.c
  ${SDK_ROOT}/components/libraries/util/app_error.c
  ${SDK_ROOT}/components/libraries/util/app_error_weak.c
  ${SDK_ROOT}/components/libraries/scheduler/app_scheduler.c
  ${SDK_ROOT}/components/libraries/timer/app_timer_ble_gzll.c
  ${SDK_ROOT}/components/libraries/timer/app_timer_appsh.c
  ${SDK_ROOT}/components/libraries/util/app_util_platform.c
  ${SDK_ROOT}/components/libraries/crc16/crc16.c
  ${SDK_ROOT}/components/libraries/fds/fds.c
  ${SDK_ROOT}/components/libraries/fstorage/fstorage.c
  ${SDK_ROOT}/components/libraries/hardfault/hardfault_implementation.c
  ${SDK_ROOT}/components/libraries/util/nrf_assert.c
  ${SDK_ROOT}/components/libraries/util/sdk_errors.c
  ${SDK_ROOT}/components/libraries/util/sdk_mapped_flags.c
  ${SDK_ROOT}/components/libraries/sensorsim/sensorsim.c
  ${SDK_ROOT}/components/boards/boards.c
  ${SDK_ROOT}/components/drivers_nrf/clock/nrf_drv_clock.c
  ${SDK_ROOT}/components/drivers_nrf/common/nrf_drv_common.c
  ${SDK_ROOT}/components/drivers_nrf/gpiote/nrf_drv_gpiote.c
  ${SDK_ROOT}/components/drivers_nrf/uart/nrf_drv_uart.c
  ${SDK_ROOT}/components/libraries/bsp/bsp.c
  ${SDK_ROOT}/components/libraries/bsp/bsp_btn_ble.c
  ${SDK_ROOT}/components/libraries/bsp/bsp_nfc.c
  ${SDK_ROOT}/external/segger_rtt/RTT_Syscalls_GCC.c
  ${SDK_ROOT}/external/segger_rtt/SEGGER_RTT.c
  ${SDK_ROOT}/external/segger_rtt/SEGGER_RTT_printf.c
  ${SDK_ROOT}/components/ble/common/ble_advdata.c
  ${SDK_ROOT}/components/ble/ble_advertising/ble_advertising.c
  ${SDK_ROOT}/components/ble/common/ble_conn_params.c
  ${SDK_ROOT}/components/ble/common/ble_conn_state.c
  ${SDK_ROOT}/components/ble/common/ble_srv_common.c
  ${SDK_ROOT}/components/ble/peer_manager/gatt_cache_manager.c
  ${SDK_ROOT}/components/ble/peer_manager/gatts_cache_manager.c
  ${SDK_ROOT}/components/ble/peer_manager/id_manager.c
  ${SDK_ROOT}/components/ble/peer_manager/peer_data.c
  ${SDK_ROOT}/components/ble/peer_manager/peer_data_storage.c
  ${SDK_ROOT}/components/ble/peer_manager/peer_database.c
  ${SDK_ROOT}/components/ble/peer_manager/peer_id.c
  ${SDK_ROOT}/components/ble/peer_manager/peer_manager.c
  ${SDK_ROOT}/components/ble/peer_manager/pm_buffer.c
  ${SDK_ROOT}/components/ble/peer_manager/pm_mutex.c
  ${SDK_ROOT}/components/ble/peer_manager/security_dispatcher.c
  ${SDK_ROOT}/components/ble/peer_manager/security_manager.c
  ${SDK_ROOT}/components/toolchain/gcc/gcc_startup_nrf51.S
  ${SDK_ROOT}/components/toolchain/system_nrf51.c
  ${SDK_ROOT}/components/ble/ble_services/ble_bas/ble_bas.c
  ${SDK_ROOT}/components/ble/ble_services/ble_dis/ble_dis.c
  ${SDK_ROOT}/components/ble/ble_services/ble_hids/ble_hids.c
  ${SDK_ROOT}/components/softdevice/common/softdevice_handler/softdevice_handler.c
  ${SDK_ROOT}/components/softdevice/common/softdevice_handler/softdevice_handler_appsh.c
  ${SDK_ROOT}/components/libraries/uart/app_uart_fifo.c
  ${SDK_ROOT}/components/libraries/fifo/app_fifo.c
  ${SDK_ROOT}/components/libraries/uart/retarget.c

)

add_executable(
  sudiMain
  ./src/main.c
  ${SRCS}
)

target_link_libraries(
  sudiMain
  PUBLIC

  ${SDK_ROOT}/components/proprietary_rf/gzll/gcc/gzll_nrf51_gcc.a
  ${SDK_ROOT}/components/proprietary_rf/gzll/gcc/gzll_nrf51_sd_resources_gcc.a
)

target_link_options(
  sudiMain
  PUBLIC
  -T ${LINKER_FILE}
)

target_compile_options(
  sudiMain
  PUBLIC
  -DBOARD_CUSTOM
  -DSOFTDEVICE_PRESENT
  -DNRF51
  -DS130
  -DBLE_STACK_SUPPORT_REQD
  -DSWI_DISABLE0
  -DNRF51822
  -DNRF_SD_BLE_API_VERSION=2
  -mcpu=cortex-m0
  -mthumb -mabi=aapcs
  -Wall -Werror -O3 -g3
  -mfloat-abi=soft
  -ffunction-sections -fdata-sections -fno-strict-aliasing
  -fno-builtin --short-enums

  -DBOARD_CUSTOM
  -DSOFTDEVICE_PRESENT
  -DNRF51
  -DS130
  -DBLE_STACK_SUPPORT_REQD
  -DSWI_DISABLE0
  -DNRF51822
  -DNRF_SD_BLE_API_VERSION=2
  -D__HEAP_SIZE=1024 -D__STACK_SIZE=1024

  -mcpu=cortex-m0
  -Wl,--gc-sections
  --specs=nano.specs -lc -lnosys
  -Wl,--no-wchar-size-warning

  -Wno-array-bounds
)

target_include_directories(
  sudiMain
  PUBLIC

  ${PROJ_DIR}/src/config
  ${SDK_ROOT}/components/drivers_nrf/comp
  ${SDK_ROOT}/components/drivers_nrf/twi_master
  ${SDK_ROOT}/components/ble/ble_services/ble_ancs_c
  ${SDK_ROOT}/components/ble/ble_services/ble_ias_c
  ${SDK_ROOT}/components/softdevice/s130/headers
  ${SDK_ROOT}/components/libraries/pwm
  ${SDK_ROOT}/components/libraries/usbd/class/cdc/acm
  ${SDK_ROOT}/components/libraries/usbd/class/hid/generic
  ${SDK_ROOT}/components/libraries/usbd/class/msc
  ${SDK_ROOT}/components/libraries/usbd/class/hid
  ${SDK_ROOT}/components/libraries/log
  ${SDK_ROOT}/components/ble/ble_services/ble_gls
  ${SDK_ROOT}/components/libraries/fstorage
  ${SDK_ROOT}/components/drivers_nrf/i2s
  ${SDK_ROOT}/components/libraries/gpiote
  ${SDK_ROOT}/components/drivers_nrf/gpiote
  ${SDK_ROOT}/components/boards
  ${SDK_ROOT}/components/drivers_nrf/common
  ${SDK_ROOT}/components/ble/ble_advertising
  ${SDK_ROOT}/components/drivers_nrf/adc
  ${SDK_ROOT}/components/softdevice/s130/headers/nrf51
  ${SDK_ROOT}/components/ble/ble_services/ble_bas_c
  ${SDK_ROOT}/components/ble/ble_services/ble_hrs_c
  ${SDK_ROOT}/components/libraries/queue
  ${SDK_ROOT}/components/ble/ble_dtm
  ${SDK_ROOT}/components/toolchain/cmsis/include
  ${SDK_ROOT}/components/ble/ble_services/ble_rscs_c
  ${SDK_ROOT}/components/drivers_nrf/uart
  ${SDK_ROOT}/components/ble/common
  ${SDK_ROOT}/components/ble/ble_services/ble_lls
  ${SDK_ROOT}/components/drivers_nrf/wdt
  ${SDK_ROOT}/components/libraries/bsp
  ${SDK_ROOT}/components/ble/ble_services/ble_bas
  ${SDK_ROOT}/components/libraries/experimental_section_vars
  ${SDK_ROOT}/components/ble/ble_services/ble_ans_c
  ${SDK_ROOT}/components/libraries/slip
  ${SDK_ROOT}/components/libraries/mem_manager
  ${SDK_ROOT}/external/segger_rtt
  ${SDK_ROOT}/components/libraries/csense_drv
  ${SDK_ROOT}/components/drivers_nrf/hal
  ${SDK_ROOT}/components/ble/ble_services/ble_nus_c
  ${SDK_ROOT}/components/drivers_nrf/rtc
  ${SDK_ROOT}/components/ble/ble_services/ble_ias
  ${SDK_ROOT}/components/libraries/usbd/class/hid/mouse
  ${SDK_ROOT}/components/drivers_nrf/ppi
  ${SDK_ROOT}/components/ble/ble_services/ble_dfu
  ${SDK_ROOT}/components/drivers_nrf/twis_slave
  ${SDK_ROOT}/components
  ${SDK_ROOT}/components/libraries/scheduler
  ${SDK_ROOT}/components/ble/ble_services/ble_lbs
  ${SDK_ROOT}/components/ble/ble_services/ble_hts
  ${SDK_ROOT}/components/drivers_nrf/delay
  ${SDK_ROOT}/components/libraries/crc16
  ${SDK_ROOT}/components/drivers_nrf/timer
  ${SDK_ROOT}/components/libraries/util
  ${SDK_ROOT}/components/drivers_nrf/pwm
  ${SDK_ROOT}/components/libraries/usbd/class/cdc
  ${SDK_ROOT}/components/libraries/csense
  ${SDK_ROOT}/components/drivers_nrf/rng
  ${SDK_ROOT}/components/libraries/low_power_pwm
  ${SDK_ROOT}/components/libraries/hardfault
  ${SDK_ROOT}/components/ble/ble_services/ble_cscs
  ${SDK_ROOT}/components/libraries/uart
  ${SDK_ROOT}/components/libraries/hci
  ${SDK_ROOT}/components/libraries/usbd/class/hid/kbd
  ${SDK_ROOT}/components/drivers_nrf/spi_slave
  ${SDK_ROOT}/components/drivers_nrf/lpcomp
  ${SDK_ROOT}/components/libraries/timer
  ${SDK_ROOT}/components/drivers_nrf/power
  ${SDK_ROOT}/components/libraries/usbd/config
  ${SDK_ROOT}/components/toolchain
  ${SDK_ROOT}/components/libraries/led_softblink
  ${SDK_ROOT}/components/drivers_nrf/qdec
  ${SDK_ROOT}/components/ble/ble_services/ble_cts_c
  ${SDK_ROOT}/components/drivers_nrf/spi_master
  ${SDK_ROOT}/components/ble/ble_services/ble_nus
  ${SDK_ROOT}/components/ble/ble_services/ble_hids
  ${SDK_ROOT}/components/drivers_nrf/pdm
  ${SDK_ROOT}/components/libraries/crc32
  ${SDK_ROOT}/components/libraries/usbd/class/audio
  ${SDK_ROOT}/components/libraries/sensorsim
  ${SDK_ROOT}/components/ble/peer_manager
  ${SDK_ROOT}/components/drivers_nrf/swi
  ${SDK_ROOT}/components/ble/ble_services/ble_tps
  ${SDK_ROOT}/components/ble/ble_services/ble_dis
  ${SDK_ROOT}/components/device
  ${SDK_ROOT}/components/ble/nrf_ble_qwr
  ${SDK_ROOT}/components/libraries/button
  ${SDK_ROOT}/components/libraries/usbd
  ${SDK_ROOT}/components/drivers_nrf/saadc
  ${SDK_ROOT}/components/ble/ble_services/ble_lbs_c
  ${SDK_ROOT}/components/ble/ble_racp
  ${SDK_ROOT}/components/toolchain/gcc
  ${SDK_ROOT}/components/libraries/fds
  ${SDK_ROOT}/components/libraries/twi
  ${SDK_ROOT}/components/drivers_nrf/clock
  ${SDK_ROOT}/components/ble/ble_services/ble_rscs
  ${SDK_ROOT}/components/drivers_nrf/usbd
  ${SDK_ROOT}/components/softdevice/common/softdevice_handler
  ${SDK_ROOT}/components/ble/ble_services/ble_hrs
  ${SDK_ROOT}/components/libraries/log/src
  ${SDK_ROOT}/components/proprietary_rf/gzll
  ${SDK_ROOT}/components/libraries/fifo
)

