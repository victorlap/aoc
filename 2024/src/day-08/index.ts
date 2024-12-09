import {parseGrid, readInput} from 'io'
import {Grid, Point} from "utils/grid.ts";
import {subsets} from "utils";

const input = await readInput('day-08')

export const part1 = () => {
  const grid = parseGrid(input)
  const towers = findTowers(grid);
  const antinodes = new Set()

  for (const [freq, tw] of Object.entries(towers)) {
    for (const [one, two] of subsets(tw, 2)) {
      const top = one.y > two.y ? two : one;
      const bot = one.y > two.y ? one : two;

      const vec = [top.y - bot.y, top.x - bot.x];
      const up = grid.get(top.y + vec[0], top.x + vec[1])
      const down = grid.get(bot.y - vec[0], bot.x - vec[1])

      up && antinodes.add(up)
      down && antinodes.add(down)
      // up && grid.set(up?.y, up?.x, '#')
      // down && grid.set(down?.y, down?.x, '#')
    }
  }

  return antinodes.size
}

export const part2 = () => {
  const grid = parseGrid(input)
  const towers = findTowers(grid);
  const antinodes = new Set()

  for (const [freq, tw] of Object.entries(towers)) {
    for (const [one, two] of subsets(tw, 2)) {
      const top = one.y > two.y ? two : one;
      const bot = one.y > two.y ? one : two;

      const vec = [top.y - bot.y, top.x - bot.x];
      let up = grid.get(bot.y + vec[0], bot.x + vec[1])
      antinodes.add(up)
      do {
        up = grid.get(up.y + vec[0], up.x + vec[1])
        up && antinodes.add(up)
        // up && grid.set(up?.y, up?.x, '#')
      } while (up)

      let down = grid.get(top.y - vec[0], top.x - vec[1])
      antinodes.add(down)
      do {
        down = grid.get(down.y - vec[0], down.x - vec[1])
        down && antinodes.add(down)
        // down && grid.set(down?.y, down?.x, '#')
      } while (down)
    }
  }

  return antinodes.size
}

function findTowers(grid: Grid<string>) {
  return grid.allPoints()
    .filter((c) => c.value !== '.')
    .reduce((acc, cur) => {
      acc[cur.value] = (acc[cur.value] || []);
      acc[cur.value].push(cur)
      return acc;
    }, {} as Record<string, Point<string>[]>);
}
