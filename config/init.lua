logo_colour = "#94e2d5"

normalPalette = colour.black("● ")
              ..colour.red("● ")
              ..colour.green("● ")
              ..colour.yellow("● ")
              ..colour.blue("● ")
              ..colour.purple("● ")
              ..colour.cyan("● ")
              ..colour.white("● ")
boldPalette = colour.boldBlack("● ")
            ..colour.boldRed("● ")
            ..colour.boldGreen("● ")
            ..colour.boldYellow("● ")
            ..colour.boldBlue("● ")
            ..colour.boldPurple("● ")
            ..colour.boldCyan("● ")
            ..colour.boldWhite("● ")

stats = {
  "Username         : " .. user.name,
  "Window Manager   : " .. wm.name,
  "Vendor name      : " .. vendor.name,
  "Vendor model     : " .. vendor.model,
  "Uptime           : " .. uptime.uptime,
  "Terminal         : " .. terminal.name,
  "Shell            : " .. shell.name,
  "OS Name          : " .. os.name,
  "OS Version       : " .. os.version,
  "OS Hostname      : " .. os.hostname,
  "Kernel Version   : " .. os.kernel,
  "Total Memory     : " .. string.format("%.1fgb", mem.total),
  "Used Memory      : " .. string.format("%.1fgb", mem.used),
  "Graphics Adaptor : " .. gpu.name,
  "Total Disk Space : " .. string.format("%.1fgb", disk.total),
  "Used Disk Space  : " .. string.format("%.1fgb", disk.used),
  "CPU Name         : " .. cpu.name,
  "CPU Frequency    : " .. cpu.freq .. " GHz",
  "Palette          : " .. normalPalette,
  "Bold Palette     : " .. boldPalette,
}
