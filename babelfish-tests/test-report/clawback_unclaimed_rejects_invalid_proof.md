# clawback_unclaimed_rejects_invalid_proof

**Source:** [`tests/clawback.rs` L130](https://github.com/cds-turbin3/vesting-position/blob/00f83f93df6e185e9f69ec6e9580d50bbc2a69c9/babelfish-tests/tests/clawback.rs#L130)

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

### Sequence

```mermaid
sequenceDiagram
    participant p0 as Creator
    participant p1 as VestingPositions
    participant p2 as system
    participant p3 as splAssociatedTokenAccount
    participant p4 as token
    participant p5 as CoRE…
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
    p3-->>p1: ✓ 13517cu
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
    p1-->>p0: ✓ 85369cu
    deactivate p1
```

### Authority

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    Creator(["Creator"]):::signer
    Collection(["Collection"]):::signer
    Mint[("Mint")]:::state
    CreatorATAMint[("Creator/ATA(Mint)")]:::state
    2rbE(["2rbE…"]):::signer
    BXgR(["BXgR…"]):::signer
    system["system"]:::program
    splAssociatedTokenAccount["splAssociatedTokenAccount"]:::program
    token["token"]:::program
    CoRE["CoRE…"]:::program
    Creator -->|signs| VestingPositions
    VestingPositions -->|writes| Collection
    VestingPositions -->|writes| Mint
    VestingPositions -->|writes| CreatorATAMint
    VestingPositions -->|writes| 2rbE
    VestingPositions -->|writes| BXgR
    Creator -->|signs| system
    2rbE -->|signs| system
    Creator -->|signs| splAssociatedTokenAccount
    splAssociatedTokenAccount -->|writes| BXgR
    BXgR -->|signs| system
    token -->|writes| BXgR
    Collection -->|signs| CoRE
    Creator -->|signs| CoRE
    Collection -->|signs| system
    system -->|writes| Collection
    token -->|writes| CreatorATAMint
    Creator -->|signs| token
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

### Ownership

```mermaid
flowchart LR
    system["system"]:::program
    Creator[("Creator")]:::state
    CoRE["CoRE…"]:::program
    Collection[("Collection")]:::state
    token["token"]:::program
    Mint[("Mint")]:::state
    CreatorATAMint[("Creator/ATA(Mint)")]:::state
    VestingPositions["VestingPositions"]:::program
    2rbE[("2rbE…")]:::state
    BXgR[("BXgR…")]:::state
    system -->|owns| Creator
    CoRE -->|owns| Collection
    token -->|owns| Mint
    token -->|owns| CreatorATAMint
    VestingPositions -->|owns| 2rbE
    token -->|owns| BXgR
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

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

<details>
<summary>tree</summary>

```

Creator (69028cu)
└─ VestingPositions::ClawbackUnclaimed ✗ 69028cu
   └─ system::createAccount ✓
```

</details>

### Sequence

```mermaid
sequenceDiagram
    participant p0 as Creator
    participant p1 as VestingPositions
    participant p2 as system
    p0->>p1: ClawbackUnclaimed
    activate p1
    p1->>p2: createAccount
    activate p2
    p2-->>p1: ✓
    deactivate p2
    note over p1: 🚩 custom program error  0x177d
    p1-->>p0: ✗ 69028cu
    deactivate p1
```

### Authority

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    Creator(["Creator"]):::signer
    6vT5(["6vT5…"]):::signer
    BXgR[("BXgR…")]:::state
    CreatorATAMint[("Creator/ATA(Mint)")]:::state
    system["system"]:::program
    Creator -->|signs| VestingPositions
    VestingPositions -->|writes| 6vT5
    VestingPositions -->|writes| BXgR
    VestingPositions -->|writes| CreatorATAMint
    Creator -->|signs| system
    6vT5 -->|signs| system
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

### Ownership

```mermaid
flowchart LR
    system["system"]:::program
    Creator[("Creator")]:::state
    VestingPositions["VestingPositions"]:::program
    6vT5[("6vT5…")]:::state
    token["token"]:::program
    BXgR[("BXgR…")]:::state
    CreatorATAMint[("Creator/ATA(Mint)")]:::state
    system -->|owns| Creator
    VestingPositions -->|owns| 6vT5
    token -->|owns| BXgR
    token -->|owns| CreatorATAMint
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

<details>
<summary>tree</summary>

```

Creator (69028cu)
└─ VestingPositions::ClawbackUnclaimed ✗ 69028cu
   └─ system::createAccount ✓
```

</details>
