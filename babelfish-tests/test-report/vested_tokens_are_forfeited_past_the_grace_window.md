# vested_tokens_are_forfeited_past_the_grace_window

**Source:** [`tests/forfeiture.rs` L16](https://github.com/cds-turbin3/vesting-position/blob/c4f43acad0bb9f007622fb43bc19ddc3610c7688/babelfish-tests/tests/forfeiture.rs#L16)

```mermaid
sequenceDiagram
    participant p0 as Alice
    participant p1 as Vesting
    p0->>p1: Claim
    activate p1
    note over p1: 🚩 custom program error  0x1789
    p1-->>p0: ✗ 18897cu
    deactivate p1
```

```mermaid
flowchart LR
    Vesting["Vesting"]:::program
    Alice(["Alice"]):::signer
    Collection[("Collection")]:::state
    CampaignATAMint[("Campaign/ATA(Mint)")]:::state
    AliceATAMint[("Alice/ATA(Mint)")]:::state
    AlicespositionNFT[("Alice's position NFT")]:::state
    7kmN[("7kmN…")]:::state
    Alice -->|signs| Vesting
    Vesting -->|writes| Collection
    Vesting -->|writes| CampaignATAMint
    Vesting -->|writes| AliceATAMint
    Vesting -->|writes| AlicespositionNFT
    Vesting -->|writes| 7kmN
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
    AlicespositionNFT[("Alice's position NFT")]:::state
    Vesting["Vesting"]:::program
    7kmN[("7kmN…")]:::state
    system -->|owns| Alice
    CoRE -->|owns| Collection
    token -->|owns| CampaignATAMint
    token -->|owns| AliceATAMint
    CoRE -->|owns| AlicespositionNFT
    Vesting -->|owns| 7kmN
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

<details>
<summary>tree</summary>

```

Alice (18897cu)
└─ Vesting::Claim ✗ 18897cu
```

</details>
