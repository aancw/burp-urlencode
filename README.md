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

## Example

```
❯ burp-urlencode -t ../../etc/shadow -i 4

    Burp Suite URL encoding all characters implementation in Rust
    by Petruknisme
    
[+] Encoding...
Text: ../../etc/shadow
Iteration: 4
[+] Result: 
%25%32%35%25%33%32%25%33%35%25%32%35%25%33%33%25%33%32%25%32%35%25%33%36%25%33%35%25%32%35%25%33%32%25%33%35%25%32%35%25%33%33%25%33%32%25%32%35%25%33%36%25%33%35%25%32%35%25%33%32%25%33%35%25%32%35%25%33%33%25%33%32%25%32%35%25%33%36%25%33%36%25%32%35%25%33%32%25%33%35%25%32%35%25%33%33%25%33%32%25%32%35%25%33%36%25%33%35%25%32%35%25%33%32%25%33%35%25%32%35%25%33%33%25%33%32%25%32%35%25%33%36%25%33%35%25%32%35%25%33%32%25%33%35%25%32%35%25%33%33%25%33%32%25%32%35%25%33%36%25%33%36%25%32%35%25%33%32%25%33%35%25%32%35%25%33%33%25%33%36%25%32%35%25%33%33%25%33%35%25%32%35%25%33%32%25%33%35%25%32%35%25%33%33%25%33%37%25%32%35%25%33%33%25%33%34%25%32%35%25%33%32%25%33%35%25%32%35%25%33%33%25%33%36%25%32%35%25%33%33%25%33%33%25%32%35%25%33%32%25%33%35%25%32%35%25%33%33%25%33%32%25%32%35%25%33%36%25%33%36%25%32%35%25%33%32%25%33%35%25%32%35%25%33%33%25%33%37%25%32%35%25%33%33%25%33%33%25%32%35%25%33%32%25%33%35%25%32%35%25%33%33%25%33%36%25%32%35%25%33%33%25%33%38%25%32%35%25%33%32%25%33%35%25%32%35%25%33%33%25%33%36%25%32%35%25%33%33%25%33%31%25%32%35%25%33%32%25%33%35%25%32%35%25%33%33%25%33%36%25%32%35%25%33%33%25%33%34%25%32%35%25%33%32%25%33%35%25%32%35%25%33%33%25%33%36%25%32%35%25%33%36%25%33%36%25%32%35%25%33%32%25%33%35%25%32%35%25%33%33%25%33%37%25%32%35%25%33%33%25%33%37
```

## Support 

- [x] URL Encoding all character like in burp
- [x] Multiple url encoding iteration support
- [x] Support silent mode 

## Motivation

Porting the little feature from burp suite, so everyone can use it without burp suite.

## License

MIT License