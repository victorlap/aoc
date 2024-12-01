import { parseLines, readInput } from 'io';

const input = await readInput('day-01')

export const part1 = () => {
  const lists = getSortedLists();
  let answer = 0;

  for (let i = 0; i < lists[0].length; i++) {
    answer += Math.abs(lists[0][i] - lists[1][i]);
  }
 
  return answer
}

const getSortedLists =() => {
  const lines = parseLines(input);
  const lists = [[], []] as Array<Array<Number>>;

  for (let i = 0; i < lines.length; i++) {
    const parts = lines[i].split("   ");

    lists[0][i] = Number(parts[0]);
    lists[1][i] = Number(parts[1]);
  }

  lists[0] = lists[0].sort();
  lists[1] = lists[1].sort();

  return lists;
}
