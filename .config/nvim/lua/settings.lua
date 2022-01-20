HOME = os.getenv("HOME")

-- basic settings
vim.o.encoding = "utf-8"
vim.o.history = 1000

-- Display
vim.o.laststatus = 2

-- Sidebar
vim.o.number = true
vim.o.relativenumber = true

-- White characters
vim.o.autoindent = true
vim.o.smartindent = true
vim.o.tabstop = 2 -- 1 tab = 2 spaces
vim.o.shiftwidth = 2 -- indentation rule
vim.o.formatoptions = 'qnj1' -- q  - comment formatting; n - numbered lists; j - remove comment when joining lines; 1 - don't break after one-letter word
vim.o.expandtab = true -- expand tab to spaces

vim.o.termguicolors = true
vim.o.background = 'light'
vim.o.mouse = 'a'
vim.o.cursorline = false
vim.o.cursorcolumn = false

vim.g.node_host_prog = '/usr/local/bin/neovim-node-host'
vim.g.python3_host_prog = '/usr/bin/python3'
vim.g.python_host_prog = '/usr/bin/python2'
vim.g.ruby_host_prog = '/home/mura/.gem/ruby/2.7.0/bin/neovim-ruby-host'
vim.g.node_host_prog = '/usr/bin/neovim-node-host'
