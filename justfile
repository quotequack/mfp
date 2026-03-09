test:
  cargo test
build_mfpcreate:
  cargo build --bin mfpcreate
build_mfpread:
  cargo build --bin mfpread 
install_all:
  cargo build --release --bin mfpcreate
  cargo build --release --bin mfpread 
  sudo cp target/release/mfpcreate /usr/bin 
  sudo cp target/release/mfpread /usr/bin 