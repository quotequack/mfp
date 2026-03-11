
## Contributing

Contributing is simple!

### Adding a new codec
Check src/decoders/example.rs
Create your new decoder with the name lowercase
**Example:** example.rs

After adding the decode encode logic:
#### header.rs:

1. Enum at line 10, add your codecs name like:

```rust
    Example = 0x00 // 00 being the next hexadecimal number
```

2. Match at line 20, append you new codec like:

```rust
            0x00 => Ok(CodecId::Example),
```

#### lib.rs 

1. Match at line 35:

```rust
        CodecId::Example => decoders::example::decode(payload),
```

2. Match at line 45: 

```rust
        CodecId::Example => decoders::example::encode(payload),
```

#### mfpcreate.rs

1. Match at line 11:
```rust
        "example"  => CodecId::Example,
```

#### mfpextract.rs

1. Match at line 7:
```rust
        0x00 => CodecId::Example,
```

2. Match at line 18:
```rust
        CodecId::Example => decoders::example::encode(&decoded).expect("failed to encode"),
```

### mfpconvert.rs

1. Match at line 14:
```rust
        "example" => CodecId::Example,
```

2. Match at line 22:
```rust
        CodecId::Example => decoders::example::encode(&img).expect("failed to encode"),
```

### Other contributions

Are welcome! Comments are on the way for the rust files!