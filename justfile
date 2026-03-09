test:
  cargo test
build_create:
  cargo build --bin mfpcreate
build_read:
  cargo build --bin mfpread 
run_create:
  cargo run --bin mfpcreate 
run_read:
  cargo run --bin mfpread -- ./testOut/test1.mfp
install_all:
  cargo build --release --bin mfpcreate
  cargo build --release --bin mfpread 
  sudo cp target/release/mfpcreate /usr/bin 
  sudo cp target/release/mfpread /usr/bin 
install_read:
  cargo build --release --bin mfpread 
  sudo cp target/release/mfpread /usr/bin 
install_create:
  cargo build --release --bin mfpcreate
  sudo cp target/release/mfpcreate /usr/bin 
remove_read:
  sudo rm /usr/bin/mfpread
remove_create:
  sudo rm /usr/bin/mfpcreate
remove_all:
  sudo rm /usr/bin/mfpcreate
  sudo rm /usr/bin/mfpread