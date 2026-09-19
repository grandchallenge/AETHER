# Reflex schemas

WP00 defines one candidate structural receipt schema:

- `decision_receipt.schema.json`

Structural validation is not semantic admission. In particular:

- `authority_effect` is fixed to `none`;
- JSON Schema cannot prove that choice names equal a separately versioned
  decision-schema vocabulary;
- JSON Schema cannot prove that distribution probability mass sums to one;
- JSON Schema cannot prove that a referenced AETHER cut, digest, policy or
  authority grant exists;
- semantic validation additionally requires the authority cut to equal the
  decision projection cut;
- structural validation requires an `act` receipt to carry authority evidence,
  while Rust validation remains the authority for cut and choice consistency.

Those checks belong to the Rust `aether_reflex` contract and, later, ordinary
AETHER admission.

Issue: #89.
