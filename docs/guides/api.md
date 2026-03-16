# API usage guide

The CyberWiki API exposes a constrained HTTP interface intended for AI-Chat systems and tooling that require deterministic, documentation-focused responses. Clients should treat the API as a read-oriented gateway into the knowledge graph, issuing structured queries and receiving markdown-rendered content rather than free-form narratives.[web:3][web:30][web:64]

API consumers are expected to cache responses where appropriate, respect any published rate limits, and avoid attempts to circumvent editorial constraints. Write or update operations, if enabled in a given deployment, must use authenticated channels and conform to the same validation and provenance requirements that govern direct repository contributions.
