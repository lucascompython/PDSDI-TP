#[cfg_attr(test, derive(PartialEq, Debug))]
pub enum OutfitType {
    LightAndBreezy,
    ComfortableAndCasual,
    ABitLayered,
    CozyAndWarm,
    HeavyAndInsulated,
}
#[derive(Clone)]
pub struct BoolPack(u16); // 0-11: bools, 12-15: OutfitType

impl BoolPack {
    /// Creates a new BoolPack with all bits set to 0 (false) and OutfitType::LightAndBreezy
    pub fn new() -> Self {
        BoolPack(0)
    }

    pub fn from_u16(value: u16) -> Self {
        BoolPack(value)
    }

    pub fn to_u16(self) -> u16 {
        self.0
    }

    pub fn set_bool(&mut self, index: usize, value: bool) {
        if index < 12 {
            if value {
                self.0 |= 1 << index;
            } else {
                self.0 &= !(1 << index);
            }
        }
    }

    pub fn get_bool(&self, index: usize) -> bool {
        if index < 12 {
            (self.0 & (1 << index)) != 0
        } else {
            false
        }
    }

    pub fn set_outfit_type(&mut self, mode: OutfitType) {
        self.0 &= 0b0000_1111_1111_1111;
        self.0 |= (mode as u16) << 12;
    }

    pub fn get_outfit_type(&self) -> OutfitType {
        match (self.0 >> 12) & 0b1111 {
            0 => OutfitType::LightAndBreezy,
            1 => OutfitType::ComfortableAndCasual,
            2 => OutfitType::ABitLayered,
            3 => OutfitType::CozyAndWarm,
            4 => OutfitType::HeavyAndInsulated,
            _ => panic!("Invalid outfit type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bool() {
        let pack = BoolPack(0b0000_0000_0001_0001);
        assert_eq!(pack.get_bool(0), true);
        assert_eq!(pack.get_bool(4), true);
        assert_eq!(pack.get_bool(1), false);
        assert_eq!(pack.get_bool(12), false);
    }

    #[test]
    fn test_set_and_get_outfit_type() {
        let mut pack = BoolPack(0);
        pack.set_outfit_type(OutfitType::HeavyAndInsulated);
        assert_eq!(pack.get_outfit_type(), OutfitType::HeavyAndInsulated);

        pack.set_outfit_type(OutfitType::CozyAndWarm);
        assert_eq!(pack.get_outfit_type(), OutfitType::CozyAndWarm);

        pack.set_outfit_type(OutfitType::ABitLayered);
        assert_eq!(pack.get_outfit_type(), OutfitType::ABitLayered);

        pack.set_outfit_type(OutfitType::ComfortableAndCasual);
        assert_eq!(pack.get_outfit_type(), OutfitType::ComfortableAndCasual);

        pack.set_outfit_type(OutfitType::LightAndBreezy);
        assert_eq!(pack.get_outfit_type(), OutfitType::LightAndBreezy);
    }
}
