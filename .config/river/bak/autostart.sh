#!/bin/sh

killall -q polkit-gnome-authentication-agent-1 waybar mako

# Polkit
/usr/lib/polkit-gnome/polkit-gnome-authentication-agent-1 &

# Wallpaper
swaybg -m fill -i /mnt/windows/Walls/anime-girl-silhouette-sunset-scenery-anime-art-hd-wallpaper-uhdpaper.com-618@0@f.jpg &

# Waybar
waybar -c ~/.config/river/waybar/config -s ~/.config/river/waybar/style.css &

# Notification
mako &

# Manage idle
# swayidle -w \
#   timeout 600 'swaylock -f' \
#   timeout 1800 'swaybg "output * dpmw off"' \
#   resume 'swaybg "output * dpms on"' \
#   before-sleep 'playerctl pause' \
#   before-sleep 'swaylock -f'
#
