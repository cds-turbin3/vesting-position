# close_campaign_succeeds_when_vault_empty

**Source:** [`tests/clawback.rs` L224](https://github.com/cds-turbin3/vesting-position/blob/cbb8875460ccf5cc3d9023c540f31113adcd0573/babelfish-tests/tests/clawback.rs#L224)

### Action: Initialize

| account | before | after |
| --- | --- | --- |
| Creator | 1000000000000 | 0 |
| Vault | 0 | 1000000000000 |

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
| Vault | 1000000000000 | 0 |
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

Creator (9878cu)
└─ VestingPositions::CloseCampaign ✓ 9878cu
   └─ token::closeAccount ✓ 118cu
```

</details>

```mermaid
sequenceDiagram
    participant p0 as Creator
    participant p1 as VestingPositions
    participant p2 as token
    p0->>p1: CloseCampaign
    activate p1
    p1->>p2: closeAccount
    activate p2
    p2-->>p1: ✓ 118cu
    deactivate p2
    p1-->>p0: ✓ 9878cu
    deactivate p1
```

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    Creator(["Creator"]):::signer
    2rbE(["2rbE…"]):::signer
    BXgR[("BXgR…")]:::state
    token["token"]:::program
    Creator -->|signs| VestingPositions
    VestingPositions -->|writes| 2rbE
    VestingPositions -->|writes| BXgR
    token -->|writes| BXgR
    token -->|writes| Creator
    2rbE -->|signs| token
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

```mermaid
flowchart LR
    system["system"]:::program
    Creator[("Creator")]:::state
    2rbE[("2rbE…")]:::state
    BXgR[("BXgR…")]:::state
    system -->|owns| Creator
    system -->|owns| 2rbE
    system -->|owns| BXgR
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

<details>
<summary>tree</summary>

```

Creator (9878cu)
└─ VestingPositions::CloseCampaign ✓ 9878cu
   └─ token::closeAccount ✓ 118cu
```

</details>
