export const isNumber = (value: unknown): value is number => typeof value === 'number'
export const isString = (value: unknown): value is string => typeof value === 'string'
export const isBoolean = (value: unknown): value is boolean => typeof value === 'boolean'
export const isObject = (value: unknown): value is object => typeof value === 'object'

export const sum = (numbers: number[]) => {
  return numbers.reduce((sum, num) => sum + num, 0)
}

export const mul = (numbers: number[]) => {
  return numbers.reduce((prev, num) => prev * num, 1)
}

export const unique = <T>(arr: T[]): T[] => {
  return [...new Set(arr)]
}

export const min = (numbers: number[]|bigint[]) => {
  return numbers.reduce((prev, num) => prev > num ? num : prev, Infinity)
}

export const max = (numbers: number[]) => {
  return numbers.reduce((prev, num) => prev < num ? num : prev, Infinity)
}

export const swapInArray = <T>(arr: T[], from: number, to: number) => {
  const temp = arr[from];
  arr.splice(from, 1, arr[to]);
  arr.splice(to, 1, temp);
}

export const asc = <T extends number | string>(a: T, b: T): number => {
  if (isNumber(a) && isNumber(b)) {
    return a - b
  } else if (isString(a) && isString(b)) {
    return a.localeCompare(b)
  }
  throw new Error('Invalid argument types')
}

// https://stackoverflow.com/a/64777515/2232346
export const chunk = <T>(arr: T[], size: number): T[][] =>
  [...Array(Math.ceil(arr.length / size))].map((_, i) =>
    arr.slice(size * i, size + size * i)
  );

export const desc = <T extends number | string>(a: T, b: T): number => {
  if (isNumber(a) && isNumber(b)) {
    return b - a
  } else if (isString(a) && isString(b)) {
    return b.localeCompare(a)
  }
  throw new Error('Invalid argument types')
}

export const isBetween = (x: number, [min, max]: [number, number]) => {
  return x >= min && x <= max
}

// https://stackoverflow.com/q/37892738/2232346
export function* subsets<T>(array: T[], length: number, start = 0): Generator<T[]> {
  if (start >= array.length || length < 1) {
    yield [];
  } else {
    while (start <= array.length - length) {
      let first = array[start];
      for (const subset of subsets(array, length - 1, start + 1)) {
        subset.push(first);
        yield subset;
      }
      ++start;
    }
  }
}

