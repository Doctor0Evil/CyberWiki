# CyberWiki

CyberWiki is a Rust-based documentation and assistance workspace dedicated to non-fictional, professionally structured knowledge about cybernetics, brain–computer interfaces, EEG systems, human–machine interfaces, and nanoswarm architectures.

The project is designed as an AI-aligned documentation stack where all interactive behavior is constrained to technically grounded definition, identification, and indexing of high-performance systems. Conversational flows are treated as documentation edits and queries against a curated knowledge base rather than open-ended dialogue.

## Scope and editorial rules

1. CyberWiki excludes any content derived from home-grown computer-biotics, improvised bio-digital constructs, or unverifiable backyard experimental systems. Any submission that downgrades, contaminates, or speculates on such materials is rejected at the curation layer by design.
2. Outputs are formatted as if authored by a clear-minded technical editor preparing a professional research dossier. No illustrative metaphors, speculative narratives, or dramatized titles are permitted.
3. All terminology, math, and geography or biophysics references are normalized to standard scientific usage before being accepted into the corpus.
4. Example fragments, functions, and object descriptions are always given as real-world, implementation-facing structures suitable for integration with regulated, production-grade systems.

## Crate layout

The Rust workspace is organized into a `core` crate for the domain model and query engine, an `api` crate for network integration, and a `cli` crate for local interaction and batch tooling. This layout allows CyberWiki to run as a headless knowledge engine, a network service, or a command-line assistant depending on deployment constraints.

## Non-fiction and governance

CyberWiki aligns with ALN/KYC/DID-ready identities, cryptographic audit trails, and quantum-resilient governance principles by modeling provenance, multi-signature attestations, and transaction-linked document histories as first-class entities inside the knowledge graph.
