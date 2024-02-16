# minikeyvalue

```html
Usage: mkv --volumes <VOLUMES> --db <PATH> [OPTIONS] <COMMAND>

Commands:
  server
  rebuild    Change the amount of volume servers
  rebalance  Regenerate LevelDB
  help       Print this message or the help of the given subcommand(s)

Options:
      --db <PATH>            Path to database
      --fallback <FALLBACK>  Fallback server for missing keys
      --port <PORT>          Port for the server to listen on [default: 3000]
      --protect              Force UNLINK before DELETE
      --replicas <AMOUNT>    Amount of replicas to make of the data [default: 3]
      --volumes <VOLUMES>    Volumes to use for storage, comma separated
      --subvolumes <AMOUNT>  Amount of subvolumes, disk per machine [default: 10]
  -h, --help                 Print help
  -V, --version              Print version
```
