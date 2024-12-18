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

  static new<T>(height: number, width: number, initializer: (y: number, x: number) => T): Grid<T> {
    return new Grid(
      Array.from({length: height}, (_, y) =>
        Array(width).fill('.').map((_, x) =>
          new Point(y, x, initializer(y, x)))))
  }

  get height() {
    return this.points.length
  }

  get width() {
    return this.points[0]?.length ?? 0
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

  toString(printer: (v: T) => string = (v) => v as string) {
    return this.points.map(row => row.map(col => col.toString(printer)).join('')).join("\n") + "\n"
  }

  clone(cloner: (v: T) => T = (v) => v): Grid<T> {
    return Grid.new(this.points.length, this.points[0].length, (y, x) => cloner(this.points[y][x].value))
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

  toString(printer: (v: T) => string) {
    return printer(this.value)
  }

  debug() {
    return `(${this.y},${this.x}): ${this.value}`
  }

  eq(other: Point<T> | undefined) {
    return other && other.y === this.y && other.x === this.x
  }
}
