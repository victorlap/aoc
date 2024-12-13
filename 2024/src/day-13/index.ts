import {parseGroups, readInput} from 'io'

const input = await readInput('day-13')

export const part1 = () => {
  const groups = parseGroups(input)

  let tokens = 0

  for (const group of groups) {
    let regex = /Button [A|B]: X\+(\d+), Y\+(\d+)/
    let [, aX, aY] = regex.exec(group[0])!
    let [, bX, bY] = regex.exec(group[1])!
    let [, prizeX, prizeY] = /Prize: X=(\d+), Y=(\d+)/.exec(group[2])!

    const aTokens = 3
    const bTokens = 1

    let cram = cramer(Number(aX), Number(bX), Number(aY), Number(bY), Number(prizeX), Number(prizeY))
    if (cram.every(n => Number.isInteger(n))) {
      tokens += (cram[0] * aTokens + cram[1] * bTokens)
    }
  }

  return tokens
}

export const part2 = () => {
  const groups = parseGroups(input)

  let tokens = 0

  for (const group of groups) {
    let regex = /Button [A|B]: X\+(\d+), Y\+(\d+)/
    let [, aX, aY] = regex.exec(group[0])!
    let [, bX, bY] = regex.exec(group[1])!
    let [, prizeX, prizeY] = /Prize: X=(\d+), Y=(\d+)/.exec(group[2])!

    const aTokens = 3
    const bTokens = 1

    let cram = cramer(Number(aX), Number(bX), Number(aY), Number(bY), 10000000000000 + Number(prizeX), 10000000000000 + Number(prizeY))
    if (cram.every(n => Number.isInteger(n))) {
      tokens += (cram[0] * aTokens + cram[1] * bTokens)
    }
  }

  return tokens
}

/**
 Cramer's Rule for 2 unknowns:

 Given two equations:
 ax + by = e
 cx + dy = f

 Calculates the values of x and y

 Source: https://www.1728.org/cramer.htm
 */
const cramer = (a: number, b: number, c: number, d: number, e: number, f: number) => {
  let dn = (a * d) - (c * b)
  let x = ((e * d) - (f * b)) / dn
  let y = ((a * f) - (c * e)) / dn

  return [x, y]
}
