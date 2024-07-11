#!/usr/bin/env sh

set -e

echo "To fill the template tell me your egui project crate name: "

read crate

echo "To fill the template tell me your name (for author in Cargo.toml): "

read name

echo "To fill the template tell me your e-mail address (also for Cargo.toml): "

read email

echo "Patching files..."

sed -i "s/fahrenheit-to-celsuis-through-fibo/$crate/g" Cargo.toml
sed -i "s/fahrenheit-to-celsuis-through-fibo/$crate/g" src/main.rs
sed -i "s/eframe template/$crate/g" index.html
sed -i "s/fahrenheit-to-celsuis-through-fibo/$crate/g" assets/sw.js
sed -i "s/Emil Ernerfeldt/$name/g" Cargo.toml
sed -i "s/emil.ernerfeldt@gmail.com/$email/g" Cargo.toml

echo "Done."

