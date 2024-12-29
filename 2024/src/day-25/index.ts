import {parseGroups, readInput} from 'io'
import {sum} from "utils";

const input = await readInput('day-25')

export const part1 = () => {
  const groups = parseGroups(input)

  const [locks, keys] = groups.reduce((acc, cur) => (acc[cur[0] == "#####" ? 0 : 1].push(cur), acc), [[], []]);
  const lockMaps = locks.map(heightmap)
  const keyMaps = keys.map(heightmap)

  return sum(lockMaps.map(lock => keyMaps.filter(keyMap => combine(lock, keyMap)).length))
}

const heightmap = (item: string[]) => {
  let map = Array(5).fill(-1)

  for (let i = 0; i < item.length; i++) {
    for (let j = 0; j < item[i].length; j++) {
      item[i][j] == "#" && map[j]++
    }
  }

  return map;
}

const combine = (lock: number[], key: number[]) => {
  for (let i = 0; i < lock.length; i++) {
    if (lock[i] + key[i] > 5) {
      return false
    }
  }

  return true;
}
