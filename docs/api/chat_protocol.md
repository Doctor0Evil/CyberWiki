# Chat integration protocol

CyberWiki’s chat integration protocol defines how AI-Chat front ends translate user intentions into structured queries against the knowledge graph. Requests should identify the target domain, entity type, or taxonomy scope alongside natural-language prompts to ensure that responses remain tightly aligned with the documentation corpus.[web:20][web:64]

Responses from CyberWiki are markdown fragments that already follow the project’s style and editorial rules, allowing AI-Chat systems to forward them with minimal post-processing. Implementations must not inject speculative content or alter technical statements in ways that change their meaning.
