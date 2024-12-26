import {parseLines, readInput} from 'io'

const input = await readInput('day-22')

export const part1 = () => {
  const lines = parseLines(input, Number)

  let sum = 0;
  for (let number of lines) {
    for (let i = 0; i < 2000; i++) {
      number = next(number)
    }
    sum += number
  }

  return sum
}

export const part2 = () => {
  const lines = parseLines(input, Number)

  let temp = 0;
  let seq = []
  let best: Record<number, Record<string, number>> = {}

  for (let i = 0; i < lines.length; i++) {
    best[i] = {}
    let number = lines[i]

    for (let j = 0; j < 2000; j++) {
      temp = next(number);
      let [prev, price] = [number % 10, temp % 10]
      let diff = price - prev;

      seq.push(diff)
      seq.length > 4 && seq.shift();

      best[i][seq.join(',')] == undefined && (best[i][seq.join(',')] = price)

      number = temp
    }
  }

  let highest = 0;

  for (let one = -9; one < 10; one++) {
    for (let two = -9; two < 10; two++) {
      for (let three = -9; three < 10; three++) {
        for (let four = -9; four < 10; four++) {
          let sum = 0;
          for (let i = 0; i < lines.length; i++) {
            sum += (best[i]?.[`${one},${two},${three},${four}`] || 0)
          }
          sum > highest && (highest = sum)
        }
      }
    }
  }

  return highest
}

const next = (number: number) => {
  number = number ^ number << 6 & 2 ** 24 - 1
  number = number ^ number >> 5 & 2 ** 24 - 1
  number = number ^ number << 11 & 2 ** 24 - 1
  return number;
}
