# transfer_blocked_when_collection_frozen

**Source:** [`tests/freeze.rs` L52](https://github.com/cds-turbin3/vesting-position/blob/cbb8875460ccf5cc3d9023c540f31113adcd0573/babelfish-tests/tests/freeze.rs#L52)

### Action: Initialize

| account | before | after |
| --- | --- | --- |
| Creator | 10000000000000 | 0 |
| Vault | 0 | 10000000000000 |

<details>
<summary>tree</summary>

```

Creator (85370cu)
└─ VestingPositions::Initialize ✓ 85370cu
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

4wQQ… (156702cu)
├─ Comp…::? ✓
└─ VestingPositions::Claim ✓ 156552cu
   ├─ splAssociatedTokenAccount::create ✓ 13416cu
   │  ├─ token::getAccountDataSize ✓ 183cu
   │  ├─ system::createAccount ✓
   │  ├─ token::initializeImmutableOwner ✓ 38cu
   │  └─ token::initializeAccount3 ✓ 235cu
   ├─ system::createAccount ✓
   └─ CoRE…::CreateV2 ✓ 25757cu
      ├─ system::createAccount ✓
      ├─ system::transferSol ✓
      ├─ system::transferSol ✓
      └─ system::transferSol ✓
```

</details>

<details>
<summary>tree</summary>

```

4wQQ… (8836cu)
└─ CoRE…::Transfer ✗ 8836cu
```

</details>

```mermaid
sequenceDiagram
    participant p0 as 4wQQ…
    participant p1 as CoRE…
    p0->>p1: Transfer
    activate p1
    note over p1: 🚩 custom program error  0x9
    p1-->>p0: ✗ 8836cu
    deactivate p1
```

```mermaid
flowchart LR
    CoRE["CoRE…"]:::program
    4QVs[("4QVs…")]:::state
    4wQQ(["4wQQ…"]):::signer
    CoRE -->|writes| 4QVs
    4wQQ -->|signs| CoRE
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

```mermaid
flowchart LR
    CoRE["CoRE…"]:::program
    4QVs[("4QVs…")]:::state
    system["system"]:::program
    4wQQ[("4wQQ…")]:::state
    CoRE -->|owns| 4QVs
    system -->|owns| 4wQQ
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

<details>
<summary>tree</summary>

```

4wQQ… (8836cu)
└─ CoRE…::Transfer ✗ 8836cu
```

</details>
