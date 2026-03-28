# Grant Tracker

---

## Table of Contents
- [Project Title](#grant-tracker)
- [Project Description](#project-description)
- [Project Vision](#project-vision)
- [Key Features](#key-features)
- [Future Scope](#future-scope)

---

## Project Description

**Grant Tracker** is a decentralised smart contract built on the **Stellar blockchain** using the **Soroban SDK**. It provides a transparent and tamper-proof system for managing the full lifecycle of grant applications — from submission by applicants, through admin approval, to final disbursement and closure.

All grant data (title, description, requested amount, recipient, status flags, and creation timestamp) is stored on-chain, giving every stakeholder — applicants, administrators, and auditors — a single source of truth that cannot be altered retroactively.

---

## Project Vision

The traditional grant management process is often opaque, slow, and prone to human error or fraud. Funds can be misdirected, approvals delayed without explanation, and records falsified.

**Grant Tracker** envisions a world where grant funding is:

- **Transparent** — every action (submission, approval, disbursement) is recorded on-chain and publicly auditable.
- **Trustless** — rules are enforced by code, not by institutional goodwill.
- **Accessible** — any organisation or individual can deploy their own grant programme without relying on centralised intermediaries.
- **Accountable** — aggregate statistics (total, approved, pending, completed) give administrators and the public an always-up-to-date picture of fund flows.

By bringing grant management on-chain, this contract lays the groundwork for a more equitable and efficient funding ecosystem for research, public-goods projects, NGOs, and decentralised autonomous organisations (DAOs).

---

## Key Features

| # | Feature | Description |
|---|---------|-------------|
| 1 | **Grant Submission** | Any user can submit a grant proposal with a title, description, requested amount, and recipient identifier. An auto-incremented unique grant ID is returned on creation. |
| 2 | **Admin Approval** | A designated administrator can approve any pending grant, transitioning its status from `pending → approved` and updating the global stats counter atomically. |
| 3 | **Grant Completion / Disbursement** | Once a grant is approved, the admin can mark it as `completed`, indicating that funds have been disbursed and the grant lifecycle is closed. |
| 4 | **Per-Grant Query** | Any stakeholder can fetch the full details of a specific grant using its unique ID — status flags, amount, recipient, and ledger timestamp included. |
| 5 | **Aggregate Statistics** | A dedicated view function returns platform-wide counters for total, approved, pending, and completed grants, enabling real-time dashboards and audits. |
| 6 | **On-chain Data Integrity** | All records are stored in Soroban instance storage with TTL extension on every write, ensuring data persistence without centralised databases. |
| 7 | **Guard Clauses & Panic Safety** | Each state-mutating function validates pre-conditions (e.g., a grant cannot be approved twice, cannot be completed before approval) and panics with descriptive messages on invalid transitions. |

---

## Future Scope

The current contract delivers a minimal, auditable grant lifecycle. The following enhancements are planned for future iterations:

- **Role-Based Access Control (RBAC)** — Introduce explicit admin and applicant address verification so only authorised wallets can call approval and completion functions, preventing unauthorised state changes.

- **Multi-Signature Approval** — Require M-of-N admin signatures before a grant is marked approved, reducing single-point-of-failure risk for high-value grants.

- **Milestone-Based Disbursement** — Split a single grant into multiple milestones, each requiring separate approval before the next tranche is released, improving accountability for recipients.

- **On-chain Voting for Grant Selection** — Allow token holders or DAO members to vote on competing grant proposals, replacing centralised admin discretion with community governance.

- **Native Token Integration** — Connect the contract to Stellar's native asset or a custom SAC (Stellar Asset Contract) token so that `complete_grant` triggers an automatic on-chain transfer of funds to the recipient's wallet.

- **Rejection & Appeal Workflow** — Add an explicit `reject_grant` function with a reason field, and an `appeal_grant` path allowing applicants to resubmit with revisions.

- **Event Emission** — Emit structured Soroban events on every state change so off-chain indexers and frontends can react in real time without polling storage.

- **Cross-Contract Interoperability** — Expose a standard interface so other Soroban contracts (e.g., treasury or DAO contracts) can programmatically create or query grants.

---
## Contracts Details :
Contract ID : CAPOIDBSGRZVPTHPU2AGORWJCAGFVCNOR67ZUK4TF3G3KMUAWF4WPCD3
<img width="1868" height="979" alt="image" src="https://github.com/user-attachments/assets/c81a1203-0663-4869-8dd3-7cabc62f5cae" />

> Built with ❤️ using [Soroban SDK](https://soroban.stellar.org/) on the Stellar blockchain.
