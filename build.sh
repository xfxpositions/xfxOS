cargo build --release --bin kernel --out-dir ./iso/kernel.bin -Z unstable-options
mkdir -p iso 
mkdir -p  iso/boot
mkdir -p  iso/boot/grub
echo 'set timeout=3' > iso/boot/grub/grub.cfg
echo 'set default=0' >> iso/boot/grub/grub.cfg
echo '' >> iso/boot/grub/grub.cfg
echo 'menuentry "xfxOS"{' >> iso/boot/grub/grub.cfg
echo '	multiboot /boot/kernel' >> iso/boot/grub/grub.cfg
echo '  boot' >> iso/boot/grub/grub.cfg
echo '}' >> iso/boot/grub/grub.cfg
echo "Build successful"
grub-mkrescue --output=xfxos.iso
