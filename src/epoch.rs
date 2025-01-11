use super::*;
use bitcoin::blockdata::constants::COIN_VALUE;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Display, PartialOrd)]
pub(crate) struct Epoch(pub(crate) u32);

impl Epoch {
    pub(crate) const STARTING_SATS: [Sat; 8] = [
        Sat(0),
        Sat(8_000_000_000_000), // Epoch 1: 8,000 coins per block
        Sat(16_000_000_000_000), // Epoch 2
        Sat(24_000_000_000_000), // Epoch 3
        Sat(32_000_000_000_000), // Epoch 4
        Sat(40_000_000_000_000), // Epoch 5
        Sat(48_000_000_000_000), // Epoch 6
        Sat(56_000_000_000_000), // Constant inflation
    ];

    pub(crate) fn subsidy(self) -> u64 {
        match self.0 {
            0 => 8_000 * COIN_VALUE, // Initial subsidy
            1 => 4_000 * COIN_VALUE,
            2 => 2_000 * COIN_VALUE,
            3 => 1_000 * COIN_VALUE,
            4 => 500 * COIN_VALUE,
            5 => 250 * COIN_VALUE,
            6 => 125 * COIN_VALUE, // Reduced inflation starts at epoch 6
            _ => 125 * COIN_VALUE, // Constant inflation beyond epoch 6
        }
    }

    pub(crate) fn starting_sat(self) -> Sat {
        *Self::STARTING_SATS
            .get(usize::try_from(self.0).unwrap())
            .unwrap_or_else(|| Self::STARTING_SATS.last().unwrap())
    }

    pub(crate) fn starting_height(self) -> Height {
        match self.0 {
            0 => Height(0),
            1 => Height(100_000),
            2 => Height(200_000),
            3 => Height(300_000),
            4 => Height(400_000),
            5 => Height(500_000),
            6 => Height(600_000), // Constant inflation starts after 600,000
            _ => panic!("Invalid epoch"),
        }
    }
}

impl PartialEq<u32> for Epoch {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl From<Sat> for Epoch {
    fn from(sat: Sat) -> Self {
        if sat < Self::STARTING_SATS[1] {
            Epoch(0)
        } else if sat < Self::STARTING_SATS[2] {
            Epoch(1)
        } else if sat < Self::STARTING_SATS[3] {
            Epoch(2)
        } else if sat < Self::STARTING_SATS[4] {
            Epoch(3)
        } else if sat < Self::STARTING_SATS[5] {
            Epoch(4)
        } else if sat < Self::STARTING_SATS[6] {
            Epoch(5)
        } else {
            Epoch(6)
        }
    }
}

impl From<Height> for Epoch {
    fn from(height: Height) -> Self {
        if height.0 < 100_000 {
            Epoch(0)
        } else if height.0 < 200_000 {
            Epoch(1)
        } else if height.0 < 300_000 {
            Epoch(2)
        } else if height.0 < 400_000 {
            Epoch(3)
        } else if height.0 < 500_000 {
            Epoch(4)
        } else if height.0 < 600_000 {
            Epoch(5)
        } else {
            Epoch(6)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn starting_sat() {
        assert_eq!(Epoch(0).starting_sat(), Sat(0));
    }

    #[test]
    fn subsidy() {
        assert_eq!(Epoch(0).subsidy(), 8_000 * COIN_VALUE);
        assert_eq!(Epoch(1).subsidy(), 4_000 * COIN_VALUE);
        assert_eq!(Epoch(6).subsidy(), 125 * COIN_VALUE); // Constant inflation
    }

    #[test]
    fn starting_height() {
        assert_eq!(Epoch(0).starting_height(), Height(0));
        assert_eq!(Epoch(1).starting_height(), Height(100_000));
        assert_eq!(Epoch(2).starting_height(), Height(200_000));
    }

    #[test]
    fn from_height() {
        assert_eq!(Epoch::from(Height(0)), Epoch(0));
        assert_eq!(Epoch::from(Height(100_000)), Epoch(1));
        assert_eq!(Epoch::from(Height(200_000)), Epoch(2));
    }

    #[test]
    fn from_sat() {
        for (epoch, starting_sat) in Epoch::STARTING_SATS.iter().enumerate() {
            if epoch > 0 {
                assert_eq!(
                    Epoch::from(Sat(starting_sat.n() - 1)),
                    Epoch((epoch - 1) as u64)
                );
            }
            assert_eq!(Epoch::from(*starting_sat), Epoch(epoch as u64));
            assert_eq!(Epoch::from(*starting_sat + 1), Epoch(epoch as u64));
        }
        assert_eq!(Epoch::from(Sat(0)), Epoch(0));
        assert_eq!(Epoch::from(Sat(1)), Epoch(0));
        assert_eq!(Epoch::from(Epoch(1).starting_sat()), Epoch(1));
        assert_eq!(Epoch::from(Epoch(1).starting_sat() + 1), Epoch(1));
    }

    #[test]
    fn eq() {
        assert_eq!(Epoch(0), 0);
        assert_eq!(Epoch(100), 100);
    }
}
