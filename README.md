<p align="center"><img src="https://github.com/arch-based/fetcher-cpp/blob/main/fetcher.png?raw=true"></p>
<h1 align="center">fetcher-rs</h1>
<p align="center">A rust implementation of the fetcher C++ tool</p><br>

# Description

Rust is a new programming language that serves as a replacement for C++ because of its vulnerabilities, it serves to become easier than C++ and also memory safe.

This rust re-implementation of fetcher provides secure code and an immensely fast implementation of neofetch after its discontinuation. There were several alternatives such as fastfetch and hyfetch.
 
Fastfetch is written in C, while hyfetch is in Python which is just slightly faster than Bash. We serve to beat fastfetch in Arch Linux in terms of speed and security. Because hyfetch is written in Python, it is said to be quite slow. 

The old implementation of this script in C++ is faster than both fastfetch and hyfetch. But since C++ has more vulnerabilities, I decided to use Rust. This github repo serve tarballs for the AUR, and the main resources, in order for it to compile. I also serve the process of compiling in the AUR using the fetcher-based-rs package in the A.U.R.

_**This project is done for now as Rust can finish projects fast, while in other programming languages it requires a lot of time to finish one project. While in rust, i'm done! :)**_

# Installation

Before installing & compiling, make sure to install the proper dependencies such as rust/rustup. But since, the A.U.R package installs the dependencies you may not need to do this.
```
--  Using rust  --
# pacman -S wget
# pacman -S rust

--  Using rustup  --
# pacman -S rustup
$ rustup default stable # This is used when you have an error in compiling with rustup.
```

Since we aren't offering a binary in this Github Repo, we will be giving you the source code instead of the binary.
```
$ git clone https://github.com/arch-based/fetcher-rs
$ cd fetcher-rs/src
$ cargo build
```

To install the source code and build it using the A.U.R repo, use any aur-helper using paru or yay.
```
-- For paru:
$ paru -S fetcher-based-rs

-- For yay:
$ yay -S fetcher-based-rs
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

$ paru -S fetcher-based-rs-bin
# For yay:

$ yay -S fetcher-based-rs-bin
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
$ wget https://github.com/arch-based/fetcher-rs/raw/main/fetcher-rs-bin.tar.gz
$ tar xvf fetcher-rs-bin.tar.gz
$ cp -r fetcher /usr/bin
# Usage
```

You can use it with the command: fetcher which is inside of /usr/bin.
```
fetcher
```
