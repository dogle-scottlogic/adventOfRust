# Advent Of Rust

## Overview

This project has been created to test integrating a WASM package built in RUST with a Next JS application.

`advent_of_rust` is written in Rust and solves days one and two of the 2016 Advent of Code challenges. https://adventofcode.com/2016.
This lib then exposes four functions that can be called from JavaScript, one to solve each part of each day.
The WASM package is pushed to NPM https://www.npmjs.com/package/advent_of_rust

`advent_of_rust_js` is a NextJs app which consumes the advent_of_rust lib and uses it to allow a user to solve days one and two.

## Build

To build the wasm, navigate to `advent_of_rust` and run 
```
wasm-pack build
```

## Publish

To publish, navigate to `advent_of_rust` and run
```
wasm-pack login
wasm-pack build
wasm-pack publish
```
*Note: Ensure the version is set to a higher version number than the current published version.

## Debug

To debug the Rust code you must update from a lib to a binary by adding a `main.rs` file in place of the `lib.rs`.

## NextJs

To run the NextJS app navigate to `advent_of_rust_js` and run
```
npm run dev 
```