
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

### resolvers.rs

Resolvers.rs is a rust file of all commonly used matches made to centralize them for easy contributions
Please append you codec accordingly in each match

### Other contributions

Are welcome! Comments are on the way for the rust files!