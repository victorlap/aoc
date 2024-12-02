import {parseLines, readInput} from 'io'

const input = await readInput('day-02')

export const part1 = () => {
  const lines = parseLines(input).map(l => l.split(' ').map(Number))
  let validCount = 0

  for (const report of lines) {
    const increasing = report[0] > report[1]
    let safe = true;

    for (let i = 0; i < report.length - 1; i++) {
      const diff = report[i] - report[i + 1]
      if (increasing) {
        if (diff < 1 || diff > 3) {
          safe = false;
          break;
        }
      } else {
        if (diff < -3 || diff > -1) {
          safe = false;
          break;
        }
      }
    }

    if (safe) {
      validCount++;
    }
  }

  return validCount
}
