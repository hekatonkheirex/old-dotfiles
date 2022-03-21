#!/bin/sh
sed -i \
         -e 's/#1E1D2F/rgb(0%,0%,0%)/g' \
         -e 's/#D9E0EE/rgb(100%,100%,100%)/g' \
    -e 's/#1E1D2F/rgb(50%,0%,0%)/g' \
     -e 's/#C9CBFF/rgb(0%,50%,0%)/g' \
     -e 's/#302D41/rgb(50%,0%,50%)/g' \
     -e 's/#D9E0EE/rgb(0%,0%,50%)/g' \
	"$@"
