#![no_std]

const ADDRESS: u8 = 0xC8;

pub mod access {
    use embedded_hal::blocking::i2c::WriteRead;

    pub fn device_id<T>(bus: &mut T) -> Result<u8, ()>
    where
        T: WriteRead,
    {
        let mut buf: [u8; 1] = [0];
        let reg: [u8; 1] = [0x08]; //device id register

        match bus.write_read(crate::ADDRESS, &reg, &mut buf) {
            Ok(()) => Ok(buf[0]),
            Err(_) => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

//registers
// devid = 0x08,
