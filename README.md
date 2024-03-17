<h1>Tic-Tac-Toe Agent Using MCTS in Rust</h1>
<h2>Project Overview</h2>

This project is an implementation of a tic-tac-toe game agent that uses Monte Carlo Search Trees (MCTS) for decision-making. Developed in Rust, the aim for me was to

<ol>
  <li>solidify my Rust knowledge which I learned a month prior to starting this project</li>
  <li>learn about MCTS</li>
</ol>

<h2>Features</h2>

<ul>
  <li><b>MCTS Algorithm:</b> Employs Monte Carlo Search Trees for robust and efficient game decision-making.</li>
  <li><b>Rust Implementation:</b> Built with Rust for potential of efficient memory management and performance.</li>
  <li><b>Interactive Game:</b> Allows users to play against the AI in a simple terminal interface.</li>
</ul>

<h2>Getting Started</h2>
<h3>Prerequisites<h3>
Ensure you have Rust installed on your system. You can install Rust via <a href="https://rustup.rs">rustup</a>.

<h3>Installation</h3>
Clone the repository:

```
git clone git@github.com:Fadi987/game_strategies.git
```

Navigate to the project directory:

```
cd game_strategies
```

Build the project:

```
cargo build --release
```

Run the game:

```
cargo run

```

(Optional) Generate the docs:

```
cargo doc --open
```

<h3>How To Play</h3>

You'll be playing as O against the AI agent, which will be X. At every turn for O, you can specify the cell in the 3x3 board to mark by `(row_index, col_index)`, where `(0,0)` is the top left cell and `(2,2)` is the bottom right.

<h3>How It Works</h3>
The MCTS algorithm is a heuristic search algorithm used for making decisions in a given domain by taking random samples in the decision space and building a search tree according to the results. This approach is particularly well-suited for games like tic-tac-toe, where a finite and discrete set of moves exists.
<br></br>

At any given iteration, we keep track of a search tree in-memory where every node represents a visited game state. And a parent-child relationship indicates that the child game state can be reached from the parent game state in one move. Note that a leaf node does not mean that the game is over at the leaf; it just means that the leaf is the lastly visited game state along a path of moves from the root node as of the current iteration.

<h4>MCTS Phases</h4>
MCTS consists of four main phases in each iteration:
<br></br>
<ol>
<li><b>Selection:</b> Starting from the root node, the algorithm selects child nodes one-by-one until it reaches a leaf node. The selection is based on the Upper Confidence Bound (UCB) plicy applied to trees, which balances exploration and exploitation.</li>

<li><b>Expansion:</b> Unless the leaf node ends the game (win/lose/draw), we add all child nodes to expand the search tree, based on the possible moves from the game state of the previously-leaf node.</li>

<li><b>Simulation:</b> For each newly-added child node, the algorithm simulates random game plays (also known as playouts or rollouts) from the child's game state until a game conclusion is reached. This step is crucial for understanding the potential of each move without exhaustively searching the entire space.</li>

<li><b>Backpropagation:</b> Finally, the results of the simulation are propagated back up the tree, updating the statistics of the nodes visited during the selection phase. This update informs future selections by improving the accuracy of the win/loss estimates.</li>

</ol>
