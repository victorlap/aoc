import {parseGrid, readInput} from 'io'
import {Direction, Grid, Point} from "utils/grid.ts";
import {sum} from "utils";

const input = await readInput('day-10')

const possibleDirections: Direction[] = [[1, 0], [-1, 0], [0, 1], [0, -1]]

export const part1 = () => {
  const grid = parseGrid(input, '', Number)

  let trailheads = grid.allPoints()
    .filter(p => p.value === 0)
    .map(p => findTrailheads(p, grid))
    .toArray()

  return sum(trailheads)
}

export const part2 = () => {
  const grid = parseGrid(input, '', Number)

  let trailheads = grid.allPoints()
    .filter(p => p.value === 0)
    .map(p => findTrailheads2(p, grid))
    .toArray()

  return sum(trailheads)
}

const findTrailheads = (point: Point<number>, grid: Grid<number>): number => {
  let queue = possibleDirections.map(d => [point, grid.move(point, d)])
  let visited = new Set<Point<number>>
  let foundTrails = 0

  while (queue.length > 0) {
    let [cur, next] = queue.shift()!

    if (!next || next?.value - cur?.value !== 1 || visited.has(next)) {
      continue;
    }

    visited.add(next)

    if (next.value === 9) {
      foundTrails++
      continue;
    }

    for (const dir of possibleDirections) {
      queue.push([next, grid.move(next, dir)])
    }
  }

  return foundTrails;
}


const findTrailheads2 = (point: Point<number>, grid: Grid<number>): number => {
  let queue = possibleDirections.map(d => [point, grid.move(point, d)])
  let foundTrails = 0

  while (queue.length > 0) {
    let [cur, next] = queue.shift()!

    if (!next || next?.value - cur?.value !== 1) {
      continue;
    }

    if (next.value === 9) {
      foundTrails++
      continue;
    }

    for (const dir of possibleDirections) {
      queue.push([next, grid.move(next, dir)])
    }
  }

  return foundTrails;
}
