import chalk from 'chalk'
import type { Day } from 'models'
import { existsSync, PathLike } from 'node:fs'
import { mkdir } from 'node:fs/promises'

type Transform<Res, Input = string> = (s: Input) => Res

export const mkdirIfNotExists = async (dir: PathLike) => {
  if (!existsSync(dir)) {
    await mkdir(dir)
  }
}

export const writeFileIfNotExists = async (filepath: PathLike & Bun.PathLike, input: Blob | NodeJS.TypedArray | ArrayBufferLike | string | Bun.BlobPart[]) => {
  if (!existsSync(filepath)) {
    await Bun.write(filepath, input)
  }
}

export const readFile = async (filepath: string) => {
  const file = Bun.file(filepath)
  const text = await file.text()
  return text.trim()
}

export const readInput = async (dir: Day, fileName?: string) => {
  fileName = fileName || Bun.argv.includes('--test') ? 'test' : 'input'
  const filepath = `./src/${dir}/${fileName}.txt`

  if (!existsSync(filepath)) {
    throw chalk.red.bold(`❌ Input does not exist, please run bun setup <day> first`);
  }

  return readFile(filepath)
}

export const parseLines = <T = string>(
  input: string,
  as?: Transform<T>,
  { includeEmpty }: { includeEmpty?: boolean } = {}
) => {
  let lines = input.split('\n')
  if (!includeEmpty) {
    lines = lines.filter(Boolean)
  }
  return as ? lines.map(as) : lines as T[]
}

export const parseGroups = <T = string>(
  input: string,
  as?: Transform<T>
) => {
  const groups = input.split('\n\n')

  return groups.map((group) => {
    return parseLines(group, as)
  }) as T[][]
}
