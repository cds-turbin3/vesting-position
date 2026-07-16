# vested_tokens_are_forfeited_past_the_grace_window

**Source:** [`tests/forfeiture.rs` L16](https://github.com/cds-turbin3/vesting-position/blob/cbb8875460ccf5cc3d9023c540f31113adcd0573/babelfish-tests/tests/forfeiture.rs#L16)

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

> A recipient's vested-but-unclaimed tokens are swept to the creator once the grace window past `end` lapses: this is the design, not a defect.

### Action: tx

| account | before | after |
| --- | --- | --- |
| Vault | 10000000000000 | 9900000000000 |
| whitelisted_1 | — | 100000000000 |

<details>
<summary>tree</summary>

```

Alice (163771cu)
├─ Comp…::? ✓
└─ Vesting::Claim ✓ 163621cu
   ├─ splAssociatedTokenAccount::create ✓ 13416cu
   │  ├─ token::getAccountDataSize ✓ 183cu
   │  ├─ system::createAccount ✓
   │  ├─ token::initializeImmutableOwner ✓ 38cu
   │  └─ token::initializeAccount3 ✓ 235cu
   ├─ system::createAccount ✓
   ├─ CoRE…::CreateV2 ✓ 29646cu
   │  ├─ system::createAccount ✓
   │  ├─ system::transferSol ✓
   │  ├─ system::transferSol ✓
   │  ├─ system::transferSol ✓
   │  └─ system::transferSol ✓
   └─ token::transferChecked ✓ 105cu
```

</details>

### Action: Clawback

| account | before | after |
| --- | --- | --- |
| Creator | 0 | 900000000000 |
| Vault | 9900000000000 | 9000000000000 |

<details>
<summary>tree</summary>

```

Creator (76464cu)
└─ Vesting::Clawback ✓ 76464cu
   ├─ CoRE…::Burn ✓ 9904cu
   └─ token::transferChecked ✓ 105cu
```

</details>

**Alice's remainder:** `900000000000` → `0` — forfeited, never delivered

<details>
<summary>tree</summary>

```

Alice (18897cu)
└─ Vesting::Claim ✗ 18897cu
```

</details>

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
