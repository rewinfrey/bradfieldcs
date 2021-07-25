# RustyLox

This is a Rust implementation of the Lox programming language found in Bob Nystrom's [_Crafting Interpreters_][crafting-interpreters].

### Get started

Assumes an already installed Rust environment. New to rust? See the official [Install Rust][install-rust] guide to get started.

```shell
$ git clone https://github.com/rewinfrey/bradfieldcs
$ cd bradfieldcs/languages-compilers-interpreters/rustylox
$ cargo build
```

### Run the REPL

By default, running the binary will start the REPL.

```shell
$ cargo run
```

### Run a file

Passing in a file path will interpret the file.

```shell
$ cargo run -- path/to/file
```

[crafting-interpreters]: http://craftinginterpreters.com
[install-rust]: https://www.rust-lang.org/tools/install

### License

Copyright 2021 Rick Winfrey

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
