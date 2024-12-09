type Transform<Res, Input = string> = (s: Input) => Res
export type Direction = [-1, 0] | [0, 1] | [1, 0] | [0, -1]

export class Grid<T = string> {
  points: Point<T>[][];

  constructor(points: Point<T>[][]) {
    this.points = points;
  }

  static of<T = string>(lines: string[], splitter = '', as?: Transform<T>) {
    return new Grid(
      lines.map((row, y) =>
        row.split(splitter).map((col, x) =>
          new Point(y, x, as ? as(col) : col as T))))
  }

  get(y: number, x: number) {
    return this.points?.[y]?.[x]
  }

  set(y: number, x: number, value: T) {
    return this.points[y][x].value = value
  }

  move(p: Point<T>, dir: Direction) {
    return this.get(p.y + dir[0], p.x + dir[1])
  }

  * allPoints() {
    for (const row of this.points) {
      for (const col of row) {
        yield col
      }
    }
  }

  toString() {
    return this.points.map(row => row.map(col => col.toString()).join('')).join("\n") + "\n"
  }

  clone() {
    return Grid.of(this.toString().split("\n"))
  }
}

export class Point<T> {
  y: number;
  x: number;
  value: T;

  constructor(y: number, x: number, value: T) {
    this.y = y;
    this.x = x;
    this.value = value;
  }

  toString() {
    return this.value
  }

  debug() {
    return `(${this.y},${this.x}): ${this.value}`
  }

  eq(other: Point<T>) {
    return other.y === this.y && other.x === this.x
  }
}
