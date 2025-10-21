# Cryptographic Rock-Paper-Scissors

A networked implementation of rock-paper-scissors that uses cryptographic commitments to prevent cheating in distributed games.

## The Problem

In traditional networked games of rock-paper-scissors, there's a fundamental trust problem:

- If **Player A** sends their choice first, **Player B** can see it and choose accordingly (guaranteed win for Player B)
- If both players send simultaneously, network delays mean one player's choice arrives first
- Even with a trusted third party, you must trust that party won't leak information

**How can two players play a fair game over a network without trusting each other or a third party?**

## The Solution: Cryptographic Commitments

This implementation uses a **commit-reveal protocol** based on cryptographic hashing to solve the cheating problem:

### Phase 1: Commitment
Instead of sending their actual choice, each player sends a **cryptographic hash** of their choice combined with a random secret (called a "nonce").

```
Hash = SHA-256(Play + Nonce)
```

The hash acts as a cryptographic commitment:
- **Binding**: Once committed, the player cannot change their choice (the hash would no longer match)
- **Hiding**: The opponent cannot reverse the hash to discover the choice

### Phase 2: Reveal
After both players have committed (by exchanging hashes), they reveal:
- Their actual play (Rock, Paper, or Scissors)
- Their secret nonce

### Phase 3: Validation
Each player verifies the opponent's commitment:
1. Recompute the hash using the revealed play and nonce
2. Compare it to the hash received in Phase 1
3. If they don't match → the opponent cheated
4. If they match → the game is valid

### Why This Works

- **No premature information leak**: The hash reveals nothing about the actual play
- **Tamper-proof**: Changing the play after commitment would require breaking SHA-256 (computationally infeasible)
- **No trusted third party needed**: The mathematics guarantee fairness
- **Verifiable**: Both players can independently verify no cheating occurred

## Game Protocol

```
Player A (Server)          Network          Player B (Client)
─────────────────────────────────────────────────────────────
1. Choose play + nonce
2. Compute hash            ───────>         Receive hash
                                            Choose play + nonce
3. Receive hash            <───────         Send hash

4. Reveal play + nonce     ───────>         Receive revelation
                                            Verify hash matches
5. Receive revelation      <───────         Reveal play + nonce
   Verify hash matches

6. Determine winner                         Determine winner
```

## Quick Start

### Prerequisites
- Rust toolchain (rustc, cargo)

### Installation

```bash
git clone <repository-url>
cd crypto-rock-paper-scissors
cargo build --release
```

### Running the Game

**Terminal 1 (Server):**
```bash
cargo run
# Select 's' for Server mode
# Choose your play (r/p/s)
```

**Terminal 2 (Client):**
```bash
cargo run
# Select 'c' for Client mode
# Choose your play (r/p/s)
```

The game will:
1. Exchange cryptographic commitments
2. Reveal plays and nonces
3. Validate both players played fairly
4. Display the winner

## Cryptographic Properties

### Security Guarantees

- **Collision Resistance**: SHA-256 makes it computationally infeasible to find two different inputs that produce the same hash
- **Pre-image Resistance**: Cannot reverse the hash to discover the original play
- **Nonce Randomness**: 256-bit random nonce ensures the same play hashes differently each game

### Attack Resistance

- **Replay attacks**: Random nonce prevents reusing old commitments
- **Brute force**: Only 3 possible plays, but 256-bit nonce makes brute force impractical
- **Timing attacks**: Both commitments exchanged before any reveals
- **Man-in-the-middle**: Would be detected during validation phase

## Protocol Details

### Message Format
Messages are JSON-encoded:

**Commitment:**
```json
{"HashedPlay": "a3f5d8..."}
```

**Revelation:**
```json
{"Explanation": {"play": "r", "nonce": "7c8f3d..."}}
```

## Limitations & Future Work

**Current State:**
- Single round per execution
- Local network only (localhost)
- Basic error handling
- No reconnection support

**Potential Enhancements:**
- Best-of-N tournament mode
- Remote play over internet
- Persistent game statistics
- GUI interface
- Multiple simultaneous games
- Configurable network settings

## Educational Value

This project demonstrates practical applications of:
- **Cryptographic commitments** in distributed systems
- **Zero-knowledge protocols** (hiding information until reveal)
- **Byzantine fault tolerance** (playing fairly with untrusted parties)
- **Mental poker** protocols (the same concept used in online card games)

The commit-reveal pattern is used in:
- Blockchain smart contracts (preventing front-running)
- Secure multiparty computation
- Online poker and gambling
- Sealed-bid auctions
- Voting systems

## References

- [Commitment Schemes (Wikipedia)](https://en.wikipedia.org/wiki/Commitment_scheme)
- [Mental Poker (Wikipedia)](https://en.wikipedia.org/wiki/Mental_poker)
