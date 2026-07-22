# compute_units_profile

**Source:** [`tests/compute_units.rs` L16](https://github.com/cds-turbin3/vesting-position/blob/7172eb8fb4ae69b1e7af671b76095a6112c41e68/babelfish-tests/tests/compute_units.rs#L16)

Over 31 days: 2 moments.

### T0: Initialize (day 0)

| observation | before | after |
| --- | --- | --- |
| Vault balance | — | 10,000,000,000,000 |
| Creator balance | — | 0 |

### Sequence

```mermaid
sequenceDiagram
    participant p0 as Creator
    participant p1 as VestingPositions
    participant p2 as system
    participant p3 as splAssociatedTokenAccount
    participant p4 as token
    participant p5 as mplCoreProgram
    p0->>p1: Initialize
    activate p1
    p1->>p2: createAccount
    activate p2
    p2-->>p1: ✓
    deactivate p2
    p1->>p3: create
    activate p3
    p3->>p4: getAccountDataSize
    activate p4
    p4-->>p3: ✓ 183cu
    deactivate p4
    p3->>p2: createAccount
    activate p2
    p2-->>p3: ✓
    deactivate p2
    p3->>p4: initializeImmutableOwner
    activate p4
    p4-->>p3: ✓ 38cu
    deactivate p4
    p3->>p4: initializeAccount3
    activate p4
    p4-->>p3: ✓ 235cu
    deactivate p4
    p3-->>p1: ✓ 18017cu
    deactivate p3
    p1->>p5: CreateCollectionV2
    activate p5
    p5->>p2: createAccount
    activate p2
    p2-->>p5: ✓
    deactivate p2
    p5->>p2: transferSol
    activate p2
    p2-->>p5: ✓
    deactivate p2
    p5->>p2: transferSol
    activate p2
    p2-->>p5: ✓
    deactivate p2
    p5->>p2: transferSol
    activate p2
    p2-->>p5: ✓
    deactivate p2
    p5-->>p1: ✓ 19976cu
    deactivate p5
    p1->>p4: transferChecked
    activate p4
    p4-->>p1: ✓ 105cu
    deactivate p4
    p1-->>p0: ✓ 89713cu
    deactivate p1
```

<details>
<summary>tree</summary>

```

Creator (89713cu)
└─ VestingPositions::Initialize ✓ 89713cu
   ├─ system::createAccount ✓
   ├─ splAssociatedTokenAccount::create ✓ 18017cu
   │  ├─ token::getAccountDataSize ✓ 183cu
   │  ├─ system::createAccount ✓
   │  ├─ token::initializeImmutableOwner ✓ 38cu
   │  └─ token::initializeAccount3 ✓ 235cu
   ├─ mplCoreProgram::CreateCollectionV2 ✓ 19976cu
   │  ├─ system::createAccount ✓
   │  ├─ system::transferSol ✓
   │  ├─ system::transferSol ✓
   │  └─ system::transferSol ✓
   └─ token::transferChecked ✓ 105cu
```

</details>

*2678401 seconds pass.*

### T1: FirstClaim (day 31)

| observation | before | after |
| --- | --- | --- |
| Vault balance | 10,000,000,000,000 | 9,000,000,000,000 |

### Sequence

```mermaid
sequenceDiagram
    participant p0 as Alice
    participant p1 as computeBudget
    participant p2 as VestingPositions
    participant p3 as splAssociatedTokenAccount
    participant p4 as token
    participant p5 as system
    participant p6 as mplCoreProgram
    p0->>p1: setComputeUnitLimit
    activate p1
    p1-->>p0: ✓
    deactivate p1
    p0->>p2: Claim
    activate p2
    p2->>p3: create
    activate p3
    p3->>p4: getAccountDataSize
    activate p4
    p4-->>p3: ✓ 183cu
    deactivate p4
    p3->>p5: createAccount
    activate p5
    p5-->>p3: ✓
    deactivate p5
    p3->>p4: initializeImmutableOwner
    activate p4
    p4-->>p3: ✓ 38cu
    deactivate p4
    p3->>p4: initializeAccount3
    activate p4
    p4-->>p3: ✓ 235cu
    deactivate p4
    p3-->>p2: ✓ 13416cu
    deactivate p3
    p2->>p5: createAccount
    activate p5
    p5-->>p2: ✓
    deactivate p5
    p2->>p6: CreateV2
    activate p6
    p6->>p5: createAccount
    activate p5
    p5-->>p6: ✓
    deactivate p5
    p6->>p5: transferSol
    activate p5
    p5-->>p6: ✓
    deactivate p5
    p6->>p5: transferSol
    activate p5
    p5-->>p6: ✓
    deactivate p5
    p6->>p5: transferSol
    activate p5
    p5-->>p6: ✓
    deactivate p5
    p6->>p5: transferSol
    activate p5
    p5-->>p6: ✓
    deactivate p5
    p6-->>p2: ✓ 29342cu
    deactivate p6
    p2->>p4: transferChecked
    activate p4
    p4-->>p2: ✓ 105cu
    deactivate p4
    p2->>p6: UpdatePlugin
    activate p6
    p6-->>p2: ✓ 13574cu
    deactivate p6
    p2-->>p0: ✓ 186680cu
    deactivate p2
```

<details>
<summary>tree</summary>

```

Alice (186830cu)
├─ computeBudget::setComputeUnitLimit ✓
└─ VestingPositions::Claim ✓ 186680cu
   ├─ splAssociatedTokenAccount::create ✓ 13416cu
   │  ├─ token::getAccountDataSize ✓ 183cu
   │  ├─ system::createAccount ✓
   │  ├─ token::initializeImmutableOwner ✓ 38cu
   │  └─ token::initializeAccount3 ✓ 235cu
   ├─ system::createAccount ✓
   ├─ mplCoreProgram::CreateV2 ✓ 29342cu
   │  ├─ system::createAccount ✓
   │  ├─ system::transferSol ✓
   │  ├─ system::transferSol ✓
   │  ├─ system::transferSol ✓
   │  └─ system::transferSol ✓
   ├─ token::transferChecked ✓ 105cu
   └─ mplCoreProgram::UpdatePlugin ✓ 13574cu
```

</details>
