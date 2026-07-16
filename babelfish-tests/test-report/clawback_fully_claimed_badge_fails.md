# clawback_fully_claimed_badge_fails

**Source:** [`tests/clawback.rs` L75](https://github.com/cds-turbin3/vesting-position/blob/cbb8875460ccf5cc3d9023c540f31113adcd0573/babelfish-tests/tests/clawback.rs#L75)

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

### Action: tx

| account | before | after |
| --- | --- | --- |
| whitelisted_1 | — | 0 |

<details>
<summary>tree</summary>

```

4wQQ… (160559cu)
├─ Comp…::? ✓
└─ VestingPositions::Claim ✓ 160409cu
   ├─ splAssociatedTokenAccount::create ✓ 13416cu
   │  ├─ token::getAccountDataSize ✓ 183cu
   │  ├─ system::createAccount ✓
   │  ├─ token::initializeImmutableOwner ✓ 38cu
   │  └─ token::initializeAccount3 ✓ 235cu
   ├─ system::createAccount ✓
   └─ CoRE…::CreateV2 ✓ 29413cu
      ├─ system::createAccount ✓
      ├─ system::transferSol ✓
      ├─ system::transferSol ✓
      ├─ system::transferSol ✓
      └─ system::transferSol ✓
```

</details>

### Action: Claim

| account | before | after |
| --- | --- | --- |
| Vault | 10000000000000 | 9000000000000 |
| whitelisted_1 | 0 | 1000000000000 |

<details>
<summary>tree</summary>

```

4wQQ… (91477cu)
└─ VestingPositions::Claim ✓ 91477cu
   ├─ CoRE…::UpdatePlugin ✓ 22492cu
   │  └─ system::transferSol ✓
   ├─ token::transferChecked ✓ 105cu
   └─ CoRE…::UpdatePlugin ✓ 13574cu
```

</details>

<details>
<summary>tree</summary>

```

Creator (50753cu)
└─ VestingPositions::Clawback ✗ 50753cu
```

</details>

```mermaid
sequenceDiagram
    participant p0 as Creator
    participant p1 as VestingPositions
    p0->>p1: Clawback
    activate p1
    note over p1: 🚩 custom program error  0x1784
    p1-->>p0: ✗ 50753cu
    deactivate p1
```

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    Creator(["Creator"]):::signer
    Collection[("Collection")]:::state
    4QVs[("4QVs…")]:::state
    BXgR[("BXgR…")]:::state
    CreatorATAMint[("Creator/ATA(Mint)")]:::state
    Creator -->|signs| VestingPositions
    VestingPositions -->|writes| Collection
    VestingPositions -->|writes| 4QVs
    VestingPositions -->|writes| BXgR
    VestingPositions -->|writes| CreatorATAMint
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

```mermaid
flowchart LR
    system["system"]:::program
    Creator[("Creator")]:::state
    CoRE["CoRE…"]:::program
    Collection[("Collection")]:::state
    4QVs[("4QVs…")]:::state
    token["token"]:::program
    BXgR[("BXgR…")]:::state
    CreatorATAMint[("Creator/ATA(Mint)")]:::state
    system -->|owns| Creator
    CoRE -->|owns| Collection
    CoRE -->|owns| 4QVs
    token -->|owns| BXgR
    token -->|owns| CreatorATAMint
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

<details>
<summary>tree</summary>

```

Creator (50753cu)
└─ VestingPositions::Clawback ✗ 50753cu
```

</details>
