### Basic blinky example for STM32F103C8T6 board

### Run

Run the project via:

```bash
cargo flash --release --chip STM32F103C8
```

> I'm using ST-Link V2 to flash the board via [cargo-flash](https://crates.io/crates/cargo-flash).
> The example alternates the pin PB12 and PC13 (Built In Led) to blink the LED.

<details>
  <summary>Pinout Diagram</summary>

  ![STM32F103C8T6-pinout](https://github.com/milewski/sensors-rs/assets/2874967/5f8e234f-2ba5-4521-be23-5e59b7b1993f)
</details>

<details>
  <summary>Demo Video</summary>

  https://github.com/milewski/sensors-rs/assets/2874967/d183bf96-bd44-4637-8eae-6012b72fdfa3
</details>

The info from `.cargo/config.toml` was found via:

- The [datasheet](https://www.st.com/resource/en/datasheet/stm32f103c8.pdf) specify that the CPU in the chip is
  a `Arm 32-bit Cortex-M3`
- [This quick start guide](https://docs.rust-embedded.org/cortex-m-quickstart/cortex_m_quickstart/index.html#usage)
  for `cortex_m` crate specify that the target to use for Cortex-M3 is `thumbv7m-none-eabi`
- The guide also mention the `memory.x` layout file, the info there is on page 1 of the datasheet
- The [cortex-m-rt](https://docs.rs/cortex-m-rt/latest/cortex_m_rt) create docs also goes in depth of what
  the `memory.x` is and the `Tlink.x` linker script.

Code for this board has to be written without std since
the [rust docs](https://doc.rust-lang.org/nightly/rustc/platform-support.html#tier-2) states that `thumbv7m-none-eabi`
target does not support `std`

