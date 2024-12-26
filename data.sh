#! /bin/bash

for year in $(seq 2024 2024);
do
	dayWithoutZeroes=15;
	for day in $(seq -w 15 25);
	do
		echo "$year $day"
		mkdir -p data/$year/day$day
		touch data/$year/day$day/test.txt
		touch data/$year/day$day/real.txt

		curl -H "Cookie: session=$(cat cookie.txt)" https://adventofcode.com/$year/day/$dayWithoutZeroes > data/$year/day$day/day.html
		curl -H "Cookie: session=$(cat cookie.txt)" https://adventofcode.com/$year/day/$dayWithoutZeroes/input > data/$year/day$day/real.txt
		dayWithoutZeroes=$((dayWithoutZeroes+1));
	done
done
