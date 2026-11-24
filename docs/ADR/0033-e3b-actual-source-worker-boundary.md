# ADR 0033 — Actual source ownership at a non-operative worker boundary

**Status:** Candidate, bounded #148 preparation. No live routing or authority change.
**Predecessor:** protected main `95c96da490abfd21b1e5b5e3f2276c3d19fb8657`.
**Basis:** ADR 0029, #146; C3 classification #130 closed through PRs #165/#167.

## Source ownership

| Fact | Actual owner / revision | Limitation |
| --- | --- | --- |
| Token, principal, scope, namespace grant | HttpAuth mutex; checked generation on successful replace_config | Not durable process identity |
| Token-default effective policy | HttpAuth grant plus bound_policy_context | Request-specific policy requires exact requested context and actual semantic binding |
| Namespace admission | namespace_admission map and OwnedSemaphorePermit | Capacity reservation, not semantic permission |
| Namespace service state | NamespaceServiceDirectory and per-handle mutex | No independently demonstrated lifecycle or journal revocation epoch |
| C1 envelope | Issuer and EnvelopeControlRegistry | Live HTTP and C1 owners are separate |
| Resource and worker | BoundedBlockingExecutor admission and worker semaphores | Worker possession is not authorization |

Reload validates configuration before mutation and locks status then auth.
The existing HttpAuth revision covers policy grants and namespace membership
as well as token and scope. Reuse it instead of adding simulated counters.
A requested policy is a value computed from a grant, not an independent
epoch. Namespace service lifecycle is a different source; expose any required
state explicitly or deny rather than inventing a revision from auth generation.

## Implemented experiment

The cfg(test) module source_owner_actual uses real authorization,
token-default policy binding, rate admission, namespace permit and actual
global admission/worker semaphores. A changed local configuration file
drives actual validated reload (status -> auth), including token revocation,
scope or namespace membership removal and policy-context change.
An unchanged reload still advances generation.

A separate actual C1 issuer/registry is instantiated inside the fixture.
Its admission flag is explicitly test-only: authentic auth/rate/namespace
checks do not prove semantic admission. The typed RunDocumentRequest is
not executed. The real shadow comparator and pure realization produce a
RouteRealized record from the local pool observation. A private,
non-serializable probe retains that exact record and weak identities of
the actual source and worker owners. Its one-use diagnostic bit consumes
even denied inspections.

A held worker stages actual queue admission. Releasing it starts a real
blocking callback; a second barrier holds the callback before inspection.
Both barriers allow validated reload and actual C1 revoke/supersede before
the hypothetical start. Under the real auth mutex then fixture C1 mutex,
inspection rechecks the exact principal, token, namespace, effective policy,
generation and current C1 revision/state. Expiry uses the wall clock and
a monotonic elapsed bound. Contended/poisoned locks deny via try_lock.
No lock crosses await or semantic work. The unchanged case returns only
MatchNonOperative; tampered route, fresh owner, replay and expiry deny.
No production code can consume this result as authorization.

## Prospective migration, NOT implemented

Acquire rate, namespace, queue and worker resources outside a short owner
transaction. The future owner must encompass the real auth grant,
current C1 capability/registry, exact request/profile/payload/effective
policy and required authoritative namespace lifecycle facts. Prefer one
actor/owner to independent locks. Migrate reload so status publication
follows owner commit without status -> auth -> owner -> status cycles.
If multiple guards remain, establish one global order and enumerate every
mutation path; any inverse or unknown path denies integration. No guard
may cross await, selector/audit I/O, namespace initialization or semantic
execution.

At a worker barrier authenticate issuer-owned envelope and exact
RouteRealized attempt, endpoint and resource lease, source generation,
policy/namespace binding, current C1 revision and expiry. Atomically consume
a unique ledger entry before returning an irrevocable start disposition.
Reload/revoke/supersede/cancel/expire must linearize against that entry.
Define start as this commit point; revocation after commit is not
retrospective cancellation. If semantics instead require revoke to win
until the first physical semantic instruction, an unlocked handoff cannot
guarantee it; that requires a stronger source-defined transaction.

The one-use test bit is not a global issuer ledger or operative permit.
A real identity binds process incarnation, issuer/envelope/authorization,
exact typed payload and operation profile, scope, namespace, principal/token,
effective policy, source generation, placement, lease and mechanical attempt.
A deserialized event may not recreate a capability. Restart either recovers
an authenticated consumption ledger and clock continuity or refuses old
attempts. Weak Arc identity only distinguishes owners in one process.
The monotonic bound and rejection of a pre-issuance wall timestamp do not
prove a trusted clock. Cancellation/finish remains separately tested in #143.

## Classification

NO_ARTICLE_IX_AUTHORITY_CHANGE for this test-only delta. No production
API, semantic callback, journal, route mode, resource domain or permission
change. #148 genuine-source probe tranche can be protected, but full
source-current consumptive start remains PARTIAL. Next specify the
authoritative namespace lifecycle contract and exact-delta owner/ledger
migration. Operative changes require fresh Article IX/X/XI classification
and any reserved dispositions. #133 and E3B-A remain open.
