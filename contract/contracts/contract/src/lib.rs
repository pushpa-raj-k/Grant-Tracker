#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short};

// Tracks aggregate stats across all grants on the platform
#[contracttype]
#[derive(Clone)]
pub struct GrantStats {
    pub total: u64,     // Total number of grants submitted
    pub approved: u64,  // Total grants approved
    pub pending: u64,   // Total grants awaiting approval
    pub completed: u64, // Total grants marked as completed/disbursed
}

// Symbol key used to store and retrieve GrantStats from instance storage
const ALL_GRANTS: Symbol = symbol_short!("ALL_GRNT");

// Counter key — incremented every time a new grant is created
const COUNT_GRANT: Symbol = symbol_short!("C_GRANT");

// Enum used as a unique storage key per grant, keyed by grant ID
#[contracttype]
pub enum Grantbook {
    Grant(u64),
}

// Represents a single grant record as created by an applicant
#[contracttype]
#[derive(Clone)]
pub struct Grant {
    pub grant_id: u64,       // Auto-assigned unique ID
    pub title: String,       // Short title of the grant proposal
    pub description: String, // Detailed description of the grant purpose
    pub amount: u64,         // Requested funding amount (in smallest token unit)
    pub recipient: String,   // Name / address of the grant recipient
    pub approved: bool,      // Whether the admin has approved this grant
    pub completed: bool,     // Whether the grant has been disbursed / closed
    pub created_at: u64,     // Ledger timestamp at the moment of creation
}

#[contract]
pub struct GrantTrackerContract;

#[contractimpl]
impl GrantTrackerContract {

    // ── FUNCTION 1 ────────────────────────────────────────────────────────────
    // Create a new grant application.
    // Called by an applicant; returns the auto-assigned grant_id.
    pub fn create_grant(
        env: Env,
        title: String,
        description: String,
        amount: u64,
        recipient: String,
    ) -> u64 {
        // Increment the global grant counter to derive a fresh unique ID
        let mut count: u64 = env
            .storage()
            .instance()
            .get(&COUNT_GRANT)
            .unwrap_or(0);
        count += 1;

        let timestamp = env.ledger().timestamp();

        // Build the Grant record
        let grant = Grant {
            grant_id: count,
            title,
            description,
            amount,
            recipient,
            approved: false,
            completed: false,
            created_at: timestamp,
        };

        // Update aggregate stats
        let mut stats = Self::view_all_grant_stats(env.clone());
        stats.total += 1;
        stats.pending += 1;

        // Persist everything to instance storage
        env.storage()
            .instance()
            .set(&Grantbook::Grant(count), &grant);
        env.storage().instance().set(&ALL_GRANTS, &stats);
        env.storage().instance().set(&COUNT_GRANT, &count);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Grant created with ID: {}", count);
        count // Return the new grant's unique ID to the caller
    }

    // ── FUNCTION 2 ────────────────────────────────────────────────────────────
    // Approve a pending grant (admin action).
    // Transitions the grant from pending → approved.
    pub fn approve_grant(env: Env, grant_id: u64) {
        let mut grant = Self::view_grant_by_id(env.clone(), grant_id);

        // Guard: only pending (non-approved, non-completed) grants can be approved
        if grant.approved || grant.completed {
            log!(&env, "Grant {} is already approved or completed.", grant_id);
            panic!("Grant is already approved or completed.");
        }

        grant.approved = true;

        let mut stats = Self::view_all_grant_stats(env.clone());
        stats.approved += 1;
        stats.pending = stats.pending.saturating_sub(1);

        env.storage()
            .instance()
            .set(&Grantbook::Grant(grant_id), &grant);
        env.storage().instance().set(&ALL_GRANTS, &stats);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Grant {} has been approved.", grant_id);
    }

    // ── FUNCTION 3 ────────────────────────────────────────────────────────────
    // Mark an approved grant as completed / disbursed (admin action).
    // Transitions the grant from approved → completed.
    pub fn complete_grant(env: Env, grant_id: u64) {
        let mut grant = Self::view_grant_by_id(env.clone(), grant_id);

        // Guard: grant must be approved before it can be marked complete
        if !grant.approved || grant.completed {
            log!(
                &env,
                "Grant {} must be approved before it can be completed, or is already completed.",
                grant_id
            );
            panic!("Grant must be approved and not yet completed.");
        }

        grant.completed = true;

        let mut stats = Self::view_all_grant_stats(env.clone());
        stats.completed += 1;

        env.storage()
            .instance()
            .set(&Grantbook::Grant(grant_id), &grant);
        env.storage().instance().set(&ALL_GRANTS, &stats);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Grant {} marked as completed.", grant_id);
    }

    // ── FUNCTION 4 (VIEW) ─────────────────────────────────────────────────────
    // Retrieve a single grant record by its ID.
    // Returns a default "not found" Grant if the ID does not exist.
    pub fn view_grant_by_id(env: Env, grant_id: u64) -> Grant {
        env.storage()
            .instance()
            .get(&Grantbook::Grant(grant_id))
            .unwrap_or(Grant {
                grant_id: 0,
                title: String::from_str(&env, "Not_Found"),
                description: String::from_str(&env, "Not_Found"),
                amount: 0,
                recipient: String::from_str(&env, "Not_Found"),
                approved: false,
                completed: false,
                created_at: 0,
            })
    }

    // ── FUNCTION 5 (VIEW) ─────────────────────────────────────────────────────
    // Retrieve aggregate stats (total / approved / pending / completed counts).
    pub fn view_all_grant_stats(env: Env) -> GrantStats {
        env.storage()
            .instance()
            .get(&ALL_GRANTS)
            .unwrap_or(GrantStats {
                total: 0,
                approved: 0,
                pending: 0,
                completed: 0,
            })
    }
}