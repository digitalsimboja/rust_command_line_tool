# rust_command_line_tool
A command line tool built with Rust Language


---To Setup---
`git clone https://github.com/digitalsimboja/rust_command_line_tool.git`

cd into rust_command_line_tool
Ensure you have Rust installed on your machine. Then build the binary

`cargo build`

To run the command line, you can pass any of the words contained in poem.txt

`cargo run the poem.txt`

To check for case sensitivity:

` $Env:CASE_INSENSITIVE=1; cargo run to poem.txt`

You will notice a case_insensitive environment variable created in your project root folder and set==1

To remove/unset the case_insensitive env variable

`Remove-Item Env:CASE_INSENSITIVE`

---Testing---

`cargo test`
