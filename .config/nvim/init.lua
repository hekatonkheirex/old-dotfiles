require('mappings')
require('plugins')
require('settings')
require('treesitter-config')
require('lualine-config')
require('bufferline-config')
require('nvimtree-config')

-- Themes options
--vim.g.gruvbox_contrast_dark = 'medium'
--vim.g.everforest_background = 'hard'
--vim.g.everforest_enable_italic = true
--vim.g.gruvbox_italic = true
--vim.g.nord_italic = true
--vim.g.nord_italic_comments = true
--vim.g.colors_name = "dracula"
--vim.g.onedark_hide_endofbuffer = true
--vim.g.onedark_terminal_italics = true
--vim.g.sonokai_style = 'andromeda'
--vim.g.sonokai_enable_italic = true
--vim.g.sonokai_disable_italic_comment = true
--vim.g.tokyonight_style = "night"
--vim.g.tokyonight_italic_functions = true
--vim.g.tokyonight_sidebars = { "qf", "vista_kind", "terminal", "packer" }
--vim.g.tokyonight_colors = { hint = "orange", error = "#ff0000" }
vim.g.rose_pine_variant = 'dawn'  -- 'main', 'moon', 'dawn'
vim.g.rose_pine_bold_vertical_split_line = false
vim.g.rose_pine_inactive_background = false
vim.g.rose_pine_disable_background = false
vim.g.rose_pine_disable_float_background = false
vim.g.rose_pine_disable_italics = false

-- Themes color loading
--vim.cmd('colorscheme nord')
--vim.cmd('colorscheme dracula')
--vim.cmd('colorscheme gruvbox')
--vim.cmd('colorscheme onedark')
--vim.cmd('colorscheme everforest')
--vim.cmd('colorscheme sonokai')
--vim.cmd('colorscheme tokyonight')
--vim.cmd('colorscheme horizon')
--vim.cmd('colorscheme catppuccin')
vim.cmd('colorscheme rose-pine')
