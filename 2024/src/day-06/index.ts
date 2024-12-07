import {parseGrid, readInput} from 'io'
import {Direction, Grid, Point} from "utils/grid";

const input = await readInput('day-06')

export const part1 = () => {
  const grid = parseGrid(input, '')

  let pos = grid.first(p => p.value === '^')!
  let nextPos = pos
  let dir = getDir(pos.value);

  while (nextPos) {
    nextPos = grid.move(pos, dir)

    if (nextPos?.value == "#") {
      dir = nextDir(dir)
    } else {
      grid.set(pos.y, pos.x, "X")
      pos = nextPos
    }
  }

  let sum = 0
  grid.each(p => p.value === "X" && sum++)

  return sum
}

export const part2 = () => {
  const grid = parseGrid(input, '')

  let pos = grid.first(p => p.value === '^')!
  let nextPos = pos
  let dir = getDir(pos.value);
  let visited = new Map<string, boolean>

  while (nextPos) {
    grid.set(pos.y, pos.x, "X")
    nextPos = grid.move(pos, dir)

    if (nextPos?.value === ".") {
      let clone = grid.clone();
      clone.set(nextPos.y, nextPos.x, "O")
      if (startsLoop(clone, pos, nextDir(dir))) {
        visited.set(`${pos.y}${pos.x}${dir[0]}${dir[1]}`, true)
        // console.log(clone.toString())
      }
    }

    if (nextPos?.value == "#") {
      dir = nextDir(dir)
    } else {
      pos = nextPos
    }
  }

  return visited.size
}


const startsLoop = (grid: Grid, startPos: Point<string>, startDir: Direction) => {
  let pos = startPos;
  let dir = startDir
  let nextPos = startPos
  let visited = new Map<string, boolean>
  let key = '';

  while (nextPos) {
    grid.set(pos.y, pos.x, "X")
    nextPos = grid.move(pos, dir)

    if (nextPos?.value == "#" || nextPos?.value == "O") {
      dir = nextDir(dir)

      key = `${pos.y}${pos.x}${dir[0]}${dir[1]}`
      if (visited.has(key)) {
        return true;
      } else {
        visited.set(key, true)
      }
    } else {
      pos = nextPos
    }
  }

  return false;
}


const getDir = (value: string): Direction => {
  switch (value) {
    case '^':
      return [-1, 0]
    case '>':
      return [0, 1]
    case 'v':
      return [1, 0]
    case '<':
      return [0, -1]
    default:
      throw 'Oops!'
  }
}

const nextDir = (dir: Direction): Direction => {
  return [dir[1], -dir[0]] as Direction
}
