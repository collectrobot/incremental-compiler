# What is this
This is me going through the [Essentials of Compilation: The Incremental, Nano-Pass Approach](https://iucompilercourse.github.io/IU-P423-P523-E313-E513-Fall-2020/), but using [Rust](https://en.wikipedia.org/wiki/Rust_(programming_language)) instead of [Scheme](https://en.wikipedia.org/wiki/Scheme_(programming_language)).

# How to get started
This project currenly only runs on Windows.
It'll probably change later.

### Dependencies
- Windows
    - [Python3](https://www.python.org/downloads/)
    - [Rust](https://www.rust-lang.org/)
    - if you get the following error `error: linker 'link.exe' not found` when building the runtime, install [Visual Studio](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=Community&rel=16) with the C++ build tools.
- Linux
    - Not supported yet.

### Installing
- `git clone https://github.com/tbre90/incremental-compiler`

### Running
- Windows
    - Just building: `py project.py`
    - Start repl: `py project.py --op run`
        - type `:help` to get a list of available commands in the repl
    - Running tests
        - To run all tests: `py project --op test`
        - To run a specific test:
            ```cd compiler
            cargo test -- <name of test>
            e.g.: cargo test -- x64_build_add_two_read
            ```
- Linux
    - Not supported yet.