import styles from "./daySolver.module.css";
import { useState } from 'react';

export interface CommonDayProps {
    setPartOne: (value: string | number | null) => void;
    partOne: string | number | null;
    setPartTwo: (value: string | number | null) => void;
    partTwo: string | number | null;
};

interface DayProps extends CommonDayProps {
    solvePartOne: () => number | string;
    solvePartTwo: () => number | string;
}


const Day = (props: DayProps) => {
    const { solvePartOne, solvePartTwo, setPartOne, setPartTwo, partOne, partTwo } = props;
    const [error, setError] = useState<string>("");

    const solve = (part: () => number | string) => {
        try {
            return part();
        } catch (e) {
            console.log(e);
            const error: any = e;
            setError(`Error: ${error.name}, ${error.message}`);
        }
        return null;
    }

    const getPartOne = () => {
        if (module != null && solvePartOne) {
            setPartOne(solve(solvePartOne));
        }
    }

    const getPartTwo = () => {
        if (module != null && solvePartTwo) {
            setPartTwo(solve(solvePartTwo));
        }
    }

    return (
        <>
            {(<>
            <p>{error}</p>
                <div className={styles.part}>
                    {partOne ? <label className={styles.label}>{partOne}</label> : <button className={styles.button} onClick={getPartOne}>Solve Part One</button>}
                </div>

                <div className={styles.part}>
                    {partTwo ? <label className={styles.label}>{partTwo}</label> : <button className={styles.button} onClick={getPartTwo}>Solve Part Two</button>}
                </div></>)
            }
        </>
    )
}

export default Day;