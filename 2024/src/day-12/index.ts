import {parseGrid, readInput} from 'io'
import {Direction, Grid, Point} from "utils/grid.ts";

const input = await readInput('day-12')

const possibleDirections: Direction[] = [[1, 0], [-1, 0], [0, 1], [0, -1]]
type Cardinal = "N" | "E" | "S" | "W"

export const part1 = () => {
  const grid = parseGrid(input)

  const visited = new Map<Point<string>, true>;
  let sum = 0;

  grid.allPoints().forEach(p => {
    if (visited.has(p)) return
    let queue = [p]
    let area = 0;
    let perim = 0;

    while (queue.length) {
      let cur = queue.shift()!
      if (visited.has(cur)) continue
      visited.set(cur, true)
      area += 1
      perim += perimChange(cur, grid, visited)

      for (const dir of possibleDirections) {
        let neighbor = grid.move(cur, dir)
        if (neighbor?.value == cur.value) {
          queue.push(neighbor)
        }
      }
    }

    sum += area * perim
    // console.log(`${p.value}: ${area} x ${perim} = ${sum}`)
  })

  return sum
}

export const part2 = () => {
  const grid = parseGrid(input)

  const visited = new Map<Point<string>, true>;
  const fences = new Map<Point<string>, Set<Cardinal>>

  grid.allPoints().forEach(p => {
    let myFences = new Set<Cardinal>;

    for (const dir of possibleDirections) {
      let neighbor = grid.move(p, dir)
      if (!neighbor?.value || neighbor.value != p.value) {
        myFences.add(toCardinal(dir))
      }
    }

    fences.set(p, myFences)
  })

  let sum = 0

  grid.allPoints().forEach(p => {
    if (visited.has(p)) return
    let queue = [p]
    let area = 0;
    let sides = 0;

    while (queue.length) {
      let cur = queue.shift()!
      if (visited.has(cur)) continue
      visited.set(cur, true)

      let visitedNeighbors = possibleDirections.map(d => grid.move(cur, d)).filter(n => n?.value === cur.value && visited.has(n))
      let mySides = fences.get(cur)!
      for (const n of visitedNeighbors) {
        mySides = mySides.difference(fences.get(n)!)
      }

      area += 1
      sides += mySides.size

      for (const dir of possibleDirections) {
        let neighbor = grid.move(cur, dir)
        if (neighbor?.value !== cur.value || visited.has(neighbor)) continue;
        queue.push(neighbor)
      }

      // Walk correctly around gaps
      queue.sort((a, b) => b.y - a.y)
    }

    sum += area * sides
    // console.log(`${p.debug()} ${area} ${sides}`)
  })

  return sum
}


const perimChange = (cur: Point<string>, grid: Grid<string>, visited: Map<Point<string>, true>): number => {
  let numNeighbors = possibleDirections.map(d => grid.move(cur, d)).filter(n => n?.value === cur.value && visited.has(n)).length
  switch (numNeighbors) {
    case 0 :
      return 4
    case 1 :
      return 2
    case 2 :
      return 0;
    case 3 :
      return -2
    case 4:
      return 0;
  }

  throw 'panic'
}

const toCardinal = (dir: Direction): Cardinal => {
  if (dir[0] < 0) return 'N'
  if (dir[0] > 0) return 'S'
  if (dir[1] < 0) return 'W'
  if (dir[1] > 0) return 'E'

  throw 'panic'
}
