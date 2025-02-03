# Custom Binary Format for Clothes because it's lowkey easier than Multipart Form Data

## Why?

I have the following data, that I need to share between the backend and the frontend back and forth:

```rust
struct Clothe {
    id: i16, // tokio-postgres doesn't support u16
    name: String,
    color: Color,
    category: Category,
    user_id: i16, // tokio-postgres doesn't support u16
    is_for_hot_weather: bool,
    image: Vec<u8>,
    file_name: String,
}
```

I need to send multiple of these at a time. For efficiency reasons, I didn't want to make multiple requests. So I initially thought of using multipart form data, where I serialized the data as JSON and sent the file as application/octet-stream. But I was having a hard time getting the data back on the frontend. So I decided to create a custom binary format for this data, that is both more efficient and lowkey easier to implement.  

## How does it work?

The format is as follows:

1. `id`: 2 bytes
2. `name_size`: 1 byte
3. `name`: `name_size` bytes
4. `color` and `category`: 1 byte (4 bits each)
5. `user_id` and `is_for_hot_weather`: 2 bytes
    - `user_id`: 15 bits
    - `is_for_hot_weather`: 1 bit
6. `image_size`: 3 bytes
7. `image`: `image_size` bytes

## Encoding and Decoding is implemented in Rust and is compiled to WebAssembly for the frontend to use

Compiling to WebAssembly: `wasm-pack build --target web`

## Testing

```bash
cargo test

## Building WebAssembly
wasm-pack build --target nodejs

## Test the WebAssembly
cd tests
bun test # Only test WebAssembly
bun run index.ts --help # Test server
```

## TODO: make.py script and the rust optimizations
