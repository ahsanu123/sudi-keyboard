# 🗞️ Introduction

Firmware version 2 with Zephyr RTOS, Will add ZMK soon

# 🧃 Basic of Zephyr 

explanation for myself to understand zephyr project structure. 
zephyr use `workspace` as top folder, first [read getting started here](https://docs.zephyrproject.org/latest/develop/getting_started/index.html),
- next you need setup a python env (for this env called `sudiv2` setuped with `micromamba`)
- next you need clone zephyr inside workspace, then init workspace with `west init`, `west` will install all dependency based on `<project-folder>/zephyr/module.yml`, installation folder will be `<workspace-root-folder>/modules`
- next you need _toolchain_, based on docs recomended folder for toolchain is in `$HOME`, you can only install several toolchain (ex: only for arm or xtensa)
- next try build your project with `cd path/to/your/project` then `west build -b <your-board> path/to/app/folder`, ex 
```shell

west build -b custom_plank app

```
```shell
$ tree -L 1
.
├── env.yml
├── modules
├── README.md
├── sudi-v2
└── zephyr
```

# 🧱 SDK Setup 

project was created use `west` zephyr RTOS _meta_ tools, 
you need activate environment with `micromamba activate sudiv2`
then you can use `west`

# ☁️ Environment Setup 

project was made with `micromamba`, you need to install micromamba, 
then install requirement inside `env.yml` with 
```shell
micromamba create -f env.yml
```


<sup>💕 08/24/2024 - made with love by ah </sup>
