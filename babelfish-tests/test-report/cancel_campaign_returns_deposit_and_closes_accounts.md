# cancel_campaign_returns_deposit_and_closes_accounts

**Source:** [`tests/clawback.rs` L268](https://github.com/cds-turbin3/vesting-position/blob/c4f43acad0bb9f007622fb43bc19ddc3610c7688/babelfish-tests/tests/clawback.rs#L268)

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
