import {parseGrid, parseGroups, readInput} from 'io'
import {Direction, Grid, Point} from "utils/grid.ts";
import {sum} from "utils";

const input = await readInput('day-15')
const directions: Record<string, Direction> = {'^': [-1, 0], 'v': [1, 0], '>': [0, 1], '<': [0, -1],}

export const part1 = () => {
  const groups = parseGroups(input)
  const grid = parseGrid(groups[0].join('\n'))
  const instructions = groups[1].join('')

  let pos = grid.allPoints().find(p => p.value === "@")!

  for (const i of instructions) {
    pos = move(grid, pos, directions[i])
  }

  return sum(grid.allPoints().map(p => p.value === "O" ? (100 * p.y + p.x) : 0).toArray())
}

export const part2 = async () => {
  const groups = parseGroups(input)
  const instructions = groups[1].join('')
  let grid = parseGrid(groups[0].map(double).join('\n'))
  let pos = grid.allPoints().find(p => p.value === "@")!

  for (const i of instructions) {
    [grid, pos] = move2(grid, pos, directions[i])
    // await Bun.sleep(1)
    // Bun.write(Bun.stdout, '\x1b[' + grid.height + 'A')
    // Bun.write(Bun.stdout, grid.toString())
  }

  return sum(grid.allPoints().map(p => p.value === "[" ? (100 * p.y + p.x) : 0).toArray())
}

const move = (grid: Grid<string>, pos: Point<string>, dir: Direction) => {
  let nextEmpty = pos;
  while (true) {
    nextEmpty = grid.move(nextEmpty, dir)!
    if (nextEmpty.value === "." || nextEmpty.value === "#") break;
  }

  // no free space found
  if (nextEmpty?.value !== ".") {
    return pos;
  }

  // move everything over one until free space
  let opposite = [dir[0] * -1, dir[1] * -1] as Direction
  let cur = nextEmpty;
  while (!cur.eq(pos)) {
    let prev = grid.move(cur, opposite)
    grid.set(cur.y, cur.x, prev?.value)
    cur = prev;
  }

  grid.set(cur.y, cur.x, '.')
  return grid.move(pos, dir)
}

const double = (line: string) => {
  return line.split('').flatMap(c => {
    switch (c) {
      case '@':
        return ['@', '.'];
      case 'O':
        return ['[', ']']
      default:
        return [c, c]
    }
  }).join('')
}

const move2 = (grid: Grid<string>, pos: Point<string>, dir: Direction) => {
  let nextEmpty = pos;
  while (true) {
    nextEmpty = grid.move(nextEmpty, dir)!
    if (nextEmpty.value === "." || nextEmpty.value === "#") break;
  }

  // no free space found
  if (nextEmpty?.value !== ".") {
    return [grid, pos] as const;
  }

  let newGrid = grid.clone()
  let opposite = [dir[0] * -1, dir[1] * -1] as Direction
  let cur = nextEmpty;
  while (!cur.eq(pos)) {
    let prev = newGrid.move(cur, opposite)

    if (!prev.eq(pos) && (prev?.value === "[" || prev?.value === "]") && dir[0] !== 0) {
      let nextSide = newGrid.move(prev, [0, opposite[1] + (prev.value === "]" ? -1 : 1)])
      let move2res;
      [newGrid, move2res] = move2(newGrid, nextSide, dir);
      if (nextSide.eq(move2res)) {
        return [grid, pos] as const;
      }
    }

    newGrid.set(cur.y, cur.x, prev?.value)
    cur = prev;
  }

  newGrid.set(cur.y, cur.x, '.')
  return [newGrid, newGrid.move(pos, dir)] as const
}
