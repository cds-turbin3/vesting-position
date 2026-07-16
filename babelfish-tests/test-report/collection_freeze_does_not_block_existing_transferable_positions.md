# collection_freeze_does_not_block_existing_transferable_positions

**Source:** [`tests/freeze.rs` L73](https://github.com/cds-turbin3/vesting-position/blob/b1f1485d1913faf6443e233580c9b626cfd06178/babelfish-tests/tests/freeze.rs#L73)

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
| Alice | — | 0 |

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

<details>
<summary>tree</summary>

```

Creator (21271cu)
└─ VestingPositions::FreezeCollection ✓ 21271cu
   └─ CoRE…::UpdateCollectionPlugin ✓ 11198cu
```

</details>

<details>
<summary>tree</summary>

```

4wQQ… (9074cu)
└─ CoRE…::Transfer ✓ 9074cu
```

</details>

```mermaid
sequenceDiagram
    participant p0 as 4wQQ…
    participant p1 as CoRE…
    p0->>p1: Transfer
    activate p1
    p1-->>p0: ✓ 9074cu
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

4wQQ… (9074cu)
└─ CoRE…::Transfer ✓ 9074cu
```

</details>
