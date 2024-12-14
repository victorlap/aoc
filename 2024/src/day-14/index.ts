import {parseLines, readInput} from 'io'
import {Grid} from "utils/grid.ts";
import {mul} from "utils";

const input = await readInput('day-14')

export const part1 = () => {
  const re = /-?\d+/g
  const robots = parseLines(input).map(l => l.match(re)!.map(Number))

  // const width = 11;
  // const height = 7;
  const width = 101;
  const height = 103;

  const emptyGrid = Grid.new(height, width, () => [] as number[][])
  let grid = emptyGrid.clone((v) => [...v])

  for (const robot of robots) {
    let cur = grid.get(robot[1], robot[0]).value
    cur.push(robot)
    grid.set(robot[1], robot[0], cur)
  }

  for (let i = 0; i < 100; i++) {
    grid = walk(grid, emptyGrid, height, width)
  }

  return safetyScore(grid, height, width)
}
export const part2 = () => {
  const re = /-?\d+/g
  const robots = parseLines(input).map(l => l.match(re)!.map(Number))

  // const width = 11;
  // const height = 7;
  const width = 101;
  const height = 103;

  const emptyGrid = Grid.new(height, width, () => [] as number[][])
  let grid = emptyGrid.clone((v) => [...v])
  let lowestSafetyScore = Infinity
  let lowestSafetyScoreAt = 0

  for (const robot of robots) {
    let cur = grid.get(robot[1], robot[0]).value
    cur.push(robot)
    grid.set(robot[1], robot[0], cur)
  }

  for (let i = 0; i < 101 * 103; i++) {
    grid = walk(grid, emptyGrid, height, width)
    let safe = safetyScore(grid, height, width)
    if (safe < lowestSafetyScore) {
      lowestSafetyScore = safe
      lowestSafetyScoreAt = i
      // console.log(i, safe)
      // console.log(grid.toString((v) => v.length || ' '))
    }
  }

  return lowestSafetyScoreAt + 1
}

const walk = (grid: Grid<number[][]>, emptyGrid: Grid<number[][]>, height: number, width: number) => {
  let newGrid = emptyGrid.clone((v) => [...v])

  grid.allPoints().forEach(({y, x, value: robots}) => {
    for (const robot of robots) {
      let newPos = [(height + y + robot[3]) % height, (width + x + robot[2]) % width]
      let cur = newGrid.get(newPos[0], newPos[1]).value
      cur.push(robot)
      newGrid.set(newPos[0], newPos[1], cur)
    }
  })

  return newGrid
}

const safetyScore = (grid: Grid<number[][]>, height: number, width: number) => {
  const halfWidth = width / 2
  const halfHeight = height / 2

  return mul(
    grid.allPoints().reduce((cur, p) => {
      switch (true) {
        case p.y < halfHeight - 1 && p.x < halfWidth - 1:
          cur[0] += p.value.length
          break
        case p.y < halfHeight - 1 && p.x > halfWidth:
          cur[1] += p.value.length
          break
        case p.y > halfHeight && p.x < halfWidth - 1:
          cur[2] += p.value.length
          break
        case p.y > halfHeight && p.x > halfWidth:
          cur[3] += p.value.length
          break
      }
      return cur
    }, [0, 0, 0, 0])
  )
}
