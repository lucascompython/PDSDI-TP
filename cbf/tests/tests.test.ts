import { expect, test } from "bun:test";
import {
  serializeClothesToBytes,
  deserializeClothesFromBytes,
  Clothe,
} from "../pkg/cbf.js";

test("serialize and deserialize a single Clothe", () => {
  const clothe = new Clothe(
    1,
    "T-Shirt",
    2,
    3,
    100,
    true,
    "tshirt.png",
    new Uint8Array([1, 2, 3, 4, 5])
  );

  const serialized = clothe.to_bytes();
  const deserialized = Clothe.from_bytes(serialized);

  expect(deserialized.id).toBe(clothe.id);
  expect(deserialized.name).toBe(clothe.name);
  expect(deserialized.color).toBe(clothe.color);
  expect(deserialized.category).toBe(clothe.category);
  expect(deserialized.user_id).toBe(clothe.user_id);
  expect(deserialized.is_for_hot_weather).toBe(clothe.is_for_hot_weather);
  expect(deserialized.file_name).toBe(clothe.file_name);
  expect(deserialized.file).toEqual(clothe.file);
});

test("serialize and deserialize multiple Clothes", () => {
  const clothes = [
    new Clothe(
      1,
      "T-Shirt",
      2,
      3,
      100,
      true,
      "tshirt.png",
      new Uint8Array([1, 2, 3, 4, 5])
    ),
    new Clothe(
      2,
      "Pants",
      3,
      4,
      100,
      true,
      "pants.png",
      new Uint8Array([1, 2, 3, 4, 5])
    ),
  ];

  const serialized = serializeClothesToBytes(clothes);
  const deserialized = deserializeClothesFromBytes(serialized);

  expect(deserialized.length).toBe(clothes.length);
  for (let i = 0; i < clothes.length; i++) {
    expect(deserialized[i].id).toBe(clothes[i].id);
    expect(deserialized[i].name).toBe(clothes[i].name);
    expect(deserialized[i].color).toBe(clothes[i].color);
    expect(deserialized[i].category).toBe(clothes[i].category);
    expect(deserialized[i].user_id).toBe(clothes[i].user_id);
    expect(deserialized[i].is_for_hot_weather).toBe(
      clothes[i].is_for_hot_weather
    );
    expect(deserialized[i].file_name).toBe(clothes[i].file_name);
    expect(deserialized[i].file).toEqual(clothes[i].file);
  }
});
