# Dave's cpp prime finder refactored

Refactored Dave's cpp code to be less c and more cpp for fun and to see if it would change the performance of it. The context for this comes from this
[source video](https://www.youtube.com/watch?v=D3h62rgewZM).

On a 3900x at 4.0GHz dave's original code ran for about ~15300 passes in 10 seconds.  
Code was reformatted with clang format, separated into its own file with header, made less use of pointers and stack values.

1. Using `bitcount == n/2` with dividing index and using `vector.at(index)` runs at ~14300
2. Removing the dividing of index, trading space for less operations it runs at ~15200
3. Removing the `vector.at(index)` which does a bounds check further increases the throughput to 17700 iterations in 10 seconds
