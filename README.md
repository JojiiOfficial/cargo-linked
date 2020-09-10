# cargo-linked
Display the packages a rust binary is linked against. As cargo subcommand!

Easy said: run `cargo linked` to find out which packages you must have installed.

# Installation
`cargo install cargo-linked`

# Usage
Navigate to a cargo directory and run `cargo linked`. You'll see a list of packages the rust binary is linked against.<br>
If you want to run the given rust application somewhere else, make sure you install those packages first.

# Note
Only arch packages are supported by now. PRs are welcome!
