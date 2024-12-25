import {parseGrid, readInput} from 'io'
import {Direction, Grid, Point} from "utils/grid.ts";
import {PriorityQueue} from "utils/queue.ts";

const input = await readInput('day-20')
const possibleDirections: Direction[] = [[1, 0], [-1, 0], [0, 1], [0, -1]]

export const part1 = () => {
  const grid = parseGrid(input)

  const startPos = grid.allPoints().find(p => p.value == "S")!;
  const endPos = grid.allPoints().find(p => p.value == "E")!;

  let path = findPath(grid, startPos, endPos)
  let cheats = findCheats(path, 2);

  return cheats.filter(c => c >= 100).length;
}

export const part2 = () => {
  const grid = parseGrid(input)

  const startPos = grid.allPoints().find(p => p.value == "S")!;
  const endPos = grid.allPoints().find(p => p.value == "E")!;

  let path = findPath(grid, startPos, endPos)
  let cheats = findCheats(path, 20);

  return cheats.filter(c => c >= 100).length;
}

const findPath = (grid: Grid<string>, startPos: Point<string>, endPos: Point<string>) => {
  let costMap = new Map<Point<string>, number>
  let queue = new PriorityQueue<Point<string>>
  costMap.set(startPos, 0)
  queue.enqueue(startPos, 0)

  while (!queue.isEmpty()) {
    const {element: point, priority: cost} = queue.dequeue()!

    for (const dir of possibleDirections) {
      let next = grid.move(point, dir)
      if (next?.value !== "." && next?.value !== "E") continue;
      if (cost + 1 >= (costMap.get(next) || Infinity)) continue;
      costMap.set(next, cost + 1)
      queue.enqueue(next, cost + 1)
    }
  }

  return costMap.keys().toArray()
}

const findCheats = (path: Point<string>[], limit: number) => {
  let cheats = [];

  for (let i = 0; i < path.length; i++) {
    for (let j = i + 1; j < path.length; j++) {
      let dist = Math.abs(path[j].y - path[i].y) + Math.abs(path[j].x - path[i].x)
      let length = j - i - dist;
      if (dist <= limit && length > 0) {
        cheats.push(length)
      }
    }
  }

  return cheats;
}
