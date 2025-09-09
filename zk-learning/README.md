# ZK Learning Plan (12 Months)

This folder tracks the detailed ZK learning stream integrated into the main roadmap. Use it as a checklist with references and project scaffolds.

## Quarter 1 (Months 1–3): Conceptual Grounding with Light DSL Work

- Week 1–2 — Noir sprint
  - Learn syntax, pipeline: Noir → ACIR → backend
  - Deploy a proof to a Solidity verifier (testnet)
  - Concepts: abstraction layers; how ACIR generalizes constraint systems
- Week 3–8 — Halo2 core
  - Regions, advice/fixed columns, custom gates
  - Polynomial commitments and validity
  - Concepts: what constraint systems are; PLONKish vs R1CS
- Week 9–12 — End-to-end DSL application
  - Private voting app
  - Concepts: public vs private signals; witness handling; verifier succinctness

## Quarter 2 (Months 4–6): Protocol & System Concepts

- Month 4 — Circuit optimization
  - Benchmark Poseidon vs SHA-256
  - Concepts: constraint count → prover complexity; native-friendly primitives
- Month 5 — Rollup simulation
  - Build a toy rollup
  - Concepts: data availability vs validity; Merkle trees as commitments; why ZK enables rollup scalability
- Month 6 — Benchmarking
  - Compare Circom, Noir, Halo2; summarize tradeoffs
  - Concepts: verification gas vs assumptions (pairings vs FRI); trusted setup vs transparent proofs

## Quarter 3 (Months 7–9): Advanced ZK Concepts

- Month 7 — Recursive proofs
  - Small Halo2 recursion demo
  - Concepts: recursion compresses proofs; aggregation vs recursion
- Month 8 — Identity proofs
  - ZK login; age/region proofs
  - Concepts: nullifiers; selective disclosure
- Month 9 — ZK integrations
  - Prototype zk light client verifier
  - Concepts: zk bridges need full consensus proofs; fraud vs validity proofs

## Quarter 4 (Months 10–12): Production & Research Literacy

- Month 10 — Security & auditing
  - Audit your circuits for under/over-constraint
  - Concepts: Fiat–Shamir heuristic and risks
- Month 11 — Protocol literacy
  - Read Groth16, PLONK, Halo2, STARKs (conceptual)
  - Concepts: SNARKs vs STARKs; Halo2 position
- Month 12 — Capstone
  - Mini rollup + recursive proofs
  - Light zkVM demo (Risc0 SHA-256)
  - Concepts: DSLs vs zkVMs; future: zkML, zk bridges, universal verifiers
