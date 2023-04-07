import { readdir } from 'fs/promises'
import { fileURLToPath, URL } from 'node:url'
import fs from 'node:fs'
import { execa } from 'execa'
import inquirer from 'inquirer'
import { magentaBright, bold } from "colorette"

const execaArgs = ['--config', 'vite.config.js']

export const commandRunner = (args, options) => {
  if (args && args.length === 0) {
    // 如果没有参数，则让选择项目
    chooseOneRun(options)
  } else {
    args.forEach(async (arg) => {
      console.log(`> current business project identification：${bold(magentaBright(arg))}`)
      await run(arg, options)
    })
  }
}

export const run = async (arg, options) => {
  try {
    console.log('> starting...')
    if (!arg) {
      throw new Error('no business project name')
    }
    if (!(await hasProject(arg))) {
      throw new Error(`'${arg}' is not a business project or business project not found`)
    }
    await execa(options.command, [options.order, `packages/${arg}`, ...execaArgs], { stdio: 'inherit' })
  } catch (err) {
    console.error(err)
  }
}

const chooseOneRun = async (options) => {
  const projectList = await getBusinessFolder();
  inquirer.prompt([
    {
      type: 'list',
      message: `select the business project to execute the ${options.order} command：`,
      name: 'mono',
      default: projectList[0],
      choices: projectList.map(p => p),
    }
  ]).then(async ({ mono: projectName }) => {
    console.log(`> current business project identification：${bold(magentaBright(projectName))}`)

    await run(projectName, options)
  })
}

export const hasProject = async (name) => {
  let projectList = []
  if (projectList.length === 0) {
    projectList = await getBusinessFolder()
  }
  if (projectList.includes(name)) return true
  return false
}

export const getBusinessFolder = async () => {
  try {
    const folders = await readdir(fileURLToPath(new URL('../packages', import.meta.url)))
    let businessFolders = []
    folders.forEach(folder => {
      if (!/^common/.test(folder)) {
        businessFolders.push(folder)
      }
    })
    return businessFolders
  } catch (e) {
    console.error(e)
  }
}

export const matchViteArgv = (argv) => {
  return argv.some(it => /^packages/.test(it))
}

export const createVscodeViteAutoConfig = async () => {
  const list = await getBusinessFolder()
  return list[0]
}

/**
 * @description 判断文件是否存在
 */
export const isFileExist = (path) => {
  if (fs.existsSync(path)) {
    return true
  } else {
    return false
  }
}

export const importViteOptions = async(projectName, command = '') => {
  let px2remOptions = {}
  let devServer = {}
  const settingPath = `./packages/${projectName}/setting.js`
  const serverPath = `./packages/${projectName}/server.js`
  if (isFileExist(settingPath)) {
    px2remOptions = await import(settingPath).then((module) => module.px2remOptions)
  }
  if (command === 'serve' && isFileExist(serverPath)) {
    devServer = await import(serverPath).then((module) => module.default)
  }

  return {px2remOptions, devServer}
}