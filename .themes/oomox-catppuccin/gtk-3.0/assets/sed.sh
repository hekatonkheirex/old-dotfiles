#!/bin/sh
sed -i \
         -e 's/#1E1D2F/rgb(0%,0%,0%)/g' \
         -e 's/#d2d4de/rgb(100%,100%,100%)/g' \
    -e 's/#C9CBFF/rgb(50%,0%,0%)/g' \
     -e 's/#ABE9B3/rgb(0%,50%,0%)/g' \
     -e 's/#131020/rgb(50%,0%,50%)/g' \
     -e 's/#d2d4de/rgb(0%,0%,50%)/g' \
	"$@"
