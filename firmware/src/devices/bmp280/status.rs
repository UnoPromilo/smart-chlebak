use defmt::Format;

#[allow(unused)]
pub struct Status {
    pub measuring: bool,
    pub im_update: bool,
}

#[derive(Debug, Clone, Format)]
pub struct CalibrationData {
    pub dig_t1: u16,
    pub dig_t2: i16,
    pub dig_t3: i16,
    pub dig_p1: u16,
    pub dig_p2: i16,
    pub dig_p3: i16,
    pub dig_p4: i16,
    pub dig_p5: i16,
    pub dig_p6: i16,
    pub dig_p7: i16,
    pub dig_p8: i16,
    pub dig_p9: i16,
}

impl CalibrationData {
    pub fn from(data: &[u8; 24]) -> Self {
        fn u16_le(bytes: &[u8]) -> u16 {
            u16::from_le_bytes([bytes[0], bytes[1]])
        }

        fn i16_le(bytes: &[u8]) -> i16 {
            i16::from_le_bytes([bytes[0], bytes[1]])
        }

        Self {
            dig_t1: u16_le(&data[0..2]),
            dig_t2: i16_le(&data[2..4]),
            dig_t3: i16_le(&data[4..6]),
            dig_p1: u16_le(&data[6..8]),
            dig_p2: i16_le(&data[8..10]),
            dig_p3: i16_le(&data[10..12]),
            dig_p4: i16_le(&data[12..14]),
            dig_p5: i16_le(&data[14..16]),
            dig_p6: i16_le(&data[16..18]),
            dig_p7: i16_le(&data[18..20]),
            dig_p8: i16_le(&data[20..22]),
            dig_p9: i16_le(&data[22..24]),
        }
    }

    pub fn t_fine_from_adc(&self, adc_t: i32) -> i32 {
        let var1 = (((adc_t >> 3) - ((self.dig_t1 as i32) << 1)) * (self.dig_t2 as i32)) >> 11;
        let var2 = (((((adc_t >> 4) - (self.dig_t1 as i32))
            * ((adc_t >> 4) - (self.dig_t1 as i32)))
            >> 12)
            * (self.dig_t3 as i32))
            >> 14;
        var1 + var2
    }

    pub fn compensate_temperature(&self, adc_t: i32) -> i32 {
        let t_fine = self.t_fine_from_adc(adc_t);
        (t_fine * 5 + 128) >> 8 // °C * 100
    }

    pub fn compensate_pressure(&self, adc_p: i32, adc_t: i32) -> u32 {
        let t_fine = self.t_fine_from_adc(adc_t);

        let mut var1 = t_fine as i64 - 128_000;
        let mut var2 = var1 * var1 * (self.dig_p6 as i64);
        var2 += (var1 * (self.dig_p5 as i64)) << 17;
        var2 += (self.dig_p4 as i64) << 35;

        var1 = ((var1 * var1 * (self.dig_p3 as i64)) >> 8) + ((var1 * (self.dig_p2 as i64)) << 12);
        var1 = (((1i64) << 47) + var1) * (self.dig_p1 as i64) >> 33;

        if var1 == 0 {
            return 0; // avoid division by zero
        }

        let mut p = 1048576 - adc_p as i64;
        p = (((p << 31) - var2) * 3125) / var1;

        var1 = ((self.dig_p9 as i64) * (p >> 13) * (p >> 13)) >> 25;
        var2 = ((self.dig_p8 as i64) * p) >> 19;

        p = ((p + var1 + var2) >> 8) + ((self.dig_p7 as i64) << 4);
        p as u32
    }
}
