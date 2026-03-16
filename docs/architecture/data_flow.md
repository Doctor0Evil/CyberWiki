# Data flow model

CyberWiki ingests structured term and document entities into an internal knowledge graph, associates them with taxonomy nodes, and links each entity to provenance records describing sources and attestations. Query engines read from this graph to produce deterministic responses to AI-Chat and API requests.[web:12][web:14]

Validation rules run before insertion or update, ensuring that each new or modified entity satisfies editorial, structural, and blacklist constraints. Storage and indexing layers provide persistence and auxiliary lookup structures but do not alter the semantic content of the corpus.

Presentation adapters render responses into markdown fragments which preserve the technical detail and structure imposed by the core model.
