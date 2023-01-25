# burp-urlencode
Burp Suite URL encoding all characters implementation in Rust

## Usage

```
❯ burp-urlencode -h

burp-urlencode 1.0
Petruknisme <me@petruknisme.com>
Burp Suite URL encoding all characters implementation in Rust

USAGE:
    burp-urlencode --text <TEXT> --iteration <ITERATION>

OPTIONS:
    -h, --help                     Print help information
    -i, --iteration <ITERATION>    How many time text will be url encoded
    -t, --text <TEXT>              Text to be encoded
    -V, --version                  Print version information
```

## Support 

[x] URL Encoding all character like in burp
[x] Multiple url encoding iteration support

## Motivation

Porting the little feature from burp suite, so everyone can use it without burp suite.

## License

MIT License