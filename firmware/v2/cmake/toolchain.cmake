set(CMAKE_SYSTEM_NAME                 Generic)
set(CMAKE_SYSTEM_PROCESSOR            arm)

set(TOOLCHAIN_PREFIX arm-none-eabi)


# CC		:= $(PREFIX)-gcc
# CXX		:= $(PREFIX)-g++
# LD		:= $(PREFIX)-gcc
# AR		:= $(PREFIX)-ar
# AS		:= $(PREFIX)-as
# OBJCOPY		:= $(PREFIX)-objcopy
# OBJDUMP		:= $(PREFIX)-objdump
# GDB		:= $(PREFIX)-gdb
# SIZE		:= $(PREFIX)-size

set(CMAKE_C_COMPILER                  ${TOOLCHAIN_PREFIX}-gcc)
set(CMAKE_CXX_COMPILER                ${TOOLCHAIN_PREFIX}-g++)
set(CMAKE_ASM_COMPILER                ${TOOLCHAIN_PREFIX}-as)
set(CMAKE_AR_COMPILER                  ${TOOLCHAIN_PREFIX}-ar)
set(CMAKE_OBJCOPY                     ${TOOLCHAIN_PREFIX}-objcopy)
set(CMAKE_SIZE                        ${TOOLCHAIN_PREFIX}-size)

set(CMAKE_TRY_COMPILE_TARGET_TYPE     STATIC_LIBRARY)

set(CMAKE_EXECUTABLE_SUFFIX_C         .out)
set(CMAKE_EXECUTABLE_SUFFIX_CXX       .out)
set(CMAKE_EXECUTABLE_SUFFIX_ASM       .out)

