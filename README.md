# A Connect 4 Solver

This project performs a full game tree search of Connect 4 using the negamax algorithm to determine optimal moves.

## Optimizations

### Alpha-Beta Pruning

This project uses alpha-beta pruning to quickly discard suboptimal states, significantly reducing the search space and improving efficiency.

### Custom Transposition Table

A custom HashMap implementation is used to manage the caching of game states. Traditional hash maps consume a prohibitive amount of memory when storing all possible states. To address this, the custom implementation discards data when the map becomes too full, preventing memory exhaustion.

### Perfect Hashing

Perfect hashing optimizes the hash map by ensuring the unique identification of keys. The custom hashing function also purposefully generates collisions for board states that are guaranteed to store the same value. For example, flipping the board horizontally does not change the hash.

### Search Ordering

Heuristics are employed to prioritize evaluating the most promising moves first. This significant improves the effectivness of alpha-beta-pruning Each square has a weight indicating its desirability for play, along with additional weights for features like encouraging three-in-a-row formations. The heuristic weights are determined using linear regression.

### Bitboards

The project makes extensive use of bitboards to accelerate move generation and evaluation heuristics.
