#!/bin/sh
sed -i \
         -e 's/#2B2D3A/rgb(0%,0%,0%)/g' \
         -e 's/#E1E3E4/rgb(100%,100%,100%)/g' \
    -e 's/#2B2D3A/rgb(50%,0%,0%)/g' \
     -e 's/#BB97EE/rgb(0%,50%,0%)/g' \
     -e 's/#2B2D3A/rgb(50%,0%,50%)/g' \
     -e 's/#E1E3E4/rgb(0%,0%,50%)/g' \
	"$@"
