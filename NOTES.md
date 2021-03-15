# NOTES

## Run with command line arguments/flags

run with 

```shell
# work only without using `#[structopt(short = "?", long = "?")]`
$ cargo run -- pattern filepath

$ FILE="./logs/snapper.log"
# short
$ cargo run -- -f pattern -p ${FILE}
$ cargo run -- -p ${FILE} -p pattern
# long
$ cargo run -- --pattern pattern --path ${FILE}
$ cargo run -- --path ${FILE} --pattern pattern
```
