"use client"
import { ChangeEvent, useEffect, useState } from 'react';
import styles from "./page.module.css";
import Nav from './Nav';
import DayOne from './DayOne';
import DayTwo from './DayTwo';
import React from 'react';

export type PuzzleDay = "Day One" | "Day Two";

interface Module {
  day_one_part_one(input: string): number;
  day_one_part_two(input: string): number;
  day_two_part_one(input: string[]): string;
  day_two_part_two(input: string[]): string;
};

const Home = () => {
  let fileInput: React.RefObject<HTMLInputElement> = React.createRef();;
  const [input, setInput] = useState<string>("");
  const [module, setModule] = useState<Module | null>(null);
  const [currentDay, setCurrentDay] = useState<PuzzleDay>("Day One");
  const [partOne, setPartOne] = useState<string | number | null>(null);
  const [partTwo, setPartTwo] = useState<string | number | null>(null);

  useEffect(() => {
    // Load and instantiate the Wasm module from the CDN
    const wasmModule = import('advent_of_rust');

    wasmModule.then((module) => {
      // Use exported functions from the Wasm module
      setModule(module);
    });
  }, []);

  const showFile = async (e: ChangeEvent<HTMLInputElement>) => {
    e.preventDefault()
    const reader = new FileReader()
    reader.onload = async (e) => {
      const text = (e.target && e.target.result)
      if (text != null) {
        setInput(text as string);
      }
    };
    if (e.target.files?.length) {
      reader.readAsText(e.target.files[0])
    }
  }

  const navClicked = (day: PuzzleDay) => {
    setCurrentDay(day);
    setPartOne(null);
    setPartTwo(null);
  }

  const clearClicked = () => {
    if (fileInput?.current?.value) {
      fileInput.current.value = "";
      setInput("");
      setPartOne(null);
      setPartTwo(null);
    }
  }

  return (
    <div className={styles.aoc}>
      <div className={styles.header}>
        <h1>Advent of Code 2016</h1>
        <h2>{currentDay}</h2>
        <Nav items={["Day One", "Day Two"]} onClick={navClicked} />
      </div>
      <div className={styles.wrapper}>
        <div className={styles.blurb}>
          <p>Advent of code 2016, {currentDay}</p>
          < br />
          <p>This Webpage is rendered using NextJS.</p>
          <p>The chosen file should be a .txt file containing a valid input for <a href='https://adventofcode.com/2016/day/1/input'>Day One.</a></p>
          <p>Once provided the app will provide the ability to solve both parts of the puzzle.</p>
          <p>The logic to solve the puzzle is written in Rust and uses Web Assembly to interface with JavaScript</p>
        </div>
        <br />
        <div className={styles.options}>
          <label className={styles.button}>
            [ Select File ]
            <input ref={fileInput} className={styles.part} type="file" onChange={showFile} />
          </label>
          {
            currentDay === "Day One" ? (
              <DayOne
                part_one={module?.day_one_part_one}
                part_two={module?.day_one_part_two}
                input={input}
                setPartOne={setPartOne}
                setPartTwo={setPartTwo}
                partOne={partOne}
                partTwo={partTwo}
              />
            ) : (
              <DayTwo
                part_one={module?.day_two_part_one}
                part_two={module?.day_two_part_two}
                input={input}
                setPartOne={setPartOne}
                setPartTwo={setPartTwo}
                partOne={partOne}
                partTwo={partTwo}
              />
            )
          }
          <button className={styles.button} onClick={clearClicked}>[ Clear ]</button>
        </div>
      </div>
      <div className={styles.footer}><small>A bench project by <a href='https://github.com/dogle-scottlogic'>@dogle-scottlogic</a></small></div>
    </div>
  );
};

export default Home;