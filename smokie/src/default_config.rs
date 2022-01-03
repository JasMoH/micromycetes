pub mod default_config {
    //slightly odd that the explicit type specifier can't be elided, especially since the size is required
    //compiler helpfully tells you how big it is tho
    pub const DEFAULT_FRONTEND_CONFIG: [(u8, u16); 32] = [
        (0x01u8, 0x019Fu16), // Enable Interrupt for A and B time slots, neccesary for waitOnSample();
        (0x11u8, 0x30A9u16), // Writes a 32-bit sum to the FIFO for Time Slot A and Time Slot B
        (0x12u8, 0x0200u16), // 16 Hz sampling rate
        (0x14u8, 0x011Du16), // Blue Slot A, IR Slot B, combine PDs
        (0x15u8, 0x0000u16), // No decimation
        (0x17u8, 0x0009u16), // Time Slot A chop mode, inverted, noninverted, noninverted, inverted (see the Improving SNR Using Integrator Chopping section for more information)
        (0x18u8, 0x0000u16), // No ADC offset
        (0x19u8, 0x3FFFu16), // Unused channel
        (0x1Au8, 0x3FFFu16), // Unused channel
        (0x1Bu8, 0x3FFFu16), // Unused channel
        (0x1Du8, 0x0009u16), // Time Slot B chop mode (inverted, noninverted, noninverted, inverted)
        (0x1Eu8, 0x0000u16), // No ADC offset
        (0x1Fu8, 0x3FFFu16), // Unused channel
        (0x20u8, 0x3FFFu16), // Unused channel
        (0x21u8, 0x3FFFu16), // Unused channel
        (0x22u8, 0x3539u16), // LED3 IR
        (0x23u8, 0x3536u16), // LED1 blue
        (0x24u8, 0x353fu16), // LED2 (when used)
        (0x25u8, 0x630Cu16), // Default LED drive trim for all 3
        (0x30u8, 0x0320u16), // 5 μs LED pulse
        (0x31u8, 0x040Eu16), // Four pulses, 15 μs LED offset
        (0x35u8, 0x0320u16), // 3 μs LED pulse
        (0x36u8, 0x040Eu16), // Four pulses, 15 μs LED offset
        (0x39u8, 0x22F0u16), // Integrator timing
        (0x3Bu8, 0x22F0u16), // Integrator timing
        (0x3Cu8, 0x31C6u16), // Power down Channel 2, Channel 3, and Channel 4
        (0x42u8, 0x1C34u16), // 200k TIA gain
        (0x43u8, 0xADA5u16), // Signal path configuration
        (0x44u8, 0x1C34u16), // 200k TIA gain
        (0x45u8, 0xADA5u16), // Signal path configuration
        (0x58u8, 0x0544u16), // Math for chop mode inverted, noninverted, noninverted, inverted LED
        (0x54u8, 0x0AA0u16), // PD reverse bias, approximately 250 mV
    ];
}
