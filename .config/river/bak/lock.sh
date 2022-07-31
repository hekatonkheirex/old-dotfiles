#!/bin/sh 

pkill swayidle
sleep 1s 

exec swayidle -w \
        timeout 600 'swaylock -f' \
        timeout 1800 'swaymsg "output * dpms off"' \
        resume 'swaymsg "output * dpms on"' \
        before-sleep 'playerctl pause' \
        before-sleep 'swaylock -f'
