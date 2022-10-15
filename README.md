# custom_user_id

Generates a random id with a string of your choice appended or prepended to it. This is useful if you want to track users across multiple devices and platforms. This crate uses uuid under the hood with js enabled so it can be used for WebAssembly.


## Usage
```rust
use custom_user_id::generate_id;

// Set second argument to true if you want to prepend the id with the string
generate_id("my_app", true) // my-app-155abb84-81d5-4f36-9220-ef664e785195

// Set second argument to false if you want to append the id with the string
generate_id("my_app", false) // 7929654a-21a9-4dc5-9ef9-e84a49e393ab-my-app

```
