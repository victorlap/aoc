import {parseLines, readInput} from 'io'
import {subsets} from "utils";

const input = await readInput('day-23')

export const part1 = () => {
  const lines = parseLines(input)

  let connections: Record<string, string[]> = {}
  const add = (a: string, b: string) => (connections[a] = connections[a] || []).push(b)

  for (const line of lines) {
    let [one, two] = line.split('-')
    add(one, two)
    add(two, one)
  }

  const possible = Object.keys(connections).filter(id => id.startsWith('t'))
  const validated = new Set<string>

  for (const id of possible) {
    let sub = subsets(connections[id], 2).toArray()
    for (const s of sub) {
      if (connections[s[0]].includes(id) && connections[s[0]].includes(s[1]) &&
        connections[s[1]].includes(id) && connections[s[1]].includes(s[0])) {
        validated.add([id, ...s].toSorted().join(''))
      }
    }
  }

  return validated.size
}

export const part2 = () => {
  const lines = parseLines(input)

  let connections: Record<string, string[]> = {}
  const add = (a: string, b: string) => (connections[a] = connections[a] || []).push(b)

  for (const line of lines) {
    let [one, two] = line.split('-')
    add(one, two)
    add(two, one)
  }

  const possible = Object.keys(connections)

  for (let i = connections[possible[0]].length; i >= 2; i--) {
    for (const id of possible) {
      let sub = subsets(connections[id], i).toArray()
      inner:
        for (const s of sub) {
          for (let j of s) {
            if (!connections[j].includes(id)) break inner;
            for (let k of s) {
              if (j !== k && !connections[j].includes(k)) {
                continue inner
              }
            }
          }
          return [id, ...s].toSorted().join(',')
        }
    }
  }
}
