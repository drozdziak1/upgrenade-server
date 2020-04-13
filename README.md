# upgrenade-server
A server for `check_upgrenade()` in the `upgrenade` upgrade checker crate.
Currently supports a simple format (version precedence is evaluated using semver
rules):
```toml
[config]
host = "127.0.0.1"
port = 8080

[[version]]
name = "0.1.0-alpha"
link = "https://your-site.org/download/0.1.0"

[[version]]
name = "0.1.0"
link = "https://your-site.org/download/0.1.0-beta"
```
