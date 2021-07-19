# html2pdf

Just a CLI over the [`headless_chrome`](https://crates.io/crates/headless_chrome) crate to create PDF.

## Install

Need the Rust toolchain: <https://rustup.rs/>.

```shell
cargo install html2pdf
```

## Usage

```shell
html2pdf path/to/file.html
```

To remove logs, set the env var `RUST_LOG` to `none` :s

```shell
RUST_LOG="none" html2pdf path/to/file.html
```

## Options

```shell
html2pdf 0.1.0

USAGE:
    html2pdf [FLAGS] [OPTIONS] <input>

FLAGS:
        --background    
            Allow print background

    -h, --help          
            Prints help information

        --landscape     
            Use landscape mode

    -V, --version       
            Prints version information


OPTIONS:
    -o, --output <output>    
            Output file
            
            By default, just change the input extension to PDF

ARGS:
    <input>    
            Input HTML file
```