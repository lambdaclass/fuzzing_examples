# Note on how to make it all work

* clone `cJSON` and run `make`
* cp the `libcjson.dylib.1.7.16` dynamic library to the root dir. There has to be a better way to make this work
* run bindgen `bindgen cJSON/cJSON.h > src/c_json.rs` to create the bindings
* create the `build.rs` file, copying and modifying the file shown in the [docs](https://rust-lang.github.io/rust-bindgen/non-system-libraries.html) seems to work fine

