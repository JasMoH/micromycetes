# about
this is just a storage space for some quick experements in embedded rust. This may evolve further
in to some more detailed projects in time, but is most likely abandonware.

some things I'm interested in exploring with this:
* familiarity with the hal crates
* developing for multiple architectures

# observations and next steps
I can't seem to get the internal pullups to enable when pb11/10 are set to alternate function. look in to this,
maybe submit patch if that's not my fault?

STM's good ol' `const ADDRESS: u8 = 0xC8 >> 1;` convention for 8 bit addresses rears its head again...

# integration with clion
weird and crap a bit so far...

# helpful projects 'n resources
## embedded stuff
* [bluepill quickstart](https://github.com/TeXitoi/blue-pill-quickstart)
* [the embeddonomicon](https://docs.rust-embedded.org/book/intro/index.html)
* [usage axamples for bluepill hal](https://github.com/stm32-rs/stm32f1xx-hal/tree/v0.7.0/examples)
* [embedded rust on portability](https://docs.rust-embedded.org/book/portability/)

### setting target info
there's a handful of config stuff that a lot of embedded micros need that isn't typically present in a std
crate. this often includes linker/locater scripting / memory layout, debugger configs, crosscompile target info, and others.
 I've found [this guide from ferrous systems](https://ferrous-systems.com/blog/test-embedded-app/#structuring-the-project-for-host--and-cross-compilation)
is useful for showing how to structure a multi-target project to keep that all clean and working.

## rust general
* [mdbook for documentation](https://rust-lang.github.io/mdBook/)
* [rust desgin patterns](https://rust-unofficial.github.io/patterns/)

## esp32 stuff
for xtensa esp32s, you'lle need [the rust xtensa compiler fork](https://github.com/esp-rs/rust-build)

the [esp32-hal](https://github.com/esp-rs/esp32-hal) crate is a good starting point for getting
peripherals set up, project configuration, etc. it is intende for no-std applications. there is also 
[a hal based on the esp-idf](https://github.com/esp-rs/esp-idf-hal) which provides std support.

### compiling for esp32
I'm using the espressif docker container. the compile command is:
```sh
./run_env.sh #mounts volumes and such
cargo +esp build
```

## notes on setting up i2c read
[embedded hal i2c documentation](https://docs.rs/embedded-hal/0.2.6/embedded_hal/blocking/i2c/index.html)

the `.cargo/config` file is critical for getting some of the options neccesary for compling for the embedded target


haven't fonud a ton of references for i2c reading. I based this off a the 
[spi dma example](https://github.com/stm32-rs/stm32f1xx-hal/blob/v0.7.0/examples/spi-dma.rs)
and the [i2c documentation](https://docs.rs/stm32f1xx-hal/latest/stm32f1xx_hal/i2c/index.html).
This is where typestate programing really shines, 'cus once your in the ballpark 
you can just chase compiler errors.

# udev rules
I've already done this on my dev machine at least once, but worth having the notes here.
from the [nucleo quickstart](https://github.com/reneherrero/nucleo-l432kc-quickstart)
```shell script
# download the OpenOCD rules file and copy it to the right location
wget -O 60-openocd.rules https://sf.net/p/openocd/code/ci/master/tree/contrib/60-openocd.rules?format=raw
sudo cp 60-openocd.rules /etc/udev/rules.d

# ask the udev daemon to reload these rules
sudo udevadm control --reload
```