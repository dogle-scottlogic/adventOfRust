export const convertModule = async (): Promise<{
  [key: string]: { solvePartOne: any; solvePartTwo: any };
}> => {
  // Load and instantiate the Wasm module from the CDN
  const wasmModule = import("advent_of_rust");

  let module = await wasmModule;
  return {
    "Day One": {
      solvePartOne: module?.day_one_part_one,
      solvePartTwo: module?.day_one_part_two,
    },
    "Day Two": {
      solvePartOne: module?.day_two_part_one,
      solvePartTwo: module?.day_two_part_two,
    },
    "Day Three": {
      solvePartOne: module?.day_three_part_one,
      solvePartTwo: module?.day_three_part_two,
    },
    "Day Four": {
      solvePartOne: module?.day_four_part_one,
      solvePartTwo: module?.day_four_part_two,
    },
    "Day Five": {
      solvePartOne: module?.day_five_part_one,
      solvePartTwo: module?.day_five_part_two,
    },
  };
};
