# ⌨️ Sudi With RMK 

Try to understand and implement RMK for Sudi, make contribution if possible.

## 🦀 RMK Note 

**RMK File Summary**

- 📁 ble
- 📁 config
- 📁 input_device
- 📁 split
- 📁 storage
- 📁 usb
- 📁 via
- 🍉 action.rs -> contain struct and function to convert action into real binary keyboard command
- 🍉 boot.rs -> contain basic boot operation, jump to bootloader and reboot keyboard
- 🍉 channel.rs -> contain multithread channel, signal, mutex and pubsub. mostly from embassy_sync lib
- 🍉 combo.rs -> contain struct to implement keyboard combo feature
- 🍉 direct_pin.rs -> contain struct that define structure of keyboard matrix,
> [!NOTE]
>
> directPinMatrix contain 2 method, `read_event` and `wait_for_key`.
>
> if `async_matrix` is use, `read_event` will wait for key (wait for keyboard user to typing on key), based on setting, switch matrix may active low or active high, inside  `wait_for_key` program will use `select_slice` from `embassy_future::select` to wait any key to be low/high then it will continue process in `read_event` method.
>
> if `async_matrix` is not used program will run loop scanning inside `read_event` method every `100 micro second` as indicated by `Timer::after_micros(100).await;` it will use debouncer to detect if debounce happen, then it will return `Event`.
> 
> (need read more about this method, and its logic)

- 🍉 event.rs -> contain struct of Event, event is not only coming from keyboard typing, but also several other type.
- 🍉 fmt.rs -> contain format for doing debug, loging, or print information
- 🍉 fork.rs -> not trully understand what is this 
- 🍉 hid.rs -> contain HID struct and trait to be implemented by concrete peripheral like USB or BLE 
- 🍉 hid_state.rs -> contain HID state like, HID modifier, HID Mouse button (need read more about HID)
- 🍉 keyboard.rs -> i think its center logic, its big file and lot of test too, need to deep dive into it.
- 🍉 keyboard_macro.rs -> define sturcture, logic and test of _keyboard macro_ (key combination / sequence operation with one key press)
- 🍉 keycode.rs -> contain all HID keycode
- 🍉 keymap.rs -> define logic of keymap (key layer, what key will do when it pressed)
- 🍉 layout_macro.rs -> contain lot of macro, still dont understand well on reading `macro_rules`
- 🍉 lib.rs -> contain library utility function, and exported API to enduser
- 🍉 light.rs -> contain led controller, need read more about logic on this file 
- 🍉 matrix.rs -> i think its high level for abstraction of `direct_pin.rs`
- 🍉 state.rs -> contain keyboard connection state (BLE, or USB), its like, its keyboard connected??

## 🍈 References 

- [cfg attribute](https://doc.rust-lang.org/rust-by-example/attribute/cfg.html)
