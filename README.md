tiny-resolver-rs
================

Inspired by Julia Evans' [A toy DNS
resolver](https://jvns.ca/blog/2022/02/01/a-dns-resolver-in-80-lines-of-go/),
I decided to Rewrite It In Rust. Julia's repository is
https://github.com/jvns/tiny-resolver

Pulling from the repository then compiling should be straigthforward.

```
$ git pull https://github.com/kmkaplan/tiny-resolver-rs.git
$ cd tiny-resolver-rs
$ cargo build --release
```

Here's an example usage.

```
$ target/release/tiny-resolver www.kim-minh.com
dig -r @198.41.0.4 www.kim-minh.com
  e.gtld-servers.net. 172800 IN A 192.12.94.30
dig -r @192.12.94.30 www.kim-minh.com
  ns.kimsufi.com. 172800 IN A 213.186.33.199
dig -r @213.186.33.199 www.kim-minh.com
  ko.kim-minh.com. 43200 IN A 176.31.117.54
Result: 176.31.117.54
```