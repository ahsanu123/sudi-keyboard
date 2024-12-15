## Introduction 

Place to save Zephyr learning with NRF51822 Module

## Setup 

- after install dependencies like cmake, devicetree, and python
- create new python Environment with `venv` on home directory for example (`~/zephyr-sdk/`), 
- run `python -m venv ~/zephyr-sdk/.venv/`
- then activate it with `source ~/zephyr-sdk/.venv/bin/activate`
- next install **west** with `pip`, run `pip install west`


## Note 

- change default Zephyr SDK installation folder with env variable [ZEPHYR_SDK_INSTALL_DIR](https://docs.zephyrproject.org/latest/develop/env_vars.html#envvar-ZEPHYR_SDK_INSTALL_DIR) 
- important Zephyr concept

> [!NOTE]
> In this document, we’ll assume:
> your application directory, `<app>`, is something like `<home>/zephyrproject/app`
> its build directory is `<app>/build`
> These terms are defined below. On Linux/macOS, `<home>` is equivalent to `~`. On Windows, it’s `%userprofile%`.
> Keeping your application inside the workspace (`<home>/zephyrproject`) makes it easier to use west build and other commands with it. (You can put your application anywhere as long as [ZEPHYR_BASE](https://docs.zephyrproject.org/latest/develop/application/index.html#important-build-vars) is set appropriately, though.)


| Application type | <app> location                           |
|------------------|------------------------------------------|
| repository       | zephyr repository                        |
| workspace        | west workspace where Zephyr is installed |
| freestanding     | other locations                          |


## Log

- Make personal note on how to setup Zephyr Environment with poetry

## Reference 

- [official bluetooth learning resource](https://www.bluetooth.com/bluetooth-resources/?types=study-guide)


