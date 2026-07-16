# full_lifecycle

**Source:** [`tests/full_lifecycle.rs` L23](https://github.com/cds-turbin3/vesting-position/blob/c4f43acad0bb9f007622fb43bc19ddc3610c7688/babelfish-tests/tests/full_lifecycle.rs#L23)

```mermaid
sequenceDiagram
    participant p0 as Alice
    participant p1 as Vesting
    participant p2 as CoRE…
    participant p3 as token
    p0->>p1: Claim
    activate p1
    p1->>p2: UpdatePlugin
    activate p2
    p2-->>p1: ✓ 20727cu
    deactivate p2
    p1->>p3: transferChecked
    activate p3
    p3-->>p1: ✓ 105cu
    deactivate p3
    p1-->>p0: ✓ 71714cu
    deactivate p1
```

```mermaid
flowchart LR
    Vesting["Vesting"]:::program
    Alice(["Alice"]):::signer
    Collection[("Collection")]:::state
    CampaignATAMint[("Campaign/ATA(Mint)")]:::state
    AliceATAMint[("Alice/ATA(Mint)")]:::state
    CharliepositionNFT[("Charlie position NFT")]:::state
    7kmN[("7kmN…")]:::state
    CoRE["CoRE…"]:::program
    vGh5(["vGh5…"]):::signer
    token["token"]:::program
    Campaign(["Campaign"]):::signer
    Alice -->|signs| Vesting
    Vesting -->|writes| Collection
    Vesting -->|writes| CampaignATAMint
    Vesting -->|writes| AliceATAMint
    Vesting -->|writes| CharliepositionNFT
    Vesting -->|writes| 7kmN
    CoRE -->|writes| CharliepositionNFT
    CoRE -->|writes| Collection
    Alice -->|signs| CoRE
    vGh5 -->|signs| CoRE
    token -->|writes| CampaignATAMint
    token -->|writes| AliceATAMint
    Campaign -->|signs| token
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

```mermaid
flowchart LR
    system["system"]:::program
    Alice[("Alice")]:::state
    CoRE["CoRE…"]:::program
    Collection[("Collection")]:::state
    token["token"]:::program
    CampaignATAMint[("Campaign/ATA(Mint)")]:::state
    AliceATAMint[("Alice/ATA(Mint)")]:::state
    CharliepositionNFT[("Charlie position NFT")]:::state
    Vesting["Vesting"]:::program
    7kmN[("7kmN…")]:::state
    system -->|owns| Alice
    CoRE -->|owns| Collection
    token -->|owns| CampaignATAMint
    token -->|owns| AliceATAMint
    CoRE -->|owns| CharliepositionNFT
    Vesting -->|owns| 7kmN
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

<details>
<summary>tree</summary>

```

Alice (71714cu)
└─ Vesting::Claim ✓ 71714cu
   ├─ CoRE…::UpdatePlugin ✓ 20727cu
   └─ token::transferChecked ✓ 105cu
```

</details>
