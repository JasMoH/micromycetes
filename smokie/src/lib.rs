#![cfg_attr(not(test), no_std)]

//quick wraapper for a random part I had lying around that talked i2c
//in this case, the EVAL-ADPD188BIZ-S2 evaluation board

//forgot that stmicro has the shifted down convention for addresses
//unclear if this is properly capturedby the HAL? haven't tried other parts yet
const ADDRESS: u8 = 0xC8 >> 1;

mod default_config;

pub mod access {
    use crate::default_config::default_config::DEFAULT_FRONTEND_CONFIG;
    use embedded_hal::blocking::i2c::{Write, WriteRead};

    pub fn reset_and_init<T>(bus: &mut T)
    where
        T: Write,
    {
        write16(bus, (0x0f, 1)); // soft reset;
                                 //clock enable
        write16(bus, (0x4B, 0x01 << 7)); //seventh bit set to 1
    }

    fn device_id<T>(bus: &mut T) -> Result<u16, ()>
    where
        T: WriteRead,
    {
        read16(bus, 0x08)
    }

    pub fn check_id<T>(bus: &mut T) -> bool
    where
        T: WriteRead,
    {
        let expected: u16 = 0x0a16;
        match device_id(bus) {
            Ok(result) => result == expected,
            Err(_) => false,
        }
    }

    pub fn write16<T>(bus: &mut T, reg: (u8, u16))
    where
        T: Write,
    {
        //repack to bytes
        let txbuff: [u8; 3] = [reg.0, (reg.1 >> 8) as u8, (reg.1 & 0xFF) as u8];

        let _ = bus.write(crate::ADDRESS, &txbuff);
    }

    pub fn read16<T>(bus: &mut T, reg: u8) -> Result<u16, ()>
    where
        T: WriteRead,
    {
        let mut buf: [u8; 2] = [0, 0];
        let reg: [u8; 1] = [reg];

        match bus.write_read(crate::ADDRESS, &reg, &mut buf) {
            Ok(()) => Ok((buf[0] as u16) << 8 | (buf[1] as u16)),
            Err(_) => Err(()),
        }
    }

    pub fn configure_defaults<T>(bus: &mut T)
    where
        T: Write,
    {
        write16(bus, (0x10, 0x01)); //program mode
        for reg in DEFAULT_FRONTEND_CONFIG {
            let _ = write16(bus, reg);
        }
        write16(bus, (0x10, 0x02)); //normal mode
    }

    #[cfg(test)]
    mod tests {

        use super::*;
        use embedded_hal::blocking::i2c::*;
        use mockall::mock;

        mock! {
        I2C {}
        impl WriteRead  for I2C {
            type Error = core::fmt::Error;
                fn write_read(
                    &mut self,
                    address: u8,
                    bytes: &[u8],
                    buffer: &mut [u8],
                ) -> Result<(), <tests::MockI2C as WriteRead>::Error>;
                    }
        }

        #[test]
        fn test_id() {
            let mut mock = MockI2C::new();
            mock.expect_write_read()
                //validate inputes
                .withf(|addr: &u8, bytes: &[u8], buffer: & [u8]| {
                    (addr == &crate::ADDRESS) && (bytes == [0x08u8]) && (buffer.len() == 2)
                })
                //spcify return value, including return by filling buffer
                .returning(|addr: u8, bytes: &[u8], buffer: &mut [u8]|{
                    buffer[0] = 0x0au8;
                    buffer[1] = 0x16u8;
                    Ok(())
                } );
//                .return_const(Ok(()));

            assert_eq!(device_id(&mut mock).unwrap(),0x0a16u16);
        }
    }
}
