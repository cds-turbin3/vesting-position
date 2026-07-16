# scenario_5_unwhitelisted_user_fails

**Source:** [`tests/claim.rs` L145](https://github.com/cds-turbin3/vesting-position/blob/4697a27f760076ccee6d3a855becaca181647236/babelfish-tests/tests/claim.rs#L145)

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

### Action: Claim

| account | before | after |
| --- | --- | --- |
| not_whitelisted | — | 0 |

<details>
<summary>tree</summary>

```

ErV6… (87917cu)
└─ VestingPositions::Claim ✗ 87917cu
   ├─ splAssociatedTokenAccount::create ✓ 13416cu
   │  ├─ token::getAccountDataSize ✓ 183cu
   │  ├─ system::createAccount ✓
   │  ├─ token::initializeImmutableOwner ✓ 38cu
   │  └─ token::initializeAccount3 ✓ 235cu
   └─ system::createAccount ✓
```

</details>

```mermaid
sequenceDiagram
    participant p0 as ErV6…
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
    p1->>p4: createAccount
    activate p4
    p4-->>p1: ✓
    deactivate p4
    note over p1: 🚩 custom program error  0x177d
    p1-->>p0: ✗ 87917cu
    deactivate p1
```

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    ErV6(["ErV6…"]):::signer
    Collection[("Collection")]:::state
    BXgR[("BXgR…")]:::state
    DP7c(["DP7c…"]):::signer
    Dtvv[("Dtvv…")]:::state
    C6G6(["C6G6…"]):::signer
    splAssociatedTokenAccount["splAssociatedTokenAccount"]:::program
    token["token"]:::program
    system["system"]:::program
    ErV6 -->|signs| VestingPositions
    VestingPositions -->|writes| Collection
    VestingPositions -->|writes| BXgR
    VestingPositions -->|writes| DP7c
    VestingPositions -->|writes| Dtvv
    VestingPositions -->|writes| C6G6
    ErV6 -->|signs| splAssociatedTokenAccount
    splAssociatedTokenAccount -->|writes| DP7c
    ErV6 -->|signs| system
    DP7c -->|signs| system
    token -->|writes| DP7c
    C6G6 -->|signs| system
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

```mermaid
flowchart LR
    system["system"]:::program
    ErV6[("ErV6…")]:::state
    CoRE["CoRE…"]:::program
    Collection[("Collection")]:::state
    token["token"]:::program
    BXgR[("BXgR…")]:::state
    DP7c[("DP7c…")]:::state
    Dtvv[("Dtvv…")]:::state
    VestingPositions["VestingPositions"]:::program
    C6G6[("C6G6…")]:::state
    system -->|owns| ErV6
    CoRE -->|owns| Collection
    token -->|owns| BXgR
    token -->|owns| DP7c
    system -->|owns| Dtvv
    VestingPositions -->|owns| C6G6
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

<details>
<summary>tree</summary>

```

ErV6… (87917cu)
└─ VestingPositions::Claim ✗ 87917cu
   ├─ splAssociatedTokenAccount::create ✓ 13416cu
   │  ├─ token::getAccountDataSize ✓ 183cu
   │  ├─ system::createAccount ✓
   │  ├─ token::initializeImmutableOwner ✓ 38cu
   │  └─ token::initializeAccount3 ✓ 235cu
   └─ system::createAccount ✓
```

</details>
