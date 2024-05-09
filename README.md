<p align="center"><img src="https://github.com/arch-based/fetcher-cpp/blob/main/fetcher.png?raw=true"></p>
<h1 align="center">fetcher-rs</h1>
<p align="center">A rust implementation of the fetcher C++ tool</p><br>

# Description

This rust implementation of fetcher  provides secure code and an immensely fast implementation of neofetch after its discontinuation. There were several alternatives such as fastfetch and hyfetch. Fastfetch is written in C, while hyfetch is in Python which is just slightly faster than Bash.

We serve to beat fastfetch in Arch Linux in terms of speed and security. Because hyfetch is written in Python, it is said to be quite slow. The old implementation of this script in C++ is faster than both fastfetch and hyfetch. But since C++ has more vulnerabilities we serve binaries for the AUR, but this github repo only has the main resources in order for it to compile.

We also serve the process of compiling in the AUR to see how rust works.

# Installation

Before installing & compiling, make sure to install the proper dependencies such as rust/rustup.
```
$ sudo pacman -S rust
# Using rustup:
$ sudo pacman -S rustup
$ rustup default stable
```
Since we aren't offering a binary in this Github Repo, we will be giving you the source code instead of the binary.
```
$ git clone https://github.com/arch-based/fetcher-rs
$ cd fetcher-rs/src
$ cargo build
```
To install the source code and build it using the A.U.R repo, use any aur-helper using paru or yay.

```
# For paru:
$ paru -S fetcher-rs

# For yay:
$ yay -S fetcher-rs
```

If you'd like to use the A.U.R repo manually, use this method:
```
$ git clone https://aur.archlinux.org/fetcher-based.git ~/fetcher-based
$ cd fetcher-based
$ makepkg -si 
```
To build the binary with the A.U.R repo, use any aur helper such as yay or paru:
```
# For paru:

$ paru -S fetcher-rs-bin
# For yay:

$ yay -S fetcher-rs-bin
```
If you want to use the tarballs instead of the A.U.R repo and the github repo. You can do that by:
```
$ sudo pacman -S wget
$ wget https://github.com/arch-based/fetcher-rs/raw/main/fetcher-rs.tar.gz
$ tar xvf fetcher-rs.tar.gz
$ cd src
$ cargo build
$ cp -r ../target/debug/fetcher /usr/bin
```
If you want to use the binary tarball:
```
$ sudo pacman -S wget
$ wget https://github.com/arch-based/fetcher-rs/raw/main/fetcher-rs-bin.tar.gz
$ tar xvf fetcher-rs-bin.tar.gz
$ cp -r fetcher /usr/bin
# Usage

You can use it with the command: fetcher which is inside of /usr/bin.
```
fetcher
```
