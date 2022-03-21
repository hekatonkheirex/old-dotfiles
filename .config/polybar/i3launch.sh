#!/usr/bin/env sh

# Terminate already running bar instances
killall -q polybar

# Wait until the processes have been shut down
while pgrep -u $UID -x polybar >/dev/null; do sleep 1; done

# Launch bars
polybar -rq i3 -c ~/.config/polybar/i3config.ini &
polybar -rq spotify -c ~/.config/polybar/i3config.ini &
polybar -rq info -c ~/.config/polybar/i3config.ini &
