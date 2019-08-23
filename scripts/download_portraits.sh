#!/bin/bash

# This could be made a helper of the main app, where it could be much more
# foolproof, but this is good-enough for now


HEROS="
dva orisa reinhardt roadhog sigma winston wrecking-ball zarya ashe bastion doomfist genji
hanzo junkrat mccree mei pharah reaper soldier-76 sombra symmetra torbjorn tracer widowmaker
ana baptiste brigitte lucio mercy moira zenyatta"

OUTPUT=../assets/images/overwatch/portraits

for hero in $HEROS; do
	 wget https://d1u1mce87gyfbn.cloudfront.net/hero/${hero}/icon-portrait.png -O ${OUTPUT}/${hero}.png
done
