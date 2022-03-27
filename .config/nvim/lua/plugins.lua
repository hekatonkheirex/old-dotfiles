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
  use { 'windwp/nvim-ts-autotag' }
  use { 'p00f/nvim-ts-rainbow' }
  use { 'windwp/nvim-autopairs' }
  use { 'folke/which-key.nvim', config = function()
    require("which-key").setup {
      -- your configuration comes here
      -- or leave it empty to use the default settings
      -- refer to the configuration section below
    }
  end
  }
  use { 'nvim-telescope/telescope.nvim', requires = { {'nvim-lua/plenary.nvim'} } }
  use 'neovim/nvim-lspconfig'
  use 'hrsh7th/cmp-nvim-lsp'
  use 'hrsh7th/cmp-buffer'
  use 'hrsh7th/cmp-path'
  use 'hrsh7th/cmp-cmdline'
  use 'hrsh7th/nvim-cmp'
  use 'hrsh7th/cmp-vsnip'
  use 'hrsh7th/vim-vsnip'
  use { 'onsails/lspkind-nvim' } 
  use { 'norcalli/nvim-colorizer.lua' }
  use { 'folke/twilight.nvim', config = function()
	  require("twilight").setup {
		  -- your configuration comes here
		  -- or leave it empty to use the default settings
		  -- refer to the configuration section below
		  }
		end
	}
use 'elkowar/yuck.vim'



  -- Themes
  use 'Mofiqul/dracula.nvim'
  use { "ellisonleao/gruvbox.nvim" }
  use 'shaunsingh/nord.nvim'
  use 'navarasu/onedark.nvim'
  use 'sainnhe/everforest'
  use 'sainnhe/sonokai'
  use 'ntk148v/vim-horizon'
  use({ 'catppuccin/nvim', as = 'catppuccin' })
  use({ 'rose-pine/neovim', as = 'rose-pine' })
  use 'folke/tokyonight.nvim'
  use 'tanvirtin/monokai.nvim'
end)
