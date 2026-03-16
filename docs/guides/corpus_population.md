# Corpus directory population guide

The `data/` directory is the default location for the CyberWiki corpus, as referenced by the `corpus.root_dir` setting in `assets/config/default.toml`. This directory holds serialized representations of terms, documents, taxonomy entries, and provenance records that the CyberWiki core and API use to construct the in-memory knowledge graph at startup.[web:41][web:44][web:105]

A typical deployment organizes corpus data into structured subdirectories such as `terms/`, `documents/`, `taxonomy/`, and `provenance/`, with each file containing a single JSON entity conforming to the corresponding schema under `assets/schemas/`. Filenames are usually derived from entity identifiers or stable slugs, allowing deterministic resolution from identifiers to on-disk objects.

For example, a minimal corpus might include:

- `data/terms/bci_motor_imagery_primary.json` containing a term describing a specific motor imagery BCI paradigm.  
- `data/documents/bci_motor_imagery_overview.md.json` containing a document that details its architecture, pipelines, and safety profile.  
- `data/taxonomy/bci_domain_root.json` defining a taxonomy node for BCI-related entities.  
- `data/provenance/bci_motor_imagery_primary_provenance.json` recording sources and attestations for the term.

Corpus population should follow these steps:

1. Draft entities in a separate working area, ensuring that each term, document, taxonomy node, and provenance record is strictly non-fictional and corresponds to real devices, protocols, datasets, or governance artifacts.  
2. Validate each JSON instance against the respective JSON Schema using standard tooling before committing it into `data/`.  
3. Run the CyberWiki CLI validation command to confirm that the populated corpus satisfies structural and editorial rules.  
4. Commit the new files with concise, technically focused messages that reference the domains and entities affected.

By adhering to this process and directory structure, deployments maintain a clear separation between Rust code, configuration, and the curated, audit-ready corpus that powers CyberWiki’s AI-anchored documentation behavior.[web:41][web:44][web:105]
