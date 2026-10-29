#!/bin/bash
set -e

echo "[*] Starting SMTH OS ISO Build Pipeline..."

# 1. Update and install required host tools + Debian keyring
sudo apt-get update
sudo apt-get install -y live-build debootstrap xorriso squashfs-tools mtools genisoimage debian-archive-keyring

# 2. Create live-build workspace directory structure
mkdir -p build-workspace/config
cd build-workspace

# 3. Configure Debian Live environment
lb config \
  --distribution trixie \
  --architectures amd64 \
  --binary-images iso-hybrid \
  --bootloader grub-efi,syslinux \
  --debian-installer none \
  --archive-areas "main contrib non-free-firmware" \
  --linux-flavours amd64 \
  --bootappend-live "boot=live quiet splash loglevel=3 zswap.enabled=1" \
  --iso-application "SMTH-OS" \
  --iso-volume "SMTH_OS"

# 4. Define required system packages in chroot
mkdir -p config/package-lists
cat << 'EOF' > config/package-lists/smth.list.chroot
linux-image-amd64
systemd-sysv
seatd
libinput10
libdrm2
mesa-vulkan-drivers
mesa-utils
xwayland
pipewire
wireplumber
network-manager
bluez
flatpak
lxc
zram-tools
fonts-inter
fonts-roboto
plymouth
foot
zsh
EOF

# 5. Add post-install hook to enable core services
mkdir -p config/hooks/normal
cat << 'EOF' > config/hooks/normal/9000-smth.hook.chroot
#!/bin/sh
set -e
systemctl enable smthd.service freezerd.service zramswap.service NetworkManager.service bluetooth.service || true
systemctl set-default graphical.target
echo 'vm.swappiness=150' > /etc/sysctl.d/99-smth.conf
useradd -m -G video,input,render,audio smth || true
EOF
chmod +x config/hooks/normal/9000-smth.hook.chroot

echo "[*] Building live ISO image..."
sudo lb build

# 6. Move generated ISO to root workspace for GitHub artifact upload
if ls live-image-amd64.hybrid.iso 1> /dev/null 2>&1; then
  cp live-image-amd64.hybrid.iso ../smth-os-amd64.iso
  echo "[+] Success! ISO generated: smth-os-amd64.iso"
else
  echo "[-] Build failed: ISO image not found."
  exit 1
fi
