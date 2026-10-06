"""AETHER/FABRIC E2 local reference conformance harness.

This package is evidence tooling only. It does not implement or activate FABRIC.
"""

from .harness import E1_SCHEMAS, load_vector_registry, validate_e1_schemas, validate_trace

__all__ = ["E1_SCHEMAS", "load_vector_registry", "validate_e1_schemas", "validate_trace"]
