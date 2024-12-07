import {parseLines, readInput} from 'io'

const input = await readInput('day-07')

export const part1 = () => {
  const lines = parseLines(input)
  let sum = 0;

  for (const line of lines) {
    let [ans, equation] = line.split(': ');
    let answer = Number(ans)
    let numbers = equation.split(' ').map(Number)

    if (canMatch(answer, numbers.reverse())) {
      sum += answer
    }
  }

  return sum
}

export const part2 = () => {
  const lines = parseLines(input)
  let sum = 0;

  for (const line of lines) {
    let [ans, equation] = line.split(': ');
    let answer = Number(ans)
    let numbers = equation.split(' ').map(Number)

    if (canMatch2(answer, numbers.reverse())) {
      sum += answer
    }
  }

  return sum
}

const canMatch = (answer: number, numbers: number[]): boolean => {
  if (!Number.isInteger(answer) || answer === 0) {
    return false
  }

  if (numbers.length > 1) {
    let [first, ...rest] = numbers
    return canMatch(answer - first, rest) || canMatch(answer / first, rest)
  }

  return (answer - numbers[0]) === 0 || (answer / numbers[0]) === 0;
}

const canMatch2 = (answer: number, numbers: number[]): boolean => {
  if (!Number.isInteger(answer) || answer === 0) {
    return false
  }

  if (numbers.length > 1) {
    let [first, ...rest] = numbers
    let firstStr = first.toString()
    let ansStr = answer.toString()

    return (
      canMatch2(answer - first, rest) ||
      canMatch2(answer / first, rest) ||
      (ansStr.endsWith(firstStr) && canMatch2(Number(ansStr.substring(0, ansStr.length - firstStr.length)), rest))
    )
  }

  return (answer - numbers[0]) === 0 || (answer / numbers[0]) === 0;
}
