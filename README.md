# lee

`tee` command for CloudWatch Logs.

## Usage

```
Usage: lee [options]

Options:
    -g, --log-group-name NAME
                        log group name
    -s, --log-stream-name NAME
                        log stream name
    -v, --version       print version and exit
    -h, --help          print usage and exit
```

```sh
while true; do
  date
  sleep 1
done | lee -g my-group -s my-stream

# LogGroup/LogStream is created automatically
```
