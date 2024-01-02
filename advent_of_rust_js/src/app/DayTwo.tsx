import Day, { CommonDayProps } from "./DaySolver";

interface DayTwoProps extends CommonDayProps {
    input: string;
    part_one?: (input: string[]) => string;
    part_two?: (input: string[]) => string;
}

const DayTwo = (props: DayTwoProps) => {
    const { part_one, part_two, input } = props;

    const arrayInput = props.input.split("");

    return (
        <>
            {part_one && part_two && input && <Day solvePartOne={() => part_one(arrayInput)} solvePartTwo={() => part_two(arrayInput)} {...props} />}
        </>
    )
}

export default DayTwo;