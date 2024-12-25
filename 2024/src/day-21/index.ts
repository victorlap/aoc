import {parseLines, readInput} from 'io'
import {Direction, Grid, Point} from "utils/grid.ts";
import {min, orderedMultiSubsets} from "utils";

const input = await readInput('day-21')
const possibleDirections: Record<string, Direction> = {'v': [1, 0], '^': [-1, 0], '>': [0, 1], '<': [0, -1]}

export const part1 = async () => {
  const lines = parseLines(input)
  const numericPad = Grid.from([["7", "8", "9"], ["4", "5", "6"], ["1", "2", "3"], [" ", "0", "A"]])
  const directPad = Grid.from([[" ", "^", "A"], ["<", "v", ">"]])

  let sum = 0;
  let numericPaths = getAllPaths(numericPad);
  let directPaths = getAllPaths(directPad);

  const cache: Record<number, Record<string, number>> = {};
  for (let i = 1; i <= 2; i++) {
    cache[i] = {};
  }

  const shortest = (path: string, depth: number) => {
    if (depth == 0) return path.length
    if (cache[depth][path]) return cache[depth][path]

    const fullPath = "A" + path
    let total = 0;
    for (let i = 0; i < fullPath.length - 1; i++) {
      const paths = directPaths[fullPath[i] + fullPath[i + 1]];
      total += Math.min(...paths.map(p => shortest(p, depth - 1)));
    }

    cache[depth][path] = total;
    return total;
  }

  for (const line of lines) {
    const fullPath = "A" + line
    let total = 0;

    for (let i = 0; i < fullPath.length - 1; i++) {
      const paths = numericPaths[fullPath[i] + fullPath[i + 1]];
      total += Math.min(...paths.map(p => shortest(p, 2)));
    }

    sum += parseInt(line) * total
  }

  return sum
}

export const part2 = async () => {
  const lines = parseLines(input)
  const numericPad = Grid.from([["7", "8", "9"], ["4", "5", "6"], ["1", "2", "3"], [" ", "0", "A"]])
  const directPad = Grid.from([[" ", "^", "A"], ["<", "v", ">"]])

  let sum = 0;
  let numericPaths = getAllPaths(numericPad);
  let directPaths = getAllPaths(directPad);

  const cache: Record<number, Record<string, number>> = {};
  for (let i = 1; i <= 25; i++) {
    cache[i] = {};
  }

  const shortest = (path: string, depth: number) => {
    if (depth == 0) return path.length
    if (cache[depth][path]) return cache[depth][path]

    const fullPath = "A" + path
    let total = 0;
    for (let i = 0; i < fullPath.length - 1; i++) {
      const paths = directPaths[fullPath[i] + fullPath[i + 1]];
      total += Math.min(...paths.map(p => shortest(p, depth - 1)));
    }

    cache[depth][path] = total;
    return total;
  }

  for (const line of lines) {
    const fullPath = "A" + line
    let total = 0;

    for (let i = 0; i < fullPath.length - 1; i++) {
      const paths = numericPaths[fullPath[i] + fullPath[i + 1]];
      total += Math.min(...paths.map(p => shortest(p, 25)));
    }

    sum += parseInt(line) * total
  }

  return sum
}

function getAllPaths(pad: Grid<string>) {
  return Object.fromEntries(orderedMultiSubsets(new Set(pad.allPoints().map(p => p.value).filter(Boolean)), 2).map(([a, b]) => {
    return [
      a + b,
      findShortestPath(b, pad.allPoints().find(p => p.value == a)!, pad, new Set)
    ]
  }));
}

const findShortestPath = (input: string, pos: Point<string>, grid: Grid<string>, visited: Set<Point<string>>): string[] => {
  visited.add(pos)
  let paths: string[] = []

  if (input == "") {
    return ["A"]
  }

  if (input[0] == pos.value) {
    return findShortestPath(input.substring(1), pos, grid, new Set).map(p => p)
  }

  for (const [p, dir] of Object.entries(possibleDirections)) {
    let next = grid.move(pos, dir)
    if (!next || next.value == " " || visited.has(next)) continue;

    let newPath = next.value == input[0] ? p + "A" : p;
    if (next.value == input[0] && input.length == 1) {
      paths.push(newPath)
      continue;
    }

    let rest = next.value == input[0]
      ? findShortestPath(input.substring(1), next, grid, new Set)
      : findShortestPath(input, next, grid, new Set(visited))

    for (const path of rest) {
      paths.push(newPath + path)
    }
  }

  let shortest = min(paths.map(p => p.length))

  return paths.filter(p => p.length == shortest);
}
