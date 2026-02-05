
## msys64

https://www.msys2.org/

1. Download and install: [msys2-x86_64-20210604.exe](https://repo.msys2.org/distrib/x86_64/msys2-x86_64-20210604.exe)
2. place *.crt in /etc/pki/ca-trust/source/anchors and run update-ca-trust
3. or C:\msys64\usr\ssl\certs\ca-bundle.trust.crt, ca-bundle.crt 에 추가
4. Update the package database and base packages. 
   1. pacman -Syu -y
   2. pacman -Su -y
   3. pacman -S --needed base-devel mingw-w64-x86_64-toolchain
   4. pacman -S mingw-w64-x86_64-rust -y
   5. pacman -S git -y

## build in msys64

git clone img_caster.git

cd img_caster/

cargo build --release

