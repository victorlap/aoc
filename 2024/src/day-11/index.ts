import {readInput} from 'io'
import {sum} from "utils";

const input = await readInput('day-11')

export const part1 = () => {
  let stones = input.split(' ')

  for (let i = 0; i < 25; i++) {
    stones = stones.flatMap(stone => blink(stone))
  }

  return stones.length
}

export const part2 = () => {
  let stones = input.split(' ').reduce((acc, stone) => {
    acc[stone] = 1
    return acc;
  }, {} as Record<string, number>)

  for (let i = 0; i < 75; i++) {
    stones = blink2(stones);
  }

  return sum(Object.values(stones))
}


const blink = (stone: string) => {
  if (stone === "0") {
    return ["1"]
  }

  if (stone.length % 2 === 0) {
    return [(stone.slice(0, stone.length / 2)), (Number(stone.slice(stone.length / 2)).toString())]
  }

  return [(Number(stone) * 2024).toString()]
}
const blink2 = (stones: Record<string, number>) => {
  let ret: Record<string, number> = {}

  for (const [stone, num] of Object.entries(stones)) {
    let newStones = blink(stone);
    for (const newStone of newStones) {
      ret[newStone] = (ret[newStone] || 0) + num
    }
  }

  return ret;
}
