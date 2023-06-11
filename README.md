Testing out the size benefits of alternative compression for updates.

The following use best release profile settings for small size and is built with `cargo tauri build --target aarch64-apple-darwin -- -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort`.

The after column is the sizes after a zstd decompression command is added to force the size of the zstd decompression code to be added to the resulting binary.

| format | before | after | diff |
| --- | --- | --- | --- |
| binary | `2,388,032` | `2,570,656` | +`182,624` |
| dmg | `1,314,086` | `1,400,766` | +`86,680` |
| tar | `2,492,416` | `2,674,688` | +`182,272` |
| gzip | `1,306,251` | `1,399,526` | +`93,275` |
| zstd | `1,002,715` | `1,073,985` | +`71,270` |
| brotli | `913,697` | `981,397` | +`67,700` |

Notably, since I left compression feature enabled, brotli decompression is already included in the final build which we last estimated to add ~170KiB, so pretty close to zstd at ~178KiB.

Compression time for zstd archives took 1.3s on my m2 pro while brotli took about 4.5s (using the command line and not the crate). Using the rust implementation of brotli, it took about 3s to compress since it adds multithreaded compression, while only increasing the size by ~1-2KiB.

I think the compression speed matters some, since CI builds are going to be slower since they typically only 2-3 cores and slower execution speed than an m2 pro. I don't think it would make sense to add zstd compression for updates while we already include brotli when the compression feature is enabled. If we are extending where plugins can be added, like build time plugins where compression could be set, then we might be able to set per-project a build time plugin for which compression scheme you want. Otherwise, I think zstd might be a good balance if we use it completely for compression (since we don't use the benefits of brotli to pass-thru assets without decompressing) although we have run into build issues with it before with how build scripts don't limit themselves and cause multiple native libs to be built all at once.
