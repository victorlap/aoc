import {parseLines, readInput} from 'io'

const input = await readInput('day-02')

export const part1 = () => {
  const lines = parseLines(input).map(l => l.split(' ').map(Number))

  return lines.map(checkReport).filter(Boolean).length
}

export const part2 = () => {
  const lines = parseLines(input).map(l => l.split(' ').map(Number))

  return lines.map(checkReportWithDampener).filter(Boolean).length
}

const checkReport = (report: number[]) =>  {
  const increasing = report[0] > report[1]

  for (let i = 0; i < report.length - 1; i++) {
    const diff = report[i] - report[i + 1]
    if (increasing) {
      if (diff < 1 || diff > 3) {
        return false;
      }
    } else {
      if (diff < -3 || diff > -1) {
        return false;
      }
    }
  }

  return true;
}

const checkReportWithDampener = (report: number[]) => {
  // Check original
  if(checkReport(report)) {
    return true;
  }

  // Check with one removed
  for (let i = 0; i < report.length; i++) {
    const copy = [...report];
    delete copy[i];

    if(checkReport(copy.filter(Number))) {
      return true;
    }
  }

  return false;
}
