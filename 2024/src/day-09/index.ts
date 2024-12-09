import {parseLines, readInput} from 'io'
import {sum} from "utils";

const input = await readInput('day-09')

export const part1 = () => {
  const disk = parseLines(input)[0].split('')

  const expanded = expand(disk)
  const defragged = defrag(expanded)

  return sum(defragged)
}

export const part2 = () => {
  const disk = parseLines(input)[0].split('')

  const expanded = expand(disk)
  const defragged = defrag2(expanded)

  return sum(defragged)
}

const expand = (disk: string[]) => {
  return disk.flatMap((char, id) => Array(Number(char)).fill((id % 2 === 0 ? (id / 2) : '.')))
}

const defrag = (disk: string[]) => {
  let backFilled = disk.length;
  return disk
    .map((val, id) => {
      if (id < backFilled && val !== '.') {
        return Number(val) * id
      }

      let nextIndex = disk.findLastIndex((val, id) => id < backFilled && val !== '.');
      if (id < nextIndex) {
        backFilled = nextIndex

        return Number(disk[nextIndex]) * id;
      }

      return 0;
    })
}

const defrag2 = (disk: string[]) => {
  let dataPos = disk.length - 1

  while (dataPos > 0) {
    let freePos = 0
    while (disk[dataPos] == '.') {
      dataPos--
    }

    let group = disk[dataPos];
    let groupLength = 0;
    while (disk[dataPos] == group) {
      groupLength++
      dataPos--
    }

    let freeLength = 0;
    while (freeLength < groupLength && freePos < disk.length) {
      freeLength = 0
      while (disk[freePos] != '.') {
        freePos++
      }
      // console.log(freePos)
      while (disk[freePos] == '.') {
        freeLength++
        freePos++
      }
    }
    freePos -= freeLength

    // swap
    if (dataPos > freePos) {
      for (let i = 0; i < groupLength; i++) {
        disk[dataPos + groupLength - i] = '.'
        disk[freePos + i] = group
      }
    }
  }

  return disk.map((l, id) => Number(l === "." ? 0 : l) * id)
}
