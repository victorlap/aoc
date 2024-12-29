import {parseGroups, readInput} from 'io'
import {desc, pluck} from "utils";

const input = await readInput('day-24')

export const part1 = () => {
  const groups = parseGroups(input)
  const wires = pluck(groups[0].map(l => l.split(': ')), (l) => l[0], (l) => Number(l[1]))
  const instructions = groups[1].map(l => l.split(' '))

  const result = runProgram(wires, instructions)

  return parseInt(toDecimal(result), 2)
}

export const part2 = () => {
  const groups = parseGroups(input)
  const instructions = groups[1].map(l => l.split(' '))

  const suspicious = new Set<string[]>;

  for (const instruction of instructions) {
    if (instruction[4][0] == "z" && instruction[1] != "XOR" && instruction[4] !== "z45") {
      suspicious.add(instruction)
    }
    if (instruction[1] == "XOR" && !["x", "y"].includes(instruction[0][0]) && !["x", "y"].includes(instruction[2][0]) && instruction[4][0] !== "z") {
      suspicious.add(instruction)
    }
    if (instruction[1] == "XOR" && ["x", "y"].includes(instruction[0][0]) && ["x", "y"].includes(instruction[2][0]) &&
      !instructions.some(ins => (ins[0] == instruction[4] || ins[2] == instruction[4]) && ins[1] == "XOR") &&
      instruction[4] !== "z00"
    ) {
      suspicious.add(instruction)
    }
    if (instruction[1] == "AND" && ["x", "y"].includes(instruction[0][0]) && ["x", "y"].includes(instruction[2][0]) &&
      !instructions.some(ins => (ins[0] == instruction[4] || ins[2] == instruction[4]) && ins[1] == "OR") &&
      instruction[0].slice(1) !== "00"
    ) {
      suspicious.add(instruction)
    }
  }

  return suspicious.values().map(s => s[4]).toArray().toSorted().join(',')
}

const toDecimal = (wires: Record<string, number>, name = "z") => {
  return Object.keys(wires)
    .filter(key => key.startsWith(name))
    .toSorted(desc)
    .reduce((acc, cur) => acc + wires[cur], "")
}

const runProgram = (wires: Record<string, number>, instructions: string[][]) => {
  while (instructions.length) {
    let instruction = instructions.shift()!
    let [a, op, b, , res] = instruction
    if (wires[a] == undefined || wires[b] == undefined) {
      instructions.push(instruction)
      continue;
    }

    if (op == "AND") {
      wires[res] = wires[a] & wires[b]
    } else if (op == "OR") {
      wires[res] = wires[a] | wires[b]
    } else if (op == "XOR") {
      wires[res] = wires[a] ^ wires[b]
    }
  }
  return wires
}
