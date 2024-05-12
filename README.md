# Minigrep
This tool is the light version of the grep, developed while studing the [rust book](https://rust-book.cs.brown.edu/). It contains basic functionality for searching within a file from the command line.


## Usage
This project uses cargo as the package manager, therefore to run the program: 

### Run dev
`cargo run -- [search query] [filepath] [opts]`

#### Options
The only option available is `--case-sensitive`. This option enables you to search for words no matter the case of the letters, you can also set it as an environment variable as `IGNORE_CASE=true`. 

### Tests
`cargo test`



