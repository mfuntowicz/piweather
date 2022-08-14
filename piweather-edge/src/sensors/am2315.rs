const AM2315_DEFAULT_I2C_ADDRESS: u16 = 0x5c;

pub struct Am2315 {}

impl Am2315 {
    pub fn new(address: u16) -> Result<Self, ()> {
        Ok(Am2315 {})
    }
}

impl Default for Am2315 {
    fn default() -> Result<Self, ()> {
        Am2315::new(AM2315_DEFAULT_I2C_ADDRESS)
    }
}
