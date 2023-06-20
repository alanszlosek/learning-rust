Test and sample apps for learning Rust. Check subfolders for individual apps/crates.

The most interesting so far is `mutexes`, which adds 1000 random strings to a Vec<String>, and spawns 10 threads that use a mutex to read from the vec/queue safely.
