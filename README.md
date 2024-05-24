# cheapnum

A tool to find optimal sequences of arithmetic operations transforming an initial set of numbers into a desired number. Each number has a usage cost associated with it, but operations are free. Intended for use with the game [Beltmatic](https://store.steampowered.com/app/2674590/Beltmatic/).

### Usage

`cheapnum <target> <cost table>`

For example:

`cheapnum 530 1=2 2=2 3=2 4=2 5=2 6=2 7=2 8=3 9=3 11=2 16=3 25=3 36=3`