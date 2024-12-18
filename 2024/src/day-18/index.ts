import {parseLines, readInput} from 'io'
import {Direction, Grid, Point} from "utils/grid.ts";
import {PriorityQueue} from "utils/queue.ts";

const input = await readInput('day-18')
const possibleDirections: Direction[] = [[1, 0], [-1, 0], [0, 1], [0, -1]]

export const part1 = () => {
  const lines = parseLines(input)
  // const [width, height, length] = [7, 7, 12];
  const [width, height, length] = [71, 71, 1024];
  const grid = Grid.new(height, width, () => '.')

  for (let i = 0; i < length; i++) {
    const [x, y] = lines[i].split(',').map(Number)
    grid.set(y, x, '#')
  }

  const startPos = grid.get(0, 0)
  const endPos = grid.get(height - 1, width - 1)

  return solve(grid, startPos, endPos);
}

export const part2 = () => {
  const lines = parseLines(input)
  // const [width, height, length] = [7, 7, 12];
  const [width, height, length] = [71, 71, 1024];
  const grid = Grid.new(height, width, () => '.')

  for (let i = 0; i < length; i++) {
    const [x, y] = lines[i].split(',').map(Number)
    grid.set(y, x, '#')
  }

  const startPos = grid.get(0, 0)
  const endPos = grid.get(height - 1, width - 1)

  let i = length
  while (true) {
    const [x, y] = lines[i].split(',').map(Number)
    grid.set(y, x, '#')
    i++

    if (!solve(grid, startPos, endPos)) {
      return `${x},${y}`;
    }
  }
}

function solve(grid: Grid<string>, startPos: Point<string>, endPos: Point<string>) {
  let costMap = new Map<Point<string>, number>
  let queue = new PriorityQueue<Point<string>>
  queue.enqueue(startPos, 0)

  while (!queue.isEmpty()) {
    const {element: point, priority: cost} = queue.dequeue()!

    for (const dir of possibleDirections) {
      let next = grid.move(point, dir)
      if (next?.value !== ".") continue;
      if (cost + 1 >= (costMap.get(next) || Infinity)) continue;
      costMap.set(next, cost + 1)
      queue.enqueue(next, cost + 1)
    }
  }

  return costMap.get(endPos)
}
