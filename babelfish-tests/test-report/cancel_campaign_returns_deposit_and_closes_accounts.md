# cancel_campaign_returns_deposit_and_closes_accounts

**Source:** [`tests/clawback.rs` L268](https://github.com/cds-turbin3/vesting-position/blob/3f946c506628d348826d858340b4ac335a62e967/babelfish-tests/tests/clawback.rs#L268)

```mermaid
sequenceDiagram
    participant p0 as Creator
    participant p1 as VestingPositions
    p0->>p1: CancelCampaign
    activate p1
    note over p1: 🚩 custom program error  0x1788
    p1-->>p0: ✗ 18233cu
    deactivate p1
```

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    Creator(["Creator"]):::signer
    2rbE[("2rbE…")]:::state
    Collection[("Collection")]:::state
    vGh5[("vGh5…")]:::state
    BXgR[("BXgR…")]:::state
    CreatorATAMint[("Creator/ATA(Mint)")]:::state
    Creator -->|signs| VestingPositions
    VestingPositions -->|writes| 2rbE
    VestingPositions -->|writes| Collection
    VestingPositions -->|writes| vGh5
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
    VestingPositions["VestingPositions"]:::program
    2rbE[("2rbE…")]:::state
    CoRE["CoRE…"]:::program
    Collection[("Collection")]:::state
    vGh5[("vGh5…")]:::state
    token["token"]:::program
    BXgR[("BXgR…")]:::state
    CreatorATAMint[("Creator/ATA(Mint)")]:::state
    system -->|owns| Creator
    VestingPositions -->|owns| 2rbE
    CoRE -->|owns| Collection
    system -->|owns| vGh5
    token -->|owns| BXgR
    token -->|owns| CreatorATAMint
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

<details>
<summary>tree</summary>

```

Creator (18233cu)
└─ VestingPositions::CancelCampaign ✗ 18233cu
```

</details>
