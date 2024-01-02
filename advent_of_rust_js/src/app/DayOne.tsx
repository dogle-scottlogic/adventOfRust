import Day, { CommonDayProps } from "./DaySolver";

interface DayOneProps extends CommonDayProps {
    input: string;
    part_one?: (input: string) => number;
    part_two?: (input: string) => number;
}

const DayOne = (props: DayOneProps) => {
    const { part_one, part_two, input } = props;

    return (
        <>
            {part_one && part_two && input && <Day solvePartOne={() => part_one(input)} solvePartTwo={() => part_two(input)} {...props} />}
        </>
    )
}

export default DayOne;