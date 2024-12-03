import {parseLines, readInput} from 'io'
import {mul, sum} from "utils";

const input = await readInput('day-03')

export const part1 = () => {
  const lines = parseLines(input)

  let answer = 0;

  for (const line of lines) {
    const instructions = line.matchAll(/mul\((\d+,\d+)\)/g).map(v => v[1].split(',').map(Number)).toArray();

    answer += sum(instructions.map(mul))
  }

  return answer
}

export const part2 = () => {
  const lines = parseLines(input)

  let answer = 0;
  let include = true;

  for (const line of lines) {
    const instructions = line.matchAll(/(do\(\)|don't\(\)|mul\((\d+,\d+)\))/g).toArray()

    for (const instruction of instructions) {
      switch (instruction[0].substring(0, 3)) {
        case 'do(':
          include = true;
          break;
        case 'don':
          include = false;
          break;
        case 'mul':
          if (include) {
            answer += mul(instruction[2].split(',').map(Number))
          }
          break;
      }
    }
  }

  return answer
}
