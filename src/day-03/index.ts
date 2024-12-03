import { parseLines, readInput } from 'io'
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
