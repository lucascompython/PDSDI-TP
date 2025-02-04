// TODO: Only implement this bool pack functionality once in rust

export enum OutfitType {
  LightAndBreezy = 0,
  ComfortableAndCasual = 1,
  ABitLayered = 2,
  CozyAndWarm = 3,
  HeavyAndInsulated = 4,
}

export class BoolPack {
  private value: number;

  constructor(value: number = 0) {
    this.value = value & 0xffff;
  }

  static fromNumber(value: number): BoolPack {
    return new BoolPack(value);
  }

  toNumber(): number {
    return this.value;
  }

  setBool(index: number, flag: boolean): void {
    if (index < 0 || index >= 12) return;
    if (flag) {
      this.value |= 1 << index;
    } else {
      this.value &= ~(1 << index);
    }
  }

  getBool(index: number): boolean {
    if (index < 0 || index >= 12) return false;
    return (this.value & (1 << index)) !== 0;
  }

  setOutfitType(outfit: OutfitType): void {
    this.value &= 0b0000111111111111;
    this.value |= outfit << 12;
  }

  getOutfitType(): OutfitType {
    const outfitBits = (this.value >> 12) & 0b1111;
    if (outfitBits in OutfitType) {
      return outfitBits as OutfitType;
    }
    throw new Error("Invalid outfit type");
  }
}
