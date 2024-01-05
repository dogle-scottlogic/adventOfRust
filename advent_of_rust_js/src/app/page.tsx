"use client"
import { ChangeEvent, useEffect, useState } from 'react';
import styles from "./page.module.css";
import Nav from './Nav';
import React from 'react';
import Day from './DaySolver';
import { convertModule } from './page.utils';

const dayList = ["Day One", "Day Two", "Day Three", "Day Four", "Day Five"];
export type PuzzleDay = typeof dayList[number];
export type PuzzleSolverFunction = (input: string) => string;

interface Module { [key: PuzzleDay]: { solvePartOne: PuzzleSolverFunction, solvePartTwo: PuzzleSolverFunction } };

const Home = () => {
  let fileInput: React.RefObject<HTMLInputElement> = React.createRef();;
  const [input, setInput] = useState<string>("");
  const [module, setModule] = useState<Module | null>(null);
  const [currentDay, setCurrentDay] = useState<PuzzleDay>("Day One");
  const [partOne, setPartOne] = useState<string | null>(null);
  const [partTwo, setPartTwo] = useState<string | null>(null);

  useEffect(() => {
    const wasmModule = convertModule();
    wasmModule.then((module) => {
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

  const getSolvers = (): { solvePartOne?: PuzzleSolverFunction, solvePartTwo?: PuzzleSolverFunction } => {
    return module ? module[currentDay] : { solvePartOne: undefined, solvePartTwo: undefined };
  }

  const dayProps = {
    input,
    setPartOne,
    setPartTwo,
    partOne,
    partTwo,
    ...getSolvers()
  };

  return (
    <div className={styles.aoc}>
      <div className={styles.header}>
        <h1>Advent of Code 2016</h1>
        <h2>{currentDay}</h2>
        <Nav items={dayList} onClick={navClicked} />
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
          <Day
            {...dayProps}
          />
          <button className={styles.button} onClick={clearClicked}>[ Clear ]</button>
        </div>
      </div>
      <div className={styles.footer}><small>A bench project by <a href='https://github.com/dogle-scottlogic'>@dogle-scottlogic</a></small></div>
    </div>
  );
};

export default Home;