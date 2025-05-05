# Learning Roadmap: Rust Distributed Systems + Zero-Knowledge Proofs (30 hrs/week, 6 months)

This roadmap combines the PingCAP Talent Plan and practical zero-knowledge proof (ZK) projects to help you level up to a mid-level Rust developer with deep understanding of distributed systems and zk-SNARK applications.

## Overview

- Total Time: 6 months
- Weekly Time: ~30 hours
- Goal: Build real distributed systems in Rust (Raft + Percolator) and deploy practical ZK applications (Groth16, Circom)
- Deliverables:
  - Rust-based distributed KV store with Raft
  - ZK projects: password verifier, mixer, token transfer, Merkle proof
  - Solidity verifier and frontend/CLI
  - PRs to TiDB/TiKV

## Available Talent Plan Tracks

This roadmap incorporates materials from these Talent Plan tracks:

1. **[Rust Track](./courses/rust/README.md)**: Building practical networked applications in Rust

   - Detailed lesson plan: [courses/rust/docs/lesson-plan.md](./courses/rust/docs/lesson-plan.md)
   - Progressive projects building a key-value store

2. **[Distributed Systems Track](./courses/dss/README.md)**: Implementing Raft and Percolator

   - Raft consensus algorithm: [courses/dss/raft/README.md](./courses/dss/raft/README.md)
   - Percolator transaction model: [courses/dss/percolator/README.md](./courses/dss/percolator/README.md)

3. **[Go Track](./tidb/README.md)** (Optional): Distributed systems in Go
   - Complementary if you want to compare Rust and Go implementations

## Month 1 – Rust Refresh + ZK Basics

Rust / Talent Plan

- [ ] Ownership, Lifetimes, Smart Pointers (Arc, Mutex, RefCell)
  - Reference: [Rust Book Chapters 4, 10, and 15](https://doc.rust-lang.org/book/)
- [ ] Error Handling and Testing Patterns
  - Checklist: [courses/rust/projects/project-1/README.md](./courses/rust/projects/project-1/README.md) (Error handling section)
- [ ] Project 1: File I/O KV Store
  - Full instructions: [courses/rust/projects/project-1/README.md](./courses/rust/projects/project-1/README.md)
- [ ] Project 2: Memory Management
  - Full instructions: [courses/rust/projects/project-2/README.md](./courses/rust/projects/project-2/README.md)

ZK Learning

- [ ] Arithmetic Circuits and Finite Fields
- [ ] Circom Hello World
- [ ] Password Verifier
- [ ] Age Checker
- [ ] Less Than Comparator

## Month 2 – Networking + Intermediate ZK

Rust / Talent Plan

- [ ] Project 3: TCP Client-Server Networking
  - Full instructions: [courses/rust/projects/project-3/README.md](./courses/rust/projects/project-3/README.md)
- [ ] Thread Pool and Concurrency Handling
  - Reference: [courses/rust/projects/project-4/README.md](./courses/rust/projects/project-4/README.md)
- [ ] Logging and CLI Argument Parsing
  - Examples in [courses/rust/projects/project-1/README.md](./courses/rust/projects/project-1/README.md)

ZK Learning

- [ ] Circom Templates and Loop Constraints
- [ ] Number in Range
- [ ] Sudoku Validator (optional)
- [ ] Merkle Tree Membership Proof

## Month 3 – Async + Raft Intro + zkSNARK Concepts

Rust / Talent Plan

- [ ] Async Refactor using Tokio
  - Full instructions: [courses/rust/projects/project-5/README.md](./courses/rust/projects/project-5/README.md)
- [ ] Project 4: Async KV Server
  - Reference: [courses/rust/building-blocks/bb-5/README.md](./courses/rust/building-blocks/bb-5/README.md) (if exists)
- [ ] Read: Raft Paper
  - Paper link: [Raft Paper](https://raft.github.io/raft.pdf)
  - Course materials: [courses/dss/raft/README.md](./courses/dss/raft/README.md)

ZK Learning

- [ ] Groth16 Concepts and R1CS to QAP
- [ ] Lagrange Interpolation and Schwartz-Zippel
- [ ] Vote Validity (ZK Voting)
- [ ] Tornado-style Mini Mixer

## Month 4 – Raft + Smart Contract Integration

Rust / Talent Plan

- [ ] Raft 2A: Leader Election and Heartbeats
  - Detailed instructions: [courses/dss/raft/README.md](./courses/dss/raft/README.md) (Part 2A section)
- [ ] Raft 2B: Log Replication and Commit Index
  - Detailed instructions: [courses/dss/raft/README.md](./courses/dss/raft/README.md) (Part 2B section)
- [ ] Raft 2C: Persistence and State Machine Snapshots
  - Detailed instructions: [courses/dss/raft/README.md](./courses/dss/raft/README.md) (Part 2C section)

ZK Learning

- [ ] Solidity Verifier (snarkjs or circom-verify-template)
- [ ] ZK ERC-20 Private Transfer (Nullifier and Commitment)
- [ ] Deploy verifier contract to testnet
- [ ] Optional: Noir or arkworks proof generator

## Month 5 – Percolator + Advanced ZK

Rust / Talent Plan

- [ ] Read Percolator Paper
  - Paper link: [Percolator Paper](https://storage.googleapis.com/pub-tools-public-publication-data/pdf/36726.pdf)
  - Course materials: [courses/dss/percolator/README.md](./courses/dss/percolator/README.md)
- [ ] Timestamp Oracle Implementation
  - Reference: [courses/dss/percolator/README.md](./courses/dss/percolator/README.md) (TSO section)
- [ ] Distributed Transaction Engine (2PC and Conflict Resolution)
  - Full instructions: [courses/dss/percolator/README.md](./courses/dss/percolator/README.md)

ZK Learning

- [ ] XOR Classifier (ZKML Toy, optional)
- [ ] Complete Mixer Withdrawal Logic and Nullifier Checks
- [ ] Integrate Circuit with CLI or Web UI
- [ ] Start zk-SNARK verifier in Rust (arkworks)

## Month 6 – TiDB, Contribution and Portfolio Finalization

Rust / Talent Plan

- [ ] TiDB Architecture and Local Deployment
  - Reference: [TiDB Documentation](https://docs.pingcap.com/tidb/stable/quick-start-with-tidb)
- [ ] TiKV Storage Engine and Raft Logic
  - Reference: [TiKV Documentation](https://tikv.org/docs/deep-dive/introduction/)
- [ ] Open PR to TiDB or TiKV (Docs or Tests)
  - TiDB GitHub: [https://github.com/pingcap/tidb](https://github.com/pingcap/tidb)
  - TiKV GitHub: [https://github.com/tikv/tikv](https://github.com/tikv/tikv)

ZK Final Projects

- [ ] Finalize CLI or Frontend for one ZK use case
- [ ] Write blog post or README walkthrough
- [ ] Polish zk-project repo with test cases and Solidity verifier

## Adjusting for a Full-Time Job

If you have a full-time job and can't commit to 30 hours/week:

- **Extend the timeline**: Consider a 12-month plan instead of 6 months
- **Focus on weekends**: Do major implementation work on weekends
- **Daily small steps**: Spend 1-2 hours on weeknights for reading and planning
- **Prioritize**: Choose either Rust/Distributed Systems or ZK track to focus on first
- **Track progress**: Check off items as you complete them to maintain momentum

## Folder Structure Suggestion

root/
├── rust-projects/
│ └── kvstore-raft/
├── zk-projects/
│ ├── password-verifier/
│ ├── mixer-circom/
│ ├── token-transfer/
│ └── merkle-proof/
├── smart-contracts/
│ └── zkVerifier.sol
├── docs/
│ └── zk-architecture.md
└── README.md

## Resources

- Rust Book: https://doc.rust-lang.org/book/
- Rust Async Book: https://rust-lang.github.io/async-book/
- Raft Paper: https://raft.github.io/raft.pdf
- Percolator Paper: https://storage.googleapis.com/pub-tools-public-publication-data/pdf/36726.pdf
- RareSkills ZK Book: https://www.rareskills.io/zero-knowledge-proofs
- TiDB Docs: https://docs.pingcap.com/tidb/stable
- Talent Plan: [Rust Track](./courses/rust/README.md)
- Talent Plan: [Distributed Systems Track](./courses/dss/README.md)
- Talent Plan: [Go Track](./tidb/README.md) (Optional)
