# about
this is just a storage space for some quick experements in embedded rust. This may evolve further
in to some more detailed projects in time, but is most likely abandonware.

some things I'm interested in exploring with this:
* familiarity with the hal crates
* developing for multiple architectures
* 
# helpful projects 'n resources
[bluepill quickstart](https://github.com/TeXitoi/blue-pill-quickstart)
[the embeddonomicon](https://docs.rust-embedded.org/book/intro/index.html)
[usage axamples for bluepill hal](https://github.com/stm32-rs/stm32f1xx-hal/tree/v0.7.0/examples)

## notes on setting up i2c read
[embedded hal i2c documentation](https://docs.rs/embedded-hal/0.2.6/embedded_hal/blocking/i2c/index.html)
haven't fonud a ton of references for i2c reading. I based this off a the 
[spi dma example](https://github.com/stm32-rs/stm32f1xx-hal/blob/v0.7.0/examples/spi-dma.rs)
and the [i2c documentation](https://docs.rs/stm32f1xx-hal/latest/stm32f1xx_hal/i2c/index.html).
This is where typestate programing really shines, 'cus once your in the ballpark 
you can just chase compiler errors.
