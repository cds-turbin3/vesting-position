# scenario_2_two_users_transfer_and_bob_claims_both

**Source:** [`tests/claim.rs` L61](https://github.com/cds-turbin3/vesting-position/blob/cbb8875460ccf5cc3d9023c540f31113adcd0573/babelfish-tests/tests/claim.rs#L61)

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

### Action: tx

| account | before | after |
| --- | --- | --- |
| whitelisted_2 | — | 0 |

<details>
<summary>tree</summary>

```

H87x… (157838cu)
├─ Comp…::? ✓
└─ VestingPositions::Claim ✓ 157688cu
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

H87x… (32530cu)
└─ VestingPositions::Claim ✓ 32530cu
```

</details>

<details>
<summary>tree</summary>

```

H87x… (32629cu)
└─ VestingPositions::Claim ✓ 32629cu
```

</details>

```mermaid
sequenceDiagram
    participant p0 as H87x…
    participant p1 as VestingPositions
    p0->>p1: Claim
    activate p1
    p1-->>p0: ✓ 32629cu
    deactivate p1
```

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    H87x(["H87x…"]):::signer
    Collection[("Collection")]:::state
    BXgR[("BXgR…")]:::state
    7NWR[("7NWR…")]:::state
    3oY2[("3oY2…")]:::state
    6vT5[("6vT5…")]:::state
    H87x -->|signs| VestingPositions
    VestingPositions -->|writes| Collection
    VestingPositions -->|writes| BXgR
    VestingPositions -->|writes| 7NWR
    VestingPositions -->|writes| 3oY2
    VestingPositions -->|writes| 6vT5
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
    CoRE -->|owns| 3oY2
    VestingPositions -->|owns| 6vT5
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

<details>
<summary>tree</summary>

```

H87x… (32629cu)
└─ VestingPositions::Claim ✓ 32629cu
```

</details>
