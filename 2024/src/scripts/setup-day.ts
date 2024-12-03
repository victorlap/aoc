import chalk from 'chalk'
import { mkdirIfNotExists, writeFileIfNotExists } from 'io'
import { existsSync } from 'node:fs'
import { isBetween } from 'utils'
import { formatDay, formatDayName, generateTemplate, validateDay } from 'utils/script'
import { fetchInput } from './api'

const setupDay = async (day: number) => {
  if (!validateDay(day)) {
    console.log(`🎅 Pick a day between ${chalk.bold(1)} and ${chalk.bold(25)}.`)
    console.log(`🎅 To get started, try: ${chalk.cyan('bun setup 1')}`)
    return
  }

  const dir = new URL(`../${formatDayName(day)}/`, import.meta.url)
  const inputFile = new URL(`input.txt`, dir.href)
  const dayFile = new URL(`index.ts`, dir.href)

  if (existsSync(inputFile) && existsSync(dayFile)) {
    console.log(chalk.red(`Day ${day} already exists!`))
    return
  }

  if (existsSync(dayFile)) {
    console.log(`Day ${day} already exists, only retrieving input`)
  }

  const currentYear = new Date().getFullYear()
  const year = Number.parseInt(Bun.env.YEAR ?? currentYear.toString())

  if (Number.isNaN(year) || !isBetween(year, [2015, currentYear])) {
    console.log(
      chalk.red(
        `📅 Year must be between ${chalk.bold(2015)} and ${chalk.bold(currentYear)}.`
      )
    )
    return
  }

  console.log('📄 Fetching input...')
  const input = await fetchInput({ day, year }).catch(() => {
    console.log(
      chalk.red.bold(
        '❌ Fetching input failed, empty file will be created.'
      )
    )
  })

  console.log(`📂 Setting up day ${formatDay(day)}...`)

  try {
    await mkdirIfNotExists(dir)
    await writeFileIfNotExists(inputFile, input ?? '')
    await writeFileIfNotExists(dayFile, generateTemplate(day))

    console.log(chalk.green.bold(`✅ Day ${formatDay(day)} set up!`))
  } catch (err) {
    console.error(chalk.red(err instanceof Error ? err.message : 'Failed to set up day'))
  }
}

const day = Number(Bun.argv[2] ?? '') || new Date().getDate()
setupDay(day)
