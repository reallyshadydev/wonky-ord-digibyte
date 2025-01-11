use super::*;
use bitcoin::blockdata::constants::COIN_VALUE;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Display, PartialOrd)]
pub(crate) struct Epoch(pub(crate) u32);

impl Epoch {
    pub(crate) const STARTING_SATS: [u64; 7] = [
        0,
        120_000_000_000,
        216_000_000_000,
        280_800_000_000,
        319_680_000_000,
        343_728_000_000,
        360_491_520_000,
    ];

    pub(crate) fn subsidy(self) -> u64 {
        match self.0 {
            0 => 12_000 * COIN_VALUE,
            1 => 6_000 * COIN_VALUE,
            2 => 3_000 * COIN_VALUE,
            3 => 1_500 * COIN_VALUE,
            4 => 750 * COIN_VALUE,
            5 => 375 * COIN_VALUE,
            6 => 100 * COIN_VALUE, // Constant gradual reduction
            _ => panic!("bad epoch"),
        }
    }

    pub(crate) fn starting_sat(self) -> u64 {
        *Self::STARTING_SATS
            .get(usize::try_from(self.0).unwrap())
            .unwrap_or_else(|| Self::STARTING_SATS.last().unwrap())
            * COIN_VALUE
    }

    pub(crate) fn starting_height(self) -> Height {
        match self.0 {
            0 => Height(0),
            1 => Height(432_000),
            2 => Height(1_261_440),
            3 => Height(2_102_400),
            4 => Height(3_150_528),
            5 => Height(4_500_000),
            6 => Height(5_000_000),
            _ => panic!("bad epoch"),
        }
    }
}

impl PartialEq<u32> for Epoch {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl From<u64> for Epoch {
    fn from(sat: u64) -> Self {
        if sat < Self::STARTING_SATS[1] * COIN_VALUE {
            Epoch(0)
        } else if sat < Self::STARTING_SATS[2] * COIN_VALUE {
            Epoch(1)
        } else if sat < Self::STARTING_SATS[3] * COIN_VALUE {
            Epoch(2)
        } else if sat < Self::STARTING_SATS[4] * COIN_VALUE {
            Epoch(3)
        } else if sat < Self::STARTING_SATS[5] * COIN_VALUE {
            Epoch(4)
        } else if sat < Self::STARTING_SATS[6] * COIN_VALUE {
            Epoch(5)
        } else {
            Epoch(6)
        }
    }
}

impl From<Height> for Epoch {
    fn from(height: Height) -> Self {
        if height.0 < 432_000 {
            Epoch(0)
        } else if height.0 < 1_261_440 {
            Epoch(1)
        } else if height.0 < 2_102_400 {
            Epoch(2)
        } else if height.0 < 3_150_528 {
            Epoch(3)
        } else if height.0 < 4_500_000 {
            Epoch(4)
        } else if height.0 < 5_000_000 {
            Epoch(5)
        } else {
            Epoch(6)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starting_sat() {
        assert_eq!(Epoch(0).starting_sat(), 0);
    }

    #[test]
    fn subsidy() {
        assert_eq!(Epoch(0).subsidy(), 12_000 * COIN_VALUE);
        assert_eq!(Epoch(1).subsidy(), 6_000 * COIN_VALUE);
        assert_eq!(Epoch(6).subsidy(), 100 * COIN_VALUE);
    }

    #[test]
    fn starting_height() {
        assert_eq!(Epoch(0).starting_height(), Height(0));
        assert_eq!(Epoch(1).starting_height(), Height(432_000));
        assert_eq!(Epoch(2).starting_height(), Height(1_261_440));
    }

    #[test]
    fn from_height() {
        assert_eq!(Epoch::from(Height(0)), Epoch(0));
        assert_eq!(Epoch::from(Height(432_000)), Epoch(1));
        assert_eq!(Epoch::from(Height(1_261_440)), Epoch(2));
    }

    #[test]
    fn from_sat() {
        for (epoch, starting_sat) in Epoch::STARTING_SATS.iter().enumerate() {
            if epoch > 0 {
                assert_eq!(
                    Epoch::from((starting_sat - 1) * COIN_VALUE),
                    Epoch((epoch - 1) as u64)
                );
            }
            assert_eq!(Epoch::from(starting_sat * COIN_VALUE), Epoch(epoch as u64));
            assert_eq!(Epoch::from((starting_sat + 1) * COIN_VALUE), Epoch(epoch as u64));
        }
        assert_eq!(Epoch::from(0), Epoch(0));
        assert_eq!(Epoch::from(COIN_VALUE), Epoch(0));
        assert_eq!(Epoch::from(Epoch(1).starting_sat()), Epoch(1));
        assert_eq!(Epoch::from(Epoch(1).starting_sat() + COIN_VALUE), Epoch(1));
    }

    #[test]
    fn eq() {
        assert_eq!(Epoch(0), 0);
        assert_eq!(Epoch(100), 100);
    }
}
