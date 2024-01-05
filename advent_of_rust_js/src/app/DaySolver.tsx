import { useState } from 'react';
import styles from "./page.module.css";
import { PuzzleSolverFunction } from './page';
import Spinner from './Spinner';

interface DayProps {
    solvePartOne?: PuzzleSolverFunction;
    solvePartTwo?: PuzzleSolverFunction;
    setPartOne: (value: string | null) => void;
    partOne: string | null;
    setPartTwo: (value: string | null) => void;
    partTwo: string | null;
    input: string;
}


const Day = (props: DayProps) => {
    const { solvePartOne, solvePartTwo, setPartOne, setPartTwo, partOne, partTwo, input } = props;
    const [error, setError] = useState<string>("");
    const [loadingPartOne, setLoadingPartOne] = useState<boolean>(false);

    const solve = (part: PuzzleSolverFunction) => {
        try {
            return part(input);
        } catch (e) {
            console.log(e);
            const error: any = e;
            setError(`Error: ${error.name}, ${error.message}`);
        }
        return null;
    }

    const getPartOne = () => {
        setLoadingPartOne(true);
        if (module != null && solvePartOne) {
            setPartOne(solve(solvePartOne));
        }
        setLoadingPartOne(false);
    }

    const getPartTwo = () => {
        if (module != null && solvePartTwo) {
            setPartTwo(solve(solvePartTwo));
        }
    }
    return (
        <>
            {solvePartOne && solvePartTwo && input && (<>
                <div className={styles.error}>{error}</div>
                {loadingPartOne ? <Spinner /> : <div>
                    {partOne ? <label>{partOne}</label> : <button className={styles.button} onClick={getPartOne}>[ Solve Part One ]</button>}
                </div>}

                <div>
                    {partTwo ? <label>{partTwo}</label> : <button className={styles.button} onClick={getPartTwo}>[ Solve Part Two ]</button>}
                </div></>)
            }
        </>
    )
}

export default Day;