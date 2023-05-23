# parser

### Generate the `cpp/code.cpp`'s AST dump with `clang`:
`clang -Xclang -ast-dump -fsyntax-only -std=c++17 -fno-color-diagnostics cpp/code.cpp > ./src/dump.txt`

`-fno-color-diagnostics` because otherwise the generated AST dump has shell color codes in it.

### Run the program
`cargo run`

Running the program will give you a prettified JSON object as an output, and write the output to `output.json`.


### Notes
Could've used `clang -ast-dump=json`, but then what's the point of this project, life, or anything.