import {parseGroups, readInput} from 'io'
import {sum} from "utils";

const input = await readInput('day-19')
const cache: Record<string, number> = {}

export const part1 = () => {
  const [[availableList], wanted] = parseGroups(input)
  const available = availableList.split(', ').toSorted((a, b) => b.length - a.length)

  return wanted.filter(t => isPossible(t, available) > 0).length;
}

export const part2 = () => {
  const [[availableList], wanted] = parseGroups(input)
  const available = availableList.split(', ').toSorted((a, b) => b.length - a.length)

  return sum(wanted.map(t => isPossible(t, available)))
}

const isPossible = (towel: string, available: string[]) => {
  if (cache[towel] !== undefined) return cache[towel];

  let sum = 0;

  for (const t of available) {
    if (towel === t) {
      sum += 1
      continue
    }
    if (towel.startsWith(t)) {
      sum += isPossible(towel.substring(t.length), available);
    }
  }


  cache[towel] = sum;
  return sum;
}
