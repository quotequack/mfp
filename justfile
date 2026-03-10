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
  cp mfpRead.desktop ~/.local/share/applications/mfpRead.desktop
  cp mfpCreate.desktop ~/.local/share/applications/mfpCreate.desktop
  cp image-mfp.xml $HOME/.local/share/mime/packages/image-mfp.xml
  update-mime-database ~/.local/share/mime
install_read:
  cargo build --release --bin mfpread 
  sudo cp target/release/mfpread /usr/bin 
  cp mfpRead.desktop ~/.local/share/applications/mfpRead.desktop
install_create:
  cargo build --release --bin mfpcreate
  sudo cp target/release/mfpcreate /usr/bin 
  cp mfpCreate.desktop ~/.local/share/applications/mfpCreate.desktop
install_file_type:
  cp image-mfp.xml $HOME/.local/share/mime/packages/image-mfp.xml
  update-mime-database ~/.local/share/mime
remove_read:
  sudo rm /usr/bin/mfpread
  rm ~/.local/share/applications/mfpRead.desktop
remove_create:
  sudo rm /usr/bin/mfpcreate
  rm ~/.local/share/applications/mfpCreate.desktop
remove_all:
  sudo rm /usr/bin/mfpcreate
  sudo rm /usr/bin/mfpread
  rm $HOME/.local/mime/packages/image-mfp.xml
  update-mime-database ~/.local/share/mime
remove_file_type:
  rm $HOME/.local/mime/packages/image-mfp.xml
  update-mime-database ~/.local/share/mime