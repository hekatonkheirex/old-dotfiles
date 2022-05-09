if [ -n "$DESKTOP_SESSION" ];then
    eval $(gnome-keyring-daemon --start --components=okcs11,secrets,ssh)
    export SSH_AUTH_SOCK
fi

export PATH="$HOME/bin:$PATH"
export PATH="$HOME/.local/bin:$PATH"
export PATH="$HOME/.local/bin:$PATH"
export PATH="$HOME/scripts:$PATH"
export EDITOR=nvim
export _JAVA_AWT_WM_NONREPARENTING=1
# export GTK_THEME=oomox-nord
export BROWSER="/usr/bin/firefox"
export LANG=en_US.UTF-8
export LC_CTYPE=en_US.UTF-8
#export QT_QPA_PLATFORM=wayland-egl
export SDL_VIDEODRIVER=wayland
export XDG_SESSION_TYPE=wayland
export MOZ_ENABLE_WAYLAND=1
export XDG_CURRENT_DESKTOP=sway
export QT_QPA_PLATFORMTHEME=qt5ct
export PYENV_ROOT="$HOME/.pyenv"
export PATH="$PYENV_ROOT/bin:$PATH"
export QT_QPA_PLATFORMTHEME="qt5ct"
