return require('packer').startup(function()
  use 'wbthomason/packer.nvim' -- Packer self-manager

  -- Common
--  use 'vim-airline/vim-airline' -- powerline
--  use 'vim-airline/vim-airline-themes'
--  use 'powerline/powerline'
--  use 'itchyny/lightline.vim'
  use { 'nvim-lualine/lualine.nvim', requires = { 'kyazdani42/nvim-web-devicons', opt = true }}
  use { 'nvim-treesitter/nvim-treesitter', run = ':TSUpdate' }
  use { 'akinsho/bufferline.nvim', requires = 'kyazdani42/nvim-web-devicons' }
  use { 'kyazdani42/nvim-tree.lua', requires = { 'kyazdani42/nvim-web-devicons' }, config = function()
    require'nvim-tree'.setup {}
  end
  }

  -- Themes
  use({ 'dracula/vim', as = 'dracula' })
  use { "ellisonleao/gruvbox.nvim" }
  use 'shaunsingh/nord.nvim'
  use 'navarasu/onedark.nvim'
  use 'sainnhe/everforest'
  use 'sainnhe/sonokai'
  use 'ntk148v/vim-horizon'
  use({ 'catppuccin/nvim', as = 'catppuccin' })
  use({ 'rose-pine/neovim', as = 'rose-pine' })
  use 'folke/tokyonight.nvim'

end)
