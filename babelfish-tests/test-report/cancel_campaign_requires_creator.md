# cancel_campaign_requires_creator

**Source:** [`tests/clawback.rs` L335](https://github.com/cds-turbin3/vesting-position/blob/c4f43acad0bb9f007622fb43bc19ddc3610c7688/babelfish-tests/tests/clawback.rs#L335)

```mermaid
sequenceDiagram
    participant p0 as 4wQQ…
    participant p1 as VestingPositions
    participant p2 as splAssociatedTokenAccount
    participant p3 as token
    participant p4 as system
    p0->>p1: CancelCampaign
    activate p1
    p1->>p2: create
    activate p2
    p2->>p3: getAccountDataSize
    activate p3
    p3-->>p2: ✓ 183cu
    deactivate p3
    p2->>p4: createAccount
    activate p4
    p4-->>p2: ✓
    deactivate p4
    p2->>p3: initializeImmutableOwner
    activate p3
    p3-->>p2: ✓ 38cu
    deactivate p3
    p2->>p3: initializeAccount3
    activate p3
    p3-->>p2: ✓ 235cu
    deactivate p3
    p2-->>p1: ✓ 13416cu
    deactivate p2
    note over p1: 🚩 custom program error  0x1778
    p1-->>p0: ✗ 26432cu
    deactivate p1
```

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    4wQQ(["4wQQ…"]):::signer
    2rbE[("2rbE…")]:::state
    Collection[("Collection")]:::state
    vGh5[("vGh5…")]:::state
    BXgR[("BXgR…")]:::state
    45PB(["45PB…"]):::signer
    splAssociatedTokenAccount["splAssociatedTokenAccount"]:::program
    token["token"]:::program
    system["system"]:::program
    4wQQ -->|signs| VestingPositions
    VestingPositions -->|writes| 2rbE
    VestingPositions -->|writes| Collection
    VestingPositions -->|writes| vGh5
    VestingPositions -->|writes| BXgR
    VestingPositions -->|writes| 45PB
    4wQQ -->|signs| splAssociatedTokenAccount
    splAssociatedTokenAccount -->|writes| 45PB
    4wQQ -->|signs| system
    45PB -->|signs| system
    token -->|writes| 45PB
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

```mermaid
flowchart LR
    system["system"]:::program
    4wQQ[("4wQQ…")]:::state
    VestingPositions["VestingPositions"]:::program
    2rbE[("2rbE…")]:::state
    CoRE["CoRE…"]:::program
    Collection[("Collection")]:::state
    vGh5[("vGh5…")]:::state
    token["token"]:::program
    BXgR[("BXgR…")]:::state
    45PB[("45PB…")]:::state
    system -->|owns| 4wQQ
    VestingPositions -->|owns| 2rbE
    CoRE -->|owns| Collection
    system -->|owns| vGh5
    token -->|owns| BXgR
    token -->|owns| 45PB
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

<details>
<summary>tree</summary>

```

4wQQ… (26432cu)
└─ VestingPositions::CancelCampaign ✗ 26432cu
   └─ splAssociatedTokenAccount::create ✓ 13416cu
      ├─ token::getAccountDataSize ✓ 183cu
      ├─ system::createAccount ✓
      ├─ token::initializeImmutableOwner ✓ 38cu
      └─ token::initializeAccount3 ✓ 235cu
```

</details>
