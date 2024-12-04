import {parseLines, readInput} from 'io'

const input = await readInput('day-04')

export const part1 = () => {
  const lines = parseLines(input)
  let count = 0;

  for (let i = 0; i < lines.length; i++) {
    for (let j = 0; j < lines[i].length; j++) {
      // horizontal
      if (lines[i].substring(j, j + 4) === "XMAS" || lines[i].substring(j, j + 4) === "SAMX") {
        count++;
      }
      // down
      if (lines[i][j] === "X" && lines?.[i + 1]?.[j] === "M" && lines?.[i+2]?.[j] === "A" && lines?.[i+3]?.[j] === "S") {
        count++
      }
      // down reverse
      if (lines[i][j] === "S" && lines?.[i+1]?.[j] === "A" && lines?.[i + 2]?.[j] === "M" && lines?.[i + 3]?.[j] === "X") {
        count++
      }
      // down left across
      if (lines[i][j] === "X" && lines?.[i - 1]?.[j + 1] === "M" && lines?.[i - 2]?.[j + 2] === "A" && lines?.[i - 3]?.[j + 3] === "S") {
        count++
      }
      // down left across reverse
      if (lines[i][j] === "S" && lines?.[i - 1]?.[j + 1] === "A" && lines?.[i - 2]?.[j + 2] === "M" && lines?.[i - 3]?.[j + 3] === "X") {
        count++
      }
      // down right across
      if (lines[i][j] === "X" && lines?.[i + 1]?.[j + 1] === "M" && lines?.[i + 2]?.[j + 2] === "A" && lines?.[i + 3]?.[j + 3] === "S") {
        count++
      }
      // down right across reverse
      if (lines[i][j] === "S" && lines?.[i + 1]?.[j + 1] === "A" && lines?.[i + 2]?.[j + 2] === "M" && lines?.[i + 3]?.[j + 3] === "X") {
        count++
      }
    }
  }

  return count
}

