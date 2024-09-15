#[allow(dead_code)]
pub enum Size {
    MB50,
    MB100,
    MB125,
    MB128,
    MB200,
    MB250,
    MB300,
    MB500,
    GB1,
    GB2,
    GB5,
    GB10,
    GB15,
    GB20,
    GB25,
    GB50,
    GB100,
    GB150,
    GB200,
    GB250,
    GB300,
    GB500,
    Custom(usize),
}

impl Size {
    fn value(&self) -> usize {
        match self {
            Size::MB50 => 50_000_000,
            Size::MB100 => 100_000_000,
            Size::MB125 => 125_000_000,
            Size::MB128 => 128_000_000,
            Size::MB200 => 200_000_000,
            Size::MB250 => 250_000_000,
            Size::MB300 => 300_000_000,
            Size::MB500 => 500_000_000,
            Size::GB1 => 1_000_000_000,
            Size::GB2 => 2_000_000_000,
            Size::GB5 => 5_000_000_000,
            Size::GB10 => 10_000_000_000,
            Size::GB15 => 15_000_000_000,
            Size::GB20 => 20_000_000_000,
            Size::GB25 => 25_000_000_000,
            Size::GB50 => 50_000_000_000,
            Size::GB100 => 100_000_000_000,
            Size::GB150 => 150_0000_000_000,
            Size::GB200 => 200_000_000_000,
            Size::GB250 => 250_000_000_000,
            Size::GB300 => 300_000_000_000,
            Size::GB500 => 500_000_000_000,
            Size::Custom(value) => *value,
        }
    }

    pub fn custom_from_str(s: &str) -> Result<Size, std::num::ParseIntError> {
        let value = s.trim().parse::<usize>()?;
        Ok(Size::Custom(value))
    }

    // impl Size {
    //     fn value(&self) -> usize {
    //         match self {
    //             Size::MB50 => 52_428_800,
    //             Size::MB100 => 104_857_600,
    //             Size::MB125 => 131_072_000,
    //             Size::MB128 => 134_217_728,
    //             Size::MB200 => 209_715_200,
    //             Size::MB250 => 262_144_000,
    //             Size::MB300 => 314_572_800,
    //             Size::MB500 => 536_870_912,
    //             Size::GB1 => 1_073_741_824,
    //             Size::GB2 => 2_147_483_648,
    //             Size::GB5 => 5_368_709_120,
    //             Size::GB10 => 10_737_418_240,
    //             Size::GB15 => 16_106_127_360,
    //             Size::GB20 => 21_474_836_480,
    //             Size::GB25 => 26_843_545_600,
    //             Size::GB50 => 53_687_091_200,
    //             Size::GB100 => 107_374_182_400,
    //             Size::GB150 => 161_061_273_600,
    //             Size::GB200 => 214_748_364_800,
    //             Size::GB250 => 268_435_456_000,
    //             Size::GB300 => 322_122_547_200,
    //             Size::GB500 => 536_870_912_000,
    //         }
    //     }

    pub fn is_exceeded_by(&self, val: usize) -> bool {
        val > self.value()
    }
}
