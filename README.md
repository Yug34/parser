# parser

Just a toy parser, mostly did this to play around with IO and String processing in Rust.

It walks the AST dump of some C++ code and extracts some meaningful info, outputs it as a JSON object.

Normally I'd have an FSA implementation instead of something like this.

### Generate the `cpp/code.cpp`'s AST dump with `clang`:
`clang -Xclang -ast-dump -fsyntax-only -std=c++17 -fno-color-diagnostics cpp/code.cpp > ./src/dump.txt`

Using `-fno-color-diagnostics` because otherwise the generated AST dump has shell color codes in it.

Using `-fsyntax-only` since we only need to run the preprocessor, parser and type checking stages of `clang`.


### Run the program
`cargo run`

Running the program will give you a prettified JSON object as an output, and write the output to `output.json`.


### Notes
Could've used `clang -ast-dump=json`, but then what's the point of this project, life, or anything.
