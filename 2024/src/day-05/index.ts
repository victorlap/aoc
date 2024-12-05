import {parseGroups, readInput} from 'io'
import {sum, swapInArray} from "utils";

const input = await readInput('day-05')

type Order = Record<number, number[]>

export const part1 = () => {
  const lines = parseGroups(input)

  const order = getOrder(lines[0])
  const updates = lines[1].map(l => l.split(',').map(Number));
  const validUpdates: number[][] = [];

  for (const update of updates) {
    if (isValidUpdate(update, order) === true) {
      validUpdates.push(update)
    }
  }

  return sum(validUpdates.map(u => u[Math.floor(u.length / 2)]))
}

export const part2 = () => {
  const lines = parseGroups(input)

  const order = getOrder(lines[0])
  const updates = lines[1].map(l => l.split(',').map(Number));
  const validUpdates: number[][] = [];

  for (const update of updates) {
    if (isValidUpdate(update, order) !== true) {
      validUpdates.push(reorderUpdate(update, order))
    }
  }

  return sum(validUpdates.map(u => u[Math.floor(u.length / 2)]))
}

const getOrder = (lines: string[]) => {
  const rules = lines.map(l => l.split('|').map(Number));
  const order: Order = {};

  for (const rule of rules) {
    if (!order[rule[0]]) {
      order[rule[0]] = []
    }
    order[rule[0]].push(rule[1])
  }

  return order
}

const isValidUpdate = (update: number[], order: Order) => {
  const pages: number[] = [];

  for (const page of update) {
    const rule = order[page];

    for (const printedPage of pages) {
      if ((rule || []).includes(printedPage)) {
        return [page, printedPage];
      }
    }

    pages.push(page)
  }

  return true;
}

const reorderUpdate = (update: number[], order: Order) => {

  let reordered = [...update];
  let isValid = isValidUpdate(update, order);

  while (isValid !== true) {
    // swap the wrong pages
    const pos1 = reordered.indexOf(isValid[0])
    const pos2 = reordered.indexOf(isValid[1])

    swapInArray(reordered, pos1, pos2)

    isValid = isValidUpdate(reordered, order)
  }

  return reordered
}
