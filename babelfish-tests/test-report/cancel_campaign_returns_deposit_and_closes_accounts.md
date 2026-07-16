# cancel_campaign_returns_deposit_and_closes_accounts

**Source:** [`tests/clawback.rs` L270](https://github.com/cds-turbin3/vesting-position/blob/cbb8875460ccf5cc3d9023c540f31113adcd0573/babelfish-tests/tests/clawback.rs#L270)

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

### Action: CancelCampaign

| account | before | after |
| --- | --- | --- |
| Creator | 0 | 10000000000000 |
| Vault | 10000000000000 | 0 |

<details>
<summary>tree</summary>

```

Creator (30937cu)
└─ VestingPositions::CancelCampaign ✓ 30937cu
   ├─ token::transferChecked ✓ 105cu
   ├─ token::closeAccount ✓ 118cu
   └─ CoRE…::BurnCollection ✓ 6500cu
```

</details>

```mermaid
sequenceDiagram
    participant p0 as Creator
    participant p1 as VestingPositions
    participant p2 as token
    participant p3 as CoRE…
    p0->>p1: CancelCampaign
    activate p1
    p1->>p2: transferChecked
    activate p2
    p2-->>p1: ✓ 105cu
    deactivate p2
    p1->>p2: closeAccount
    activate p2
    p2-->>p1: ✓ 118cu
    deactivate p2
    p1->>p3: BurnCollection
    activate p3
    p3-->>p1: ✓ 6500cu
    deactivate p3
    p1-->>p0: ✓ 30937cu
    deactivate p1
```

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    Creator(["Creator"]):::signer
    2rbE(["2rbE…"]):::signer
    Collection[("Collection")]:::state
    vGh5(["vGh5…"]):::signer
    BXgR[("BXgR…")]:::state
    CreatorATAMint[("Creator/ATA(Mint)")]:::state
    token["token"]:::program
    CoRE["CoRE…"]:::program
    Creator -->|signs| VestingPositions
    VestingPositions -->|writes| 2rbE
    VestingPositions -->|writes| Collection
    VestingPositions -->|writes| vGh5
    VestingPositions -->|writes| BXgR
    VestingPositions -->|writes| CreatorATAMint
    token -->|writes| BXgR
    token -->|writes| CreatorATAMint
    2rbE -->|signs| token
    token -->|writes| Creator
    CoRE -->|writes| Collection
    Creator -->|signs| CoRE
    vGh5 -->|signs| CoRE
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

```mermaid
flowchart LR
    system["system"]:::program
    Creator[("Creator")]:::state
    2rbE[("2rbE…")]:::state
    CoRE["CoRE…"]:::program
    Collection[("Collection")]:::state
    vGh5[("vGh5…")]:::state
    BXgR[("BXgR…")]:::state
    token["token"]:::program
    CreatorATAMint[("Creator/ATA(Mint)")]:::state
    system -->|owns| Creator
    system -->|owns| 2rbE
    CoRE -->|owns| Collection
    system -->|owns| vGh5
    system -->|owns| BXgR
    token -->|owns| CreatorATAMint
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

<details>
<summary>tree</summary>

```

Creator (30937cu)
└─ VestingPositions::CancelCampaign ✓ 30937cu
   ├─ token::transferChecked ✓ 105cu
   ├─ token::closeAccount ✓ 118cu
   └─ CoRE…::BurnCollection ✓ 6500cu
```

</details>
