#! /bin/sh

export EDITOR=nvim
export QT_QPA_PLATFORMTHEME="qt5ct"
export GTK_IM_MODULE=fcitx
export QT_IM_MODULE=fcitx
export XMODIFIERS=@im=fcitx
fcitx5 &
xinput --set-prop "ELAN1300:00 04F3:3087 Touchpad" "libinput Natural Scrolling Enabled" 1 &
#setxkbmap -option grp:alt_shift_toggle us,es &
xsetroot -cursor_name left_ptr &
feh --bg-fill /mnt/windows/Walls/Nix/Arch\ Linux\ \(Text\ Purple\).png
picom -f -b --config /home/mura/.config/picom/picom_not_rounded.conf
#picom -f -b --config /home/mura/.config/picom/picom.conf
$HOME/.config/scripts/thunar.sh &
xss-lock -l $HOME/.config/scripts/lock.sh & 
blueberry-tray &
xfce4-power-manager &
nm-applet &
gnome-keyring-daemon --start --components=pkcs11 &                                   
