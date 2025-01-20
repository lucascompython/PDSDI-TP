import {
  serialize_clothes_to_bytes,
  deserialize_clothes_from_bytes,
  Clothe,
} from "./pkg/cbf.js";

const file1 = Bun.file("../resources/test_data/tshirt/black.png");
const file2 = Bun.file("../resources/test_data/pants/blue.png");

const clothes = [
  new Clothe(1, "T-Shirt", 2, 3, 1, false, "tshirt.png", await file1.bytes()),
  new Clothe(2, "Pants", 3, 4, 1, true, "pants.png", await file2.bytes()),
];

const serialized = serialize_clothes_to_bytes(clothes);

const response = await fetch(`http://localhost:1234/clothes/upload`, {
  method: "POST",
  headers: {
    "Content-Type": "application/octet-stream",
  },
  body: serialized,
});

console.log(response.ok);
