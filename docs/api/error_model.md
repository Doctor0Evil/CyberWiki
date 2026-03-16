# Error model

The CyberWiki API uses a structured error model with stable codes and machine-readable fields. Each error response includes an identifier, a human-readable message, and optional context fields describing validation failures, authorization issues, or unavailable resources.[web:3][web:59]

Clients should branch on error codes rather than parsing messages, enabling robust handling of conditions such as invalid schema instances, rejected content due to editorial rules, or attempts to access unsupported operations in a read-only deployment.
