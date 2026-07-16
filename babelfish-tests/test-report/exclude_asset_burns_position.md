# exclude_asset_burns_position

**Source:** [`tests/freeze.rs` L219](https://github.com/cds-turbin3/vesting-position/blob/c4f43acad0bb9f007622fb43bc19ddc3610c7688/babelfish-tests/tests/freeze.rs#L219)

```mermaid
sequenceDiagram
    participant p0 as Creator
    participant p1 as VestingPositions
    participant p2 as CoRE…
    participant p3 as token
    p0->>p1: ExcludeAsset
    activate p1
    p1->>p2: Burn
    activate p2
    p2-->>p1: ✓ 9904cu
    deactivate p2
    p1->>p3: transferChecked
    activate p3
    p3-->>p1: ✓ 105cu
    deactivate p3
    p1-->>p0: ✓ 75427cu
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
    CoRE["CoRE…"]:::program
    vGh5(["vGh5…"]):::signer
    token["token"]:::program
    2rbE(["2rbE…"]):::signer
    Creator -->|signs| VestingPositions
    VestingPositions -->|writes| Collection
    VestingPositions -->|writes| 4QVs
    VestingPositions -->|writes| BXgR
    VestingPositions -->|writes| CreatorATAMint
    CoRE -->|writes| 4QVs
    CoRE -->|writes| Collection
    Creator -->|signs| CoRE
    vGh5 -->|signs| CoRE
    token -->|writes| BXgR
    token -->|writes| CreatorATAMint
    2rbE -->|signs| token
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

Creator (75427cu)
└─ VestingPositions::ExcludeAsset ✓ 75427cu
   ├─ CoRE…::Burn ✓ 9904cu
   └─ token::transferChecked ✓ 105cu
```

</details>
