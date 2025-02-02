import { parseArgs } from "util";
import {
  serializeClothesToBytes,
  deserializeClothesFromBytes,
  Clothe,
} from "../pkg/cbf.js";

const { values, positionals } = parseArgs({
  args: Bun.argv,
  options: {
    get: {
      type: "boolean",
    },
    post: {
      type: "boolean",
    },
    help: {
      type: "boolean",
    },
  },
  strict: true,
  allowPositionals: true,
});

if (values.help || positionals.length === 0 || (!values.get && !values.post)) {
  console.log(`Usage: cbf [options] [arguments]
Options:
    --get    Download clothes from the server
    --post   Upload clothes to the server
    --help   Display this help message
`);
  process.exit(0);
}

if (values.get) {
  console.log("GET");

  const resp = await fetch("http://localhost:1234/clothes/get");

  if (!resp.ok) {
    console.error(
      `Failed to download clothes\nCheck if the server is running\nStatus: ${resp.status}`
    );
    process.exit(1);
  }

  const buffer = await resp.arrayBuffer();
  console.log("length:", buffer.byteLength);
  const clothes = deserializeClothesFromBytes(new Uint8Array(buffer));

  for (const clothe of clothes) {
    console.log(`Clothe {
        id: ${clothe.id},
        name: ${clothe.name},
        color: ${clothe.color},
        category: ${clothe.category},
        user_id: ${clothe.user_id},
        is_for_hot_weather: ${clothe.is_for_hot_weather},
        file_name: ${clothe.file_name},
        file.length: ${clothe.file.length},
}`);

    await Bun.write(`clothes/${clothe.id}-${clothe.name}.png`, clothe.file);
  }

  console.log("\nClothes downloaded successfully");
}
if (values.post) {
  console.log("POST");

  const file1 = Bun.file("../../resources/test_data/tshirt/black.png");
  const file2 = Bun.file("../../resources/test_data/pants/blue.png");

  const clothes = [
    new Clothe(0, "T-Shirt", 2, 3, 1, false, "tshirt.png", await file1.bytes()),
    new Clothe(0, "Pants", 3, 4, 1, true, "pants.png", await file2.bytes()),
  ];

  const serialized = serializeClothesToBytes(clothes);

  const response = await fetch(`http://localhost:1234/clothes/upload`, {
    method: "POST",
    headers: {
      "Content-Type": "application/octet-stream",
    },
    body: serialized,
  });

  if (!response.ok) {
    console.error(
      `Failed to upload clothes\nCheck if cookies are enabled\nStatus: ${response.status}`
    );
  } else {
    console.log("Clothes uploaded successfully");
  }
}
