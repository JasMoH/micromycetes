
use stm32f1xx_hal::i2c::*;

//type traits?
//embedded hal?

pub fn read_id<I2C,PINS>(_bus: BlockingI2c<I2C, PINS>) -> u8{
    let buf: [u8;1] = [0];
//    bus.read(0x08,&buf);
    return buf[0];
}