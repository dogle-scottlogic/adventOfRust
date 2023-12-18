"use client"
import { ChangeEvent, useEffect, useState } from 'react';
import styles from "./page.module.css";

interface Module {
  default: typeof import("c:/dev/adventOfRust/advent_of_rust_js/node_modules/advent_of_rust/advent_of_rust");
  day_one_part_one(input: string): number;
  day_one_part_two(input: string): number;
};

const Home = () => {
  const [partOne, setPartOne] = useState<number | null>(null);
  const [partTwo, setPartTwo] = useState<number | null>(null);
  const [text, setText] = useState<string>("");
  const [module, setModule] = useState<Module | null>(null);
  useEffect(() => {
    // Load and instantiate the Wasm module from the CDN
    const wasmModule = import('advent_of_rust');

    wasmModule.then((module) => {
      // Use exported functions from the Wasm module
      setModule(module);
    });
  }, []);

  const solveDayOne = () => {
    if (module != null) {
      setPartOne(module.day_one_part_one(text));
    }
  }

  const solveDayTwo = () => {
    if (module != null) {
      setPartTwo(module.day_one_part_two(text));
    }
  }

  const showFile = async (e: ChangeEvent<HTMLInputElement>) => {
    e.preventDefault()
    const reader = new FileReader()
    reader.onload = async (e) => {
      const text = (e.target && e.target.result)
      if (text != null) {
        setText(text as string);
      }
    };
    if (e.target.files?.length) {
      reader.readAsText(e.target.files[0])
    }
  }

  return (
    <div className={styles.aoc}>
      <h1>Advent of Code 2016</h1>
      <h2>Day One</h2>

      <div className={styles.wrapper}>
        <div className={styles.options}>
          <div className="daySelector">
            <input className={styles.part} type="file" onChange={showFile} />
          </div>
          {text && (<>
            <div className={styles.part}>
              {partOne ? <label className={styles.label}>{partOne}</label> : <button className={styles.button} onClick={solveDayOne}>Solve Day One Part One</button>}
            </div>

            <div className={styles.part}>
              {partTwo ? <label className={styles.label}>{partTwo}</label> : <button className={styles.button} onClick={solveDayTwo}>Solve Day One Part Two</button>}
            </div></>)
          }
        </div>
        <div className={styles.blurb}>
          <p>Advent of code 2016, Day One.</p>
          <p>This Webpage is rendered using NextJS.</p>
          <p>The chosen file should be a .txt file containing a valid input for <a href='https://adventofcode.com/2016/day/1/input'>Day One.</a></p>
          <p>Once provided the app will provide the ability to solve both parts of the puzzle.</p>
          <p>The logic to solve the puzzle is written in Rust and uses Web Assembly to interface with JavaScript</p>
        </div>
      </div>
      <div className={styles.footer}><small>A bench project by <a href='https://github.com/dogle-scottlogic'>dogle-scottlogic</a></small></div>
    </div>
  );
};

export default Home;