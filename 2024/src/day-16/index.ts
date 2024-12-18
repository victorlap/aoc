import {parseGrid, readInput} from 'io'
import {Direction, Point} from "utils/grid.ts";
import {min} from "utils";

const input = await readInput('day-16')
const possibleDirections: Direction[] = [[1, 0], [-1, 0], [0, 1], [0, -1]]

export const part1 = () => {
  const grid = parseGrid(input)

  const startPos = grid.allPoints().find(p => p.value === 'S')!
  const startDir = [0, -1] satisfies Direction;
  const endPos = grid.allPoints().find(p => p.value === 'E')!

  let costMap = new Map<string, number>
  const costKey = (point: Point<String>, dir: Direction) => '' + point.y + ',' + point.x + dir[0] + dir[1]

  type QueueItem = { 'point': Point<string>, cost: number, dir: Direction }
  let queue: QueueItem[] = [{point: startPos, dir: startDir, cost: 0}]

  while (queue.length) {
    let {point, cost, dir} = queue.shift()!

    if (point.value === "E") continue;

    const possible = [
      {point: grid.move(point, dir), dir: dir, cost: cost + 1},
      {point: point, dir: [dir[1], -dir[0]], cost: cost + 1000},
      {point: point, dir: [-dir[1], dir[0]], cost: cost + 1000},
    ] as QueueItem[]

    for (const p of possible) {
      if (!['S', 'E', '.'].includes(p.point.value)) continue;
      if (p.cost > (costMap.get(costKey(p.point, p.dir)) || Infinity)) continue;
      costMap.set(costKey(p.point, p.dir), p.cost)
      queue.push(p)
    }
  }

  return min(possibleDirections.map(d => costMap.get(costKey(endPos, d))!))
}

export const part2 = () => {
  const grid = parseGrid(input)

  const startPos = grid.allPoints().find(p => p.value === 'S')!
  const startDir = [0, -1] satisfies Direction;
  const endPos = grid.allPoints().find(p => p.value === 'E')!

  let costMap = new Map<string, { cost: number, path: Set<Point<string>> }>
  const costKey = (point: Point<String>, dir: Direction) => '' + point.y + ',' + point.x + dir[0] + dir[1]

  type QueueItem = { 'point': Point<string>, cost: number, dir: Direction, path: Set<Point<string>> }
  let queue: QueueItem[] = [{point: startPos, dir: startDir, cost: 0, path: new Set}]

  while (queue.length) {
    let {point, cost, dir, path} = queue.shift()!
    path.add(point)

    if (point.value === "E") continue;

    const possible: Omit<QueueItem, 'path'>[] = [
      {point: grid.move(point, dir), dir: dir, cost: cost + 1},
      {point: point, dir: [dir[1], -dir[0]] as Direction, cost: cost + 1000},
      {point: point, dir: [-dir[1], dir[0]] as Direction, cost: cost + 1000},
    ]

    for (const p of possible) {
      if (!['S', 'E', '.'].includes(p.point.value)) continue;
      let prevPath = costMap.get(costKey(p.point, p.dir));
      if (p.cost > (prevPath?.cost || Infinity)) continue;
      let newPath = p.cost === prevPath?.cost ? path.union(prevPath.path) : path;
      costMap.set(costKey(p.point, p.dir), {cost: p.cost, path: newPath})
      queue.push({...p, path: new Set(newPath)})
    }
  }

  let costs = possibleDirections.map(d => costMap.get(costKey(endPos, d)));
  let win = min(costs.map(c => c?.cost || Infinity))
  let bestPath = costs.find(c => c?.cost == win)!

  return bestPath.path.size + 1
}

