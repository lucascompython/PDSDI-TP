# TODO: Only implement this bool pack functionality once in rust
from enum import IntEnum


class OutfitType(IntEnum):
    LightAndBreezy = 0
    ComfortableAndCasual = 1
    ABitLayered = 2
    CozyAndWarm = 3
    HeavyAndInsulated = 4


class Color(IntEnum):
    Red = 0
    Orange = 1
    Yellow = 2
    Green = 3
    Blue = 4
    Purple = 5
    Pink = 6
    Brown = 7
    Black = 8
    White = 9
    Gold = 10
    Gray = 11


class Category(IntEnum):
    Shirt = 0
    Pants = 1
    Shorts = 2
    Dress = 3
    Skirt = 4
    Jacket = 5
    Sweater = 6
    Shoes = 7
    Hat = 8
    Socks = 9
    Gloves = 10
    Scarf = 11


class BoolPack:
    def __init__(self, value: int = 0):
        self.value = value & 0xFFFF  # Ensure it's a 16-bit number

    @classmethod
    def from_int(cls, value: int) -> "BoolPack":
        return cls(value)

    def to_int(self) -> int:
        return self.value

    def set_bool(self, index: int, flag: bool):
        if 0 <= index < 12:
            if flag:
                self.value |= 1 << index
            else:
                self.value &= ~(1 << index)  # Clear

    def get_bool(self, index: int) -> bool:
        if 0 <= index < 12:
            return (self.value & (1 << index)) != 0
        return False

    def set_outfit_type(self, outfit: OutfitType):
        self.value &= 0b0000111111111111
        self.value |= outfit.value << 12

    def get_outfit_type(self) -> OutfitType:
        outfit_bits = (self.value >> 12) & 0b1111
        if outfit_bits in OutfitType._value2member_map_:
            return OutfitType(outfit_bits)
        raise ValueError("Invalid outfit type")

    def get_colors(self) -> list[Color]:
        colors = []
        for i in range(12):
            if self.get_bool(i):
                if i in Color._value2member_map_:
                    colors.append(Color(i))
        return colors

    def __str__(self):
        return f"BoolPack({bin(self.value)[2:].zfill(16)})"


if __name__ == "__main__":
    pack = BoolPack()
    pack.set_bool(3, True)
    pack.set_bool(7, True)
    pack.set_bool(11, True)
    pack.set_outfit_type(OutfitType.ABitLayered)

    print(pack)  # Binary representation
    print("Bit 3:", pack.get_bool(3))  # True
    print("Bit 7:", pack.get_bool(7))  # True
    print("Bit 11:", pack.get_bool(11))  # True
    print("Outfit Type:", pack.get_outfit_type().name)  # "ABitLayered"
