# clawback_unclaimed_recovers_allocation_and_blocks_claim

**Source:** [`tests/clawback.rs` L103](https://github.com/cds-turbin3/vesting-position/blob/4697a27f760076ccee6d3a855becaca181647236/babelfish-tests/tests/clawback.rs#L103)

### Action: Initialize

| account | before | after |
| --- | --- | --- |
| Creator | 10000000000000 | 0 |
| Vault | 0 | 10000000000000 |

<details>
<summary>tree</summary>

```

Creator (85369cu)
└─ VestingPositions::Initialize ✓ 85369cu
   ├─ system::createAccount ✓
   ├─ splAssociatedTokenAccount::create ✓ 13517cu
   │  ├─ token::getAccountDataSize ✓ 183cu
   │  ├─ system::createAccount ✓
   │  ├─ token::initializeImmutableOwner ✓ 38cu
   │  └─ token::initializeAccount3 ✓ 235cu
   ├─ CoRE…::CreateCollectionV2 ✓ 19976cu
   │  ├─ system::createAccount ✓
   │  ├─ system::transferSol ✓
   │  ├─ system::transferSol ✓
   │  └─ system::transferSol ✓
   └─ token::transferChecked ✓ 105cu
```

</details>

### Action: ClawbackUnclaimed

| account | before | after |
| --- | --- | --- |
| Creator | 0 | 2000000000000 |
| Vault | 10000000000000 | 8000000000000 |

<details>
<summary>tree</summary>

```

Creator (95434cu)
└─ VestingPositions::ClawbackUnclaimed ✓ 95434cu
   ├─ system::createAccount ✓
   └─ token::transferChecked ✓ 105cu
```

</details>

### Action: Claim

| account | before | after |
| --- | --- | --- |
| whitelisted_2 | — | 0 |

<details>
<summary>tree</summary>

```

H87x… (36421cu)
└─ VestingPositions::Claim ✗ 36421cu
   └─ splAssociatedTokenAccount::create ✓ 13416cu
      ├─ token::getAccountDataSize ✓ 183cu
      ├─ system::createAccount ✓
      ├─ token::initializeImmutableOwner ✓ 38cu
      └─ token::initializeAccount3 ✓ 235cu
```

</details>

```mermaid
sequenceDiagram
    participant p0 as H87x…
    participant p1 as VestingPositions
    participant p2 as splAssociatedTokenAccount
    participant p3 as token
    participant p4 as system
    p0->>p1: Claim
    activate p1
    p1->>p2: create
    activate p2
    p2->>p3: getAccountDataSize
    activate p3
    p3-->>p2: ✓ 183cu
    deactivate p3
    p2->>p4: createAccount
    activate p4
    p4-->>p2: ✓
    deactivate p4
    p2->>p3: initializeImmutableOwner
    activate p3
    p3-->>p2: ✓ 38cu
    deactivate p3
    p2->>p3: initializeAccount3
    activate p3
    p3-->>p2: ✓ 235cu
    deactivate p3
    p2-->>p1: ✓ 13416cu
    deactivate p2
    note over p1: 🚩 custom program error  0x1789
    p1-->>p0: ✗ 36421cu
    deactivate p1
```

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    H87x(["H87x…"]):::signer
    Collection[("Collection")]:::state
    BXgR[("BXgR…")]:::state
    7NWR(["7NWR…"]):::signer
    3oY2[("3oY2…")]:::state
    6vT5[("6vT5…")]:::state
    splAssociatedTokenAccount["splAssociatedTokenAccount"]:::program
    token["token"]:::program
    system["system"]:::program
    H87x -->|signs| VestingPositions
    VestingPositions -->|writes| Collection
    VestingPositions -->|writes| BXgR
    VestingPositions -->|writes| 7NWR
    VestingPositions -->|writes| 3oY2
    VestingPositions -->|writes| 6vT5
    H87x -->|signs| splAssociatedTokenAccount
    splAssociatedTokenAccount -->|writes| 7NWR
    H87x -->|signs| system
    7NWR -->|signs| system
    token -->|writes| 7NWR
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

```mermaid
flowchart LR
    system["system"]:::program
    H87x[("H87x…")]:::state
    CoRE["CoRE…"]:::program
    Collection[("Collection")]:::state
    token["token"]:::program
    BXgR[("BXgR…")]:::state
    7NWR[("7NWR…")]:::state
    3oY2[("3oY2…")]:::state
    VestingPositions["VestingPositions"]:::program
    6vT5[("6vT5…")]:::state
    system -->|owns| H87x
    CoRE -->|owns| Collection
    token -->|owns| BXgR
    token -->|owns| 7NWR
    system -->|owns| 3oY2
    VestingPositions -->|owns| 6vT5
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

<details>
<summary>tree</summary>

```

H87x… (36421cu)
└─ VestingPositions::Claim ✗ 36421cu
   └─ splAssociatedTokenAccount::create ✓ 13416cu
      ├─ token::getAccountDataSize ✓ 183cu
      ├─ system::createAccount ✓
      ├─ token::initializeImmutableOwner ✓ 38cu
      └─ token::initializeAccount3 ✓ 235cu
```

</details>
