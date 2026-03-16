# CyberWiki architecture overview

CyberWiki is implemented as a Rust workspace with a core domain crate, a command-line interface crate, and an HTTP API crate. The core crate holds the knowledge graph, taxonomy, and provenance layers that encode the non-fictional corpus.[web:3][web:37]

The CLI crate provides scripted access to validation and query operations, while the API crate exposes read-only and controlled interfaces for AI-Chat integrations. All layers follow a clear separation between domain, application, infrastructure, and presentation concerns.

Data flows are designed to be deterministic and auditable, supporting traceability for ALN/KYC/DID-compliant environments and quantum-resilient governance systems.
