# ctfsak (CTF Swiss Army Knife)

This is a tool to help saving time during CTFs, where it's common
to have to do a lot of encoding/decoding, encrypting/decrypting,
bytes manipulation etc. My aim for ctfsak is for it to be the only
tool one can ever need for doing such activities.
Of course it can be used also as a general-purpose tool, but the
development is not focused on this.
(This software is still in its very infancy, and far from complete.
Contributors are welcomed).

## Build
Make sure you have the latest version of Rust installed on your system, then run:

```
$ git clone https://github.com/andreademurtas/ctfsak.git && cd ctfsak
$ cargo build --release
```

You will find the executable under target/release/, labeled as 'ctfsak'
