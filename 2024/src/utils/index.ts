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

export const min = <T extends number | bigint>(numbers: T[]) => {
  return numbers.reduce((prev: T | undefined, num) => !prev || prev > num ? num : prev, undefined)
}

export const max = (numbers: number[]) => {
  return numbers.reduce((prev, num) => prev < num ? num : prev, Infinity)
}

export const minBy = <T>(items: T[], pluck: (item: T) => number) => items.reduce((seed: T | null, item) => {
  return (seed && pluck(seed) < pluck(item)) ? seed : item;
}, null);
export const maxBy = <T>(items: T[], pluck: (item: T) => number) => items.reduce((seed: T | null, item) => {
  return (seed && pluck(seed) > pluck(item)) ? seed : item;
}, null);

export const pluck = <T, K extends keyof any, V>(items: T[], key: (item: T) => K, value: (item: T) => V): Record<K, V> => {
  let res = {} as Record<K, V>;

  for (const item of items) {
    res[key(item)] = value(item)
  }

  return res;
}

export const groupBy = <K extends keyof any, T>(items: T[], accumulator: (item: T) => K) => {
  return items.reduce((acc, cur) => {
    let key = accumulator(cur)
    acc[key] = acc[key] || []
    acc[key].push(cur)
    return acc
  }, {} as Record<K, T[]>)
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

export function* permutations<T>(elements: T[]): Generator<T[]> {
  if (elements.length === 1) {
    yield elements;
  } else {
    let [first, ...rest] = elements;
    for (let perm of permutations(rest)) {
      for (let i = 0; i < elements.length; i++) {
        let start = perm.slice(0, i);
        let rest = perm.slice(i);
        yield [...start, first, ...rest];
      }
    }
  }
}

export function orderedMultiSubsets<T>(set: Set<T>, n: number): Generator<T[]> {
  if (!Number.isInteger(n) || n < 0) return function* () {
  }();
  const subset = new Array(n), iterator = set.values();
  return (function* backtrack(index): Generator<T[]> {
    if (index === n) {
      yield subset.slice();
    } else {
      for (let i = 0; i < set.size; ++i) {
        subset[index] = iterator.next().value; /* Get first item */
        set.delete(subset[index]); /* Remove it */
        set.add(subset[index]); /* Insert it at the end */
        yield* backtrack(index + 1);
      }
    }
  })(0);
}

