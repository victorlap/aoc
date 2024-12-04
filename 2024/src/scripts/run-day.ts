import chalk from 'chalk'
import { formatDay, formatDayName, formatPerformance, validateDay, withPerformance } from 'utils/script'

const runDay = async (day: number) => {
  if (!validateDay(day)) {
    console.log(`🎅 Pick a day between ${chalk.bold(1)} and ${chalk.bold(25)}.`)
    console.log(`🎅 To get started, try: ${chalk.cyan('bun day 1')}`)
    return
  }

  const file = Bun.file(`./src/${formatDayName(day)}/index.ts`)
  const fileExists = await file.exists()

  if (!fileExists) {
    console.log(chalk.red(`Day ${formatDay(day)} does not exist!`))
    return
  }

  const { part1, part2 } = await import(`../${formatDayName(day)}/index.ts`)

  const [one, onePerformance] = withPerformance(() => part1?.())
  const [two, twoPerformance] = withPerformance(() => part2?.())

  console.log(
    '🌲',
    'Part One:',
    chalk.green(one ?? '—'),
    one ? `(${formatPerformance(onePerformance)})` : ''
  )
  console.log(
    '🎄',
    'Part Two:',
    chalk.green(two ?? '—'),
    two ? `(${formatPerformance(twoPerformance)})` : ''
  )
}

const day = Number(Bun.argv[2] ?? '') || new Date().getDate()
runDay(day)