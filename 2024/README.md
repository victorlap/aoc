# 🎄 Advent of Code 2024

![Bun](https://img.shields.io/badge/Bun-%23000000.svg?style=for-the-badge&logo=bun&logoColor=white) ![TypeScript](https://img.shields.io/badge/typescript-%23007ACC.svg?style=for-the-badge&logo=typescript&logoColor=white)
![ESLint](https://img.shields.io/badge/ESLint-4B3263?style=for-the-badge&logo=eslint&logoColor=white)

- **Bun & TypeScript**, with ESLint
- Automatic day setup
- Puzzle input automatically fetched
- Performance metrics

Based on https://github.com/matijaoe/advent-of-code

## Setup

Ensure [`bun`](https://bun.sh/) is installed:
```
bun -v

# if not installed, run this:
curl -fsSL https://bun.sh/install | bash
```

Install dependencies:
```bash
bun install
```

Set up a new day:
```bash
bun setup 1
```

Run day solutions:
```bash
bun day 1
```

Run day solutions in dev mode (without clearing console outputs):
```bash
bun day 1 --dev
```

For automatic puzzle input retrieval, define `.env` file with `session` cookie from [adventofcode.com](https://adventofcode.com):
```env
SESSION=
YEAR=2023
```

## Structure

Generated day structure:
```ts
import { parseLines, readInput } from 'io'

const input = await readInput('day-01')

export const part1 = () => {
  const lines = parseLines(input)
  // your code goes here
  return lines.length
}
```

Expected `index.ts` solution structure (day 1 of 2022):
```ts
import { parseGroups, readInput } from 'io'
import { desc, sum } from 'utils'

const input = await readInput('day-01')

export const part1 = () => {
  const lines = parseGroups(input, Number)
  return lines.map(sum).sort(desc).at(0)
}

export const part2 = () => {
  const lines = parseGroups(input, Number)
  const top = lines.map(sum).sort(desc).slice(0, 3)
  return sum(top)
}
```

`bun day <day-num>` output:
```
🌲 Part One: 70698 (1.11 ms)
🎄 Part Two: 140471 (383.5 µs)
```
