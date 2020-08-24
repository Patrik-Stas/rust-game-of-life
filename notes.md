# Ideas
## 1st version: 2dboard
- we keep 2d Vector of booleans `live`/`dead`.
- difficult to scale to infinity
- even if we have just few sparse cell alive, we still have to iterate row*cols times on every tick
- could be parallelized by sharing the vector between threads

## 2nd version
- we keep vector of cells
- each cell knows its location
- it's hard to tell if a cell has a neighbour (we could do some sorting within, 
but then updates will be expensive)

## 3rd version hashstate
- we keep hashmap of cells 
- the key is tuple of 2 coordinates
- it's O(1) to tell if there is a cell on certain position
- step1: each update we iterate all cells and their neighbours, count number of alive neighbours in counting
sort style
- step2: then we iterate over this "has-a-neighbour" datastructure and determine who should be alive, who
should be dead and update cell hashmap accordingly
- step3: repeat step1
- Should be parallelizable - each thread can iterate part of original cell hashmap and generate candidates,
then also reviving/killing cells in shared cell hashmap.

# GOL Interface
Any of these solutions should have certain shared interface:
- new                        ## creates empty gol
- load (x, y, gol_interface) ## loads goal board relative to position (x,y)
- make cell alive (x, y)     ## make cell (x,y) alive
- make cell dead (x, y)      ## make cell (x,y) dead
- update (n_iters)           ## run GOL update
- iter                       ## returns iterator over all alive cells

# Gol loader
Should be module capable of reading/generating patterns and setting them into GOL Interface.

# Gol visualizer
- visualize(gol_iterator)