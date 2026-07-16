# freeze_asset_fails_without_freeze_plugin

**Source:** [`tests/freeze.rs` L277](https://github.com/cds-turbin3/vesting-position/blob/c4f43acad0bb9f007622fb43bc19ddc3610c7688/babelfish-tests/tests/freeze.rs#L277)

```mermaid
sequenceDiagram
    participant p0 as Creator
    participant p1 as VestingPositions
    p0->>p1: FreezeAsset
    activate p1
    note over p1: 🚩 custom program error  0x178b
    p1-->>p0: ✗ 10491cu
    deactivate p1
```

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    Creator(["Creator"]):::signer
    Collection[("Collection")]:::state
    4QVs[("4QVs…")]:::state
    Creator -->|signs| VestingPositions
    VestingPositions -->|writes| Collection
    VestingPositions -->|writes| 4QVs
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
    system -->|owns| Creator
    CoRE -->|owns| Collection
    CoRE -->|owns| 4QVs
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

<details>
<summary>tree</summary>

```

Creator (10491cu)
└─ VestingPositions::FreezeAsset ✗ 10491cu
```

</details>
