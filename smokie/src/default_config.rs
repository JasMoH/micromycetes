pub mod default_config {
    pub const DEFAULT_FRONTEND_CONFIG: [(u8, u16); 32] = [
        (0x01, 0x019F), // Enable Interrupt for A and B time slots, neccesary for waitOnSample();
        (0x11, 0x30A9), // Writes a 32-bit sum to the FIFO for Time Slot A and Time Slot B
        (0x12, 0x0200), // 16 Hz sampling rate
        (0x14, 0x011D), // Blue Slot A, IR Slot B, combine PDs
        (0x15, 0x0000), // No decimation
        (0x17, 0x0009), // Time Slot A chop mode, inverted, noninverted, noninverted, inverted (see the Improving SNR Using Integrator Chopping section for more information)
        (0x18, 0x0000), // No ADC offset
        (0x19, 0x3FFF), // Unused channel
        (0x1A, 0x3FFF), // Unused channel
        (0x1B, 0x3FFF), // Unused channel
        (0x1D, 0x0009), // Time Slot B chop mode (inverted, noninverted, noninverted, inverted)
        (0x1E, 0x0000), // No ADC offset
        (0x1F, 0x3FFF), // Unused channel
        (0x20, 0x3FFF), // Unused channel
        (0x21, 0x3FFF), // Unused channel
        (0x22, 0x3539), // LED3 IR
        (0x23, 0x3536), // LED1 blue
        (0x24, 0x353f), // LED2 (when used)
        (0x25, 0x630C), // Default LED drive trim for all 3
        (0x30, 0x0320), // 5 μs LED pulse
        (0x31, 0x040E), // Four pulses, 15 μs LED offset
        (0x35, 0x0320), // 3 μs LED pulse
        (0x36, 0x040E), // Four pulses, 15 μs LED offset
        (0x39, 0x22F0), // Integrator timing
        (0x3B, 0x22F0), // Integrator timing
        (0x3C, 0x31C6), // Power down Channel 2, Channel 3, and Channel 4
        (0x42, 0x1C34), // 200k TIA gain
        (0x43, 0xADA5), // Signal path configuration
        (0x44, 0x1C34), // 200k TIA gain
        (0x45, 0xADA5), // Signal path configuration
        (0x58, 0x0544), // Math for chop mode inverted, noninverted, noninverted, inverted LED
        (0x54, 0x0AA0), // PD reverse bias, approximately 250 mV
    ];
}
