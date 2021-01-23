## paste-rs 
[![Build Status](https://travis-ci.org/Indiv0/paste-rs.svg?branch=master)](https://travis-ci.org/Indiv0/paste-rs)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

### Setup
Setup Application:

```sh
# install upx and GNU make and git arch linux
sudo pacman -S make upx git
# install rust-lang
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# clone project
git clone github.com/robatipoor/paste-rs
# build and install
cd paste-rs
make
# copy default config 
cp config.toml ~/.config/paste-rs/config.toml
# install sqlx-cli
cargo install --version=0.2.0 sqlx-cli
# create database
export DATABASE_URL="mysql://username:password@localhost:3306/paste_rs"
sqlx database create
# run migration
sqlx migrate run
# show info logs in stdout
export RUST_LOG=info
# run app
paste

```
### client tools
for client side you can use [pf](https://github.com/robatipoor/pf) 
