import {readInput} from 'io'
import {min} from "utils";

const input = await readInput('day-17')

export const part1 = () => {
  let [, a, b, c, instructions] = input.match(/Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)\n\nProgram: ([\d|,]+)/)!

  return program(BigInt(a), BigInt(b), BigInt(c), instructions.split(',').map(Number)).join(',')
}

export const part2 = () => {
  let [, , , , ins] = input.match(/Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)\n\nProgram: ([\d|,]+)/)!
  let instructions = ins.split(',').map(Number)

  return min(solve(0n, instructions.length - 1, instructions))
}

const solve = (a: bigint, index: number, instructions: number[]) => {
  let possible: bigint[] = [];

  for (let i = 0n; i < 8; i++) {
    let out = program(a + i, 0n, 0n, instructions)
    if (out[0] == BigInt(instructions[index])) {
      possible = possible.concat(index == 0 ? [a + i] : solve((a + i) * 8n, index - 1, instructions))
    }
  }

  return possible;
}


const program = (a: bigint, b: bigint, c: bigint, instructions: number[]) => {
  let out = []

  for (let ptr = 0; ptr < instructions.length; ptr += 2) {
    let [instruction, literal] = [instructions[ptr], instructions[ptr + 1]]
    const combo = () => literal <= 3 ? BigInt(literal) : literal == 4 ? a : literal == 5 ? b : c;
    switch (instruction) {
      case 0:
        a = a / 2n ** combo()
        break
      case 1:
        b ^= BigInt(literal)
        break
      case 2:
        b = combo() % 8n
        break
      case 3:
        if (a != 0n) {
          ptr = literal - 2
          continue;
        }
        break;
      case 4:
        b ^= c
        break;
      case 5:
        out.push(combo() % 8n)
        break;
      case 6:
        b = a / 2n ** combo()
        break
      case 7:
        c = a / 2n ** combo()
        break
    }
  }

  return out;
}
