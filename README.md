# YADIL: Yet another data interchange language

> [!WARNING]
> There's no working version of yadil yet. A parser is being built for rust.

Yep, it's exactly what it sounds.

```py
Y.1;
# This is a comment #
# \# works #

str @a_string = "Hello, world!"; # string #
uint @some_uint = 42; # unsigned integer #
sint @i_hate_sints = -42; # signed integer #
float @my_float = 3.14159; # floating point number #
bool @is_it_ok = true; # boolean #
list @lists_work = [1, 2, 3]; # lists #
bytes @weird_bytes = [1A, 2F, 03]; # bytes: equivalent to 0x1A, 0x2F, 03 #

map @my_map: # maps #
    str @key "value";
    uint @key2 42;
end;
```

it's that simple.

## Why?

You might (not) noticed, but `yadil` works perfectly without spaces, newlines, or any other whitespace form. That's intentional. `yadil` is meant to be easy to read by a machine and, if necessary, by a human. It's meant to reduce the amount of bytes needed to pass data between server and client (which, honestly, JSON does a poor job at). It's easy to parse, and easy to generate.

## Shorter data types

`yadil` supports single-character acronyms for data types. This is not meant for human use, but can be useful in data passing.

| Parser-acceptable | Acronym | Data type             |
| ----------------- | ------- | --------------------- |
| s                 | str     | String                |
| u                 | uint    | Unsigned integer      |
| i                 | sint    | Signed integer        |
| f                 | float   | Floating point number |
| b                 | bool    | Boolean               |
| l                 | list    | List                  |
| m                 | map     | HashMap               |
| x                 | bytes   | A byte list           |

you can also use `t` for true and `f` for false:

```py
b@my_bool=t;
```

## Roadmap

- [ ] Rust parser
- [ ] Integration with `serde`
- [ ] Implementation in JavaScript (for both browser & nodejs/bun/deno)
- [ ] Language server (which shouldn't be hard)
- [ ] Extensions for ides & editors
