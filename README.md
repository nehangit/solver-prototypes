# Cross-Chain Automated Solver for CoW Protocol

## Overview

This project focuses on building a cross-chain automated solver off-chain to efficiently route trading intents to the correct venue or party for execution or fulfillment on-chain. Initially, the solver targets CoW (Coincidence of Wants) batch auctions, aiming to match peer-to-peer orders optimally. The solver will integrate with auction protocols using a Request for Quotes (RFQ) model to submit competitive bids and complete transactions efficiently within tight timeframes.

By extending an open-source solver, this project contributes to the growing decentralized finance (DeFi) ecosystem, addressing challenges like order routing, capital efficiency, and minimizing latency in transaction execution. The system aims to maximize surplus for users, reduce gas fees, and mitigate MEV risks.

---

## Motivation

DeFi's rapid growth necessitates efficient trading solutions. Blockchains present unique challenges, such as:

- Different token standards
- Variations in liquidity pools
- Infrastructure constraints

Solvers play a pivotal role by acting as market makers, executing trades across tokens via protocols and bridges. The solver in this project specifically targets CoW protocol batch auctions, optimizing order surplus to benefit users and the protocol.

---

## Features

### Auction Solvers

- **Batch Auctions**: Match orders directly when possible; fallback to AMMs or liquidity pools when necessary.
- **Surplus Maximization**: Optimize the surplus function for limit buy and sell orders.
- **Gas and MEV Efficiency**: Avoid gas fees for canceled trades and minimize MEV risks.

### Solution Graph Visualization

- **Order Routing**: Dynamically compute optimal routes for cross-chain trading.
- **Auction Integration**: Participate in RFQ-based auctions with competitive bids.
- **Latency Reduction**: Respond within 10-second batch auction windows.
- **Combinatorial Problem Solving**: Solve algebraic problems to maximize efficiency.

---

## Technical Details

### Surplus Calculation and Objective Function

The surplus for an order \( o \) is defined as:

\[ U(x, -y) = \begin{cases}
(x - y \pi)p(b) & \text{for limit sell orders} \\
(x \pi - y)p(s) & \text{for limit buy orders}
\end{cases} \]

where:

- \( x \): Amount of token to buy
- \( y \): Amount of token to sell
- \( \pi \): Price ratio provided by the user
- \( p(b), p(s) \): Normalized prices of the bought/sold tokens

The solution quality \( Q \) is the sum of surpluses over all orders plus protocol fees:

\[ Q = \sum_o U(o) + \sum_o p \cdot f(o) \]

The solver maximizes \( Q \) to win auctions.

### Naive Solver

### Provided Liquidity Immediate Matching (Luke's Solver)

### Gecko Terminal On Chain Liquidity Solver (Phillip's Solver)

### Bonus: Nonlinear Programming

### Architecture

- **Programming Languages**:
  - **Python**: For high-level logic, testing, and performance analysis.
- **Server**: Hosted on Heroku, implementing CoW protocol-required endpoints.
- **Optimization Algorithms**: Includes Bellman-Ford for graph-based optimization.
- **Testing and Simulation**: Locally simulate batch auctions and compare solutions.

### Tools and Libraries

- Open-source solvers for CoW protocol:
  - [Solver Template](https://github.com/cowprotocol/solver-template-py)
  - [Searcher CoWswap](https://github.com/autistic-symposium/searcher-cowswap-py)
- CoW Protocol Documentation:
  - [CoW Docs](https://docs.cow.fi/cow-protocol)
  - [The Problem](https://docs.cow.fi/cow-protocol/reference/core/auctions/the-problem)

---

## Milestones and Timeline

### Phase 1: Research and Setup

- **11/16 - 11/20**
  - Set up Heroku server and basic graph functionality.
  - Explore Bellman-Ford Algorithm.
  - Refactor core files for new auction schema.

### Phase 2: Refactoring and POC Development

- **11/21 - 11/29**
  - Implement Bellman-Ford Proof of Concept (POC).
  - Refactor `spread_solver.py` and integrate with Heroku endpoints.
  - Debug and integrate functionality for batch auctions.

### Phase 3: Testing and Optimization

- **11/30 - 12/9**
  - Create simulation for batch auctions and test surplus calculations.
  - Implement and test limit buy orders.
  - Begin shadow competition integration.

### Phase 4: Finalization and Visualization

- **12/10 - 12/17**
  - Develop visualizations for graph transactions in real-time.
  - Optimize and parallelize code.
  - Fully integrate with CoW protocol and finalize for competition.

---

## Server Requirements

The Heroku server must:

1. Provide a health check endpoint returning `True`.
2. Accept a JSON file with:
   - Orders
   - Available AMM liquidity in the current batch
3. Respond with a JSON file containing:
   - Order routing solutions
   - Pricing details

### Local Testing

1. Use CoW protocol's shadow competition tools.
2. Simulate batch auctions and verify surplus calculations.

---

## Usage Instructions

### Cloning the Repository

```bash
git clone <repository-url>
cd <repository-folder>
```

### Setting Up the Environment

1. Install Python dependencies:
   ```bash
   pip install -r requirements.txt
   ```
2. Deploy the server locally or on Heroku:
   ```bash
   heroku deploy
   ```

### Running the Solver

1. Start the server:
   ```bash
   python main.py
   ```
2. Test endpoints using sample batch auction data.

### Visualizations

Run the visualization script to see real-time graph transactions:

```bash
python visualize.py
```

---

## Contributing

1. Fork the repository.
2. Create a feature branch:
   ```bash
   git checkout -b feature-name
   ```
3. Commit your changes:
   ```bash
   git commit -m "Description of changes"
   ```
4. Push to the branch:
   ```bash
   git push origin feature-name
   ```
5. Open a Pull Request.

---

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

---

## Acknowledgments

- [CoW Protocol Documentation](https://docs.cow.fi/cow-protocol)
- [Solver Template Repository](https://github.com/cowprotocol/solver-template-py)
- [Searcher CoWswap Repository](https://github.com/autistic-symposium/searcher-cowswap-py)
