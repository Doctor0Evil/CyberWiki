# Schema objects

CyberWiki defines JSON schemas for core objects such as terms, documents, and provenance records. These schemas formalize the shape of data exchanged through the API and used in offline tooling, ensuring consistent validation and predictable evolution of the knowledge model.[web:45][web:48][web:58]

External systems can rely on these schemas to construct, validate, or transform CyberWiki entities without depending on internal implementation details of the Rust workspace.
