# How to work with please

# Philosophy

`please` strives to do what `cargo` does; from functionality to developer experience and user experience. `please` tries to keep everything as simple as possible, and uses simple tools to do so. Therefore I chose TOML as the configuration language of `please`. Well-known and very simple.

Why TOML?

I looked through some C and C++ build-systems. Most of them - aside from a few exceptions like meson - invented a whole new language just to compile code. I was very surprised that it is so hard to find a build system that doesn't use an unfathomable scripting language just to compile code.

`please` is made to be compatible with other build systems; you don't need to rewrite your Makefiles in `build.toml` syntax.

How? Let's get started with some documentation!