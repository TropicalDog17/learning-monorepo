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
  - (Optional) Go implementations of distributed algorithms

## Available Talent Plan Tracks

This roadmap incorporates materials from these Talent Plan tracks:

1. **[Rust Track](./talent-plan/courses/rust/README.md)**: Building practical networked applications in Rust

   - Detailed lesson plan: [talent-plan/courses/rust/docs/lesson-plan.md](./talent-plan/courses/rust/docs/lesson-plan.md)
   - Progressive projects building a key-value store

2. **[Distributed Systems Track](./talent-plan/courses/dss/README.md)**: Implementing Raft and Percolator

   - Raft consensus algorithm: [talent-plan/courses/dss/raft/README.md](./talent-plan/courses/dss/raft/README.md)
   - Percolator transaction model: [talent-plan/courses/dss/percolator/README.md](./talent-plan/courses/dss/percolator/README.md)

3. **[Go Track](./talent-plan/tidb/README.md)** (Optional): Distributed systems in Go
   - Parallel merge sort: [talent-plan/tidb/mergesort/README.md](./talent-plan/tidb/mergesort/README.md)
   - MapReduce implementation: [talent-plan/tidb/mapreduce/README.md](./talent-plan/tidb/mapreduce/README.md)
   - Parallel join: [talent-plan/tidb/join/README.md](./talent-plan/tidb/join/README.md)

## Month 1 – Rust Refresh + ZK Basics

Rust / Talent Plan

- [ ] Ownership, Lifetimes, Smart Pointers (Arc, Mutex, RefCell)
  - Reference: [Rust Book Chapters 4, 10, and 15](https://doc.rust-lang.org/book/)
- [ ] Error Handling and Testing Patterns
  - Checklist: [talent-plan/courses/rust/projects/project-1/README.md](./talent-plan/courses/rust/projects/project-1/README.md) (Error handling section)
- [x] Project 1: File I/O KV Store
  - Full instructions: [talent-plan/courses/rust/projects/project-1/README.md](./talent-plan/courses/rust/projects/project-1/README.md)
- [ ] Project 2: Memory Management
  - Full instructions: [talent-plan/courses/rust/projects/project-2/README.md](./talent-plan/courses/rust/projects/project-2/README.md)

ZK Learning

- [ ] Arithmetic Circuits and Finite Fields
- [ ] Circom Hello World
- [ ] Password Verifier
- [ ] Age Checker
- [ ] Less Than Comparator

Go Track (Optional)

- [ ] Parallel Merge Sort Implementation
  - Instructions: [talent-plan/tidb/mergesort/README.md](./talent-plan/tidb/mergesort/README.md)
  - Optimize with pprof profiling
  - Outperform standard sort.Slice()

## Month 2 – Networking + Intermediate ZK

Rust / Talent Plan

- [ ] Project 3: TCP Client-Server Networking
  - Full instructions: [talent-plan/courses/rust/projects/project-3/README.md](./talent-plan/courses/rust/projects/project-3/README.md)
- [ ] Thread Pool and Concurrency Handling
  - Reference: [talent-plan/courses/rust/projects/project-4/README.md](./talent-plan/courses/rust/projects/project-4/README.md)
- [ ] Logging and CLI Argument Parsing
  - Examples in [talent-plan/courses/rust/projects/project-1/README.md](./talent-plan/courses/rust/projects/project-1/README.md)

ZK Learning

- [ ] Circom Templates and Loop Constraints
- [ ] Number in Range
- [ ] Sudoku Validator (optional)
- [ ] Merkle Tree Membership Proof

Go Track (Optional)

- [ ] MapReduce Framework Implementation
  - Instructions: [talent-plan/tidb/mapreduce/README.md](./talent-plan/tidb/mapreduce/README.md)
  - Implement URL Top 10 extractor
  - Optimize for different data distributions
  - Profile and document performance improvements

## Month 3 – Async + Raft Intro + zkSNARK Concepts

Rust / Talent Plan

- [ ] Async Refactor using Tokio
  - Full instructions: [talent-plan/courses/rust/projects/project-5/README.md](./talent-plan/courses/rust/projects/project-5/README.md)
- [ ] Project 4: Async KV Server
  - Reference: [talent-plan/courses/rust/building-blocks/bb-5.md](./talent-plan/courses/rust/building-blocks/bb-5.md)
- [ ] Read: Raft Paper
  - Paper link: [Raft Paper](https://raft.github.io/raft.pdf)
  - Course materials: [talent-plan/courses/dss/raft/README.md](./talent-plan/courses/dss/raft/README.md)

ZK Learning

- [ ] Groth16 Concepts and R1CS to QAP
- [ ] Lagrange Interpolation and Schwartz-Zippel
- [ ] Vote Validity (ZK Voting)
- [ ] Tornado-style Mini Mixer

Go Track (Optional)

- [ ] Parallel Join Implementation
  - Instructions: [talent-plan/tidb/join/README.md](./talent-plan/tidb/join/README.md)
  - Implement an efficient join algorithm
  - Optimize performance with profiling
  - Document optimization process

## Month 4 – Raft + Smart Contract Integration

Rust / Talent Plan

- [ ] Raft 2A: Leader Election and Heartbeats
  - Detailed instructions: [talent-plan/courses/dss/raft/README.md](./talent-plan/courses/dss/raft/README.md) (Part 2A section)
- [ ] Raft 2B: Log Replication and Commit Index
  - Detailed instructions: [talent-plan/courses/dss/raft/README.md](./talent-plan/courses/dss/raft/README.md) (Part 2B section)
- [ ] Raft 2C: Persistence and State Machine Snapshots
  - Detailed instructions: [talent-plan/courses/dss/raft/README.md](./talent-plan/courses/dss/raft/README.md) (Part 2C section)

ZK Learning

- [ ] Solidity Verifier (snarkjs or circom-verify-template)
- [ ] ZK ERC-20 Private Transfer (Nullifier and Commitment)
- [ ] Deploy verifier contract to testnet
- [ ] Optional: Noir or arkworks proof generator

Go Track (Optional)

- [ ] Compare Go vs Rust Implementations
  - Document performance differences between languages
  - Analyze code organization patterns in both ecosystems
  - Reflect on concurrency models (goroutines vs async/await)

## Month 5 – Percolator + Advanced ZK

Rust / Talent Plan

- [ ] Read Percolator Paper
  - Paper link: [Percolator Paper](https://storage.googleapis.com/pub-tools-public-publication-data/pdf/36726.pdf)
  - Course materials: [talent-plan/courses/dss/percolator/README.md](./talent-plan/courses/dss/percolator/README.md)
- [ ] Timestamp Oracle Implementation
  - Reference: [talent-plan/courses/dss/percolator/README.md](./talent-plan/courses/dss/percolator/README.md) (TSO section)
- [ ] Distributed Transaction Engine (2PC and Conflict Resolution)
  - Full instructions: [talent-plan/courses/dss/percolator/README.md](./talent-plan/courses/dss/percolator/README.md)

ZK Learning

- [ ] XOR Classifier (ZKML Toy, optional)
- [ ] Complete Mixer Withdrawal Logic and Nullifier Checks
- [ ] Integrate Circuit with CLI or Web UI
- [ ] Start zk-SNARK verifier in Rust (arkworks)

Go Track (Optional)

- [ ] Explore TiDB Architecture (Go-based)
  - Study TiDB/TiKV interaction patterns
  - Analyze how Go is used in a large distributed system
  - Consider how to apply lessons to your Rust implementations

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

Go Track (Optional)

- [ ] Open PR to TiDB (Go-based components)
  - Consider documentation improvements
  - Fix small issues in Go codebase
  - Apply distributed systems concepts learned in both languages

## Adjusting for a Full-Time Job

If you have a full-time job and can't commit to 30 hours/week:

- **Extend the timeline**: Consider a 12-month plan instead of 6 months
- **Focus on weekends**: Do major implementation work on weekends
- **Daily small steps**: Spend 1-2 hours on weeknights for reading and planning
- **Prioritize**: Choose either Rust/Distributed Systems or ZK track to focus on first
- **Drop optional Go track**: Consider removing the Go track to focus on Rust if time is limited
- **Track progress**: Check off items as you complete them to maintain momentum

## Folder Structure Suggestion

learning-monorepo/
├── rust-projects/
│ └── kvstore-raft/
├── zk-projects/
│ ├── password-verifier/
│ ├── mixer-circom/
│ ├── token-transfer/
│ └── merkle-proof/
├── go-projects/ (optional)
│ ├── mergesort/
│ ├── mapreduce/
│ └── join/
├── smart-contracts/
│ └── zkVerifier.sol
├── talent-plan/ (cloned repo)
│ ├── courses/
│ │ ├── rust/
│ │ └── dss/
│ └── tidb/
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
- Talent Plan: [Rust Track](./talent-plan/courses/rust/README.md)
- Talent Plan: [Distributed Systems Track](./talent-plan/courses/dss/README.md)
- Talent Plan: [Go Track](./talent-plan/tidb/README.md) (Optional)
