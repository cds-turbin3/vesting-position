# exclude_asset_blocks_subsequent_claims

**Source:** [`tests/freeze.rs` L187](https://github.com/cds-turbin3/vesting-position/blob/8b919a4afb93b5878c3ba60d3c6a8b23c5b3b11f/babelfish-tests/tests/freeze.rs#L187)

Over 31 days: 5 moments.

### Cast

| name | address |
| --- | --- |
| Collection | DZ9SxyoirUd6SJtqTyLzGjyRuQjmxcs1ySPeuZqzmAA9 |
| Creator | 2ZBYuwtWiRzk7CwiCYTv5MQhQHDEaN4B8xhw4L7L3RY5 |
| Mint | J6WtRQHtRxemkWECcNq1wQAfgyJtsx7ZUT7Xpb53vo2N |
| VestingPositions | 7DkU9TQhcN87f2djZDd2MjjPZoXLfnZZj8HhybeZswX1 |
| campaign(Collection) | 2rbEagbB2s6tm4BWjcCmYnPxEi8cmF6CqCYPK6WhKZrC |
| campaignAta(campaign(Collection), Toke…, Mint) | BXgRMbNU5GHfjUAEUL4gc4dkK47Kbd26uZfbunemQyjQ |
| creatorAta(Creator, Toke…, Mint) | 7RKXPrw2SRFrpVR7h81dMPhHdaVV7y57tEYmDraizX43 |
| updateAuthority(Collection) | vGh5gMbD7AsVatTVNDo3pC7hveLFKdXbZ3eNt17chDs |
| userAta(4wQQ…, Toke…, Mint) | 45PBE5WDayGKEeGHvaLP5VfqXSS5JoGVewXyVbSE8JSq |

### T0: Initialize (day 0)

| observation | before | after |
| --- | --- | --- |
| Vault balance | — | 10,000,000,000,000 |
| Creator balance | — | 0 |

### Sequence

```mermaid
sequenceDiagram
    participant p0 as Creator
    participant p1 as VestingPositions
    participant p2 as system
    participant p3 as splAssociatedTokenAccount
    participant p4 as token
    participant p5 as CoRE…
    p0->>p1: Initialize
    activate p1
    p1->>p2: createAccount
    activate p2
    p2-->>p1: ✓
    deactivate p2
    p1->>p3: create
    activate p3
    p3->>p4: getAccountDataSize
    activate p4
    p4-->>p3: ✓ 183cu
    deactivate p4
    p3->>p2: createAccount
    activate p2
    p2-->>p3: ✓
    deactivate p2
    p3->>p4: initializeImmutableOwner
    activate p4
    p4-->>p3: ✓ 38cu
    deactivate p4
    p3->>p4: initializeAccount3
    activate p4
    p4-->>p3: ✓ 235cu
    deactivate p4
    p3-->>p1: ✓ 13517cu
    deactivate p3
    p1->>p5: CreateCollectionV2
    activate p5
    p5->>p2: createAccount
    activate p2
    p2-->>p5: ✓
    deactivate p2
    p5->>p2: transferSol
    activate p2
    p2-->>p5: ✓
    deactivate p2
    p5->>p2: transferSol
    activate p2
    p2-->>p5: ✓
    deactivate p2
    p5->>p2: transferSol
    activate p2
    p2-->>p5: ✓
    deactivate p2
    p5-->>p1: ✓ 19976cu
    deactivate p5
    p1->>p4: transferChecked
    activate p4
    p4-->>p1: ✓ 105cu
    deactivate p4
    p1-->>p0: ✓ 85369cu
    deactivate p1
```

### Authority

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    Creator(["Creator"]):::signer
    Collection(["Collection"]):::signer
    Mint[("Mint")]:::state
    creatorAtaCreatorTokeMint[("creatorAta(Creator, Toke…, Mint)")]:::state
    campaignCollection(["campaign(Collection)"]):::signer
    campaignAtacampaignCollectionTokeMint(["campaignAta(campaign(Collection), Toke…, Mint)"]):::signer
    system["system"]:::program
    splAssociatedTokenAccount["splAssociatedTokenAccount"]:::program
    token["token"]:::program
    CoRE["CoRE…"]:::program
    Creator -->|signs| VestingPositions
    VestingPositions -->|writes| Collection
    VestingPositions -->|writes| Mint
    VestingPositions -->|writes| creatorAtaCreatorTokeMint
    VestingPositions -->|writes| campaignCollection
    VestingPositions -->|writes| campaignAtacampaignCollectionTokeMint
    Creator -->|signs| system
    campaignCollection -->|signs| system
    Creator -->|signs| splAssociatedTokenAccount
    splAssociatedTokenAccount -->|writes| campaignAtacampaignCollectionTokeMint
    campaignAtacampaignCollectionTokeMint -->|signs| system
    token -->|writes| campaignAtacampaignCollectionTokeMint
    Collection -->|signs| CoRE
    Creator -->|signs| CoRE
    Collection -->|signs| system
    system -->|writes| Collection
    token -->|writes| creatorAtaCreatorTokeMint
    Creator -->|signs| token
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

### Ownership

```mermaid
flowchart LR
    system["system"]:::program
    Creator[("Creator")]:::state
    CoRE["CoRE…"]:::program
    Collection[("Collection")]:::state
    token["token"]:::program
    Mint[("Mint")]:::state
    creatorAtaCreatorTokeMint[("creatorAta(Creator, Toke…, Mint)")]:::state
    VestingPositions["VestingPositions"]:::program
    campaignCollection[("campaign(Collection)")]:::state
    campaignAtacampaignCollectionTokeMint[("campaignAta(campaign(Collection), Toke…, Mint)")]:::state
    system -->|owns| Creator
    CoRE -->|owns| Collection
    token -->|owns| Mint
    token -->|owns| creatorAtaCreatorTokeMint
    VestingPositions -->|owns| campaignCollection
    token -->|owns| campaignAtacampaignCollectionTokeMint
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

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

*1 day pass.*

### T1: FirstClaim (day 1)

### Sequence

```mermaid
sequenceDiagram
    participant p0 as 4wQQ…
    participant p1 as Comp…
    participant p2 as VestingPositions
    participant p3 as splAssociatedTokenAccount
    participant p4 as token
    participant p5 as system
    participant p6 as CoRE…
    p0->>p1: ?
    activate p1
    p1-->>p0: ✓
    deactivate p1
    p0->>p2: Claim
    activate p2
    p2->>p3: create
    activate p3
    p3->>p4: getAccountDataSize
    activate p4
    p4-->>p3: ✓ 183cu
    deactivate p4
    p3->>p5: createAccount
    activate p5
    p5-->>p3: ✓
    deactivate p5
    p3->>p4: initializeImmutableOwner
    activate p4
    p4-->>p3: ✓ 38cu
    deactivate p4
    p3->>p4: initializeAccount3
    activate p4
    p4-->>p3: ✓ 235cu
    deactivate p4
    p3-->>p2: ✓ 13416cu
    deactivate p3
    p2->>p5: createAccount
    activate p5
    p5-->>p2: ✓
    deactivate p5
    p2->>p6: CreateV2
    activate p6
    p6->>p5: createAccount
    activate p5
    p5-->>p6: ✓
    deactivate p5
    p6->>p5: transferSol
    activate p5
    p5-->>p6: ✓
    deactivate p5
    p6->>p5: transferSol
    activate p5
    p5-->>p6: ✓
    deactivate p5
    p6->>p5: transferSol
    activate p5
    p5-->>p6: ✓
    deactivate p5
    p6->>p5: transferSol
    activate p5
    p5-->>p6: ✓
    deactivate p5
    p6-->>p2: ✓ 29413cu
    deactivate p6
    p2-->>p0: ✓ 160409cu
    deactivate p2
```

### Authority

```mermaid
flowchart LR
    Comp["Comp…"]:::program
    VestingPositions["VestingPositions"]:::program
    4wQQ(["4wQQ…"]):::signer
    Collection[("Collection")]:::state
    campaignAtacampaignCollectionTokeMint[("campaignAta(campaign(Collection), Toke…, Mint)")]:::state
    45PB(["45PB…"]):::signer
    4QVs(["4QVs…"]):::signer
    7kmN(["7kmN…"]):::signer
    splAssociatedTokenAccount["splAssociatedTokenAccount"]:::program
    token["token"]:::program
    system["system"]:::program
    CoRE["CoRE…"]:::program
    updateAuthorityCollection(["updateAuthority(Collection)"]):::signer
    4wQQ -->|signs| VestingPositions
    VestingPositions -->|writes| Collection
    VestingPositions -->|writes| campaignAtacampaignCollectionTokeMint
    VestingPositions -->|writes| 45PB
    VestingPositions -->|writes| 4QVs
    VestingPositions -->|writes| 7kmN
    4wQQ -->|signs| splAssociatedTokenAccount
    splAssociatedTokenAccount -->|writes| 45PB
    4wQQ -->|signs| system
    45PB -->|signs| system
    token -->|writes| 45PB
    7kmN -->|signs| system
    4QVs -->|signs| CoRE
    CoRE -->|writes| Collection
    updateAuthorityCollection -->|signs| CoRE
    4wQQ -->|signs| CoRE
    4QVs -->|signs| system
    system -->|writes| 4QVs
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

### Ownership

```mermaid
flowchart LR
    system["system"]:::program
    4wQQ[("4wQQ…")]:::state
    CoRE["CoRE…"]:::program
    Collection[("Collection")]:::state
    token["token"]:::program
    campaignAtacampaignCollectionTokeMint[("campaignAta(campaign(Collection), Toke…, Mint)")]:::state
    45PB[("45PB…")]:::state
    4QVs[("4QVs…")]:::state
    VestingPositions["VestingPositions"]:::program
    7kmN[("7kmN…")]:::state
    system -->|owns| 4wQQ
    CoRE -->|owns| Collection
    token -->|owns| campaignAtacampaignCollectionTokeMint
    token -->|owns| 45PB
    CoRE -->|owns| 4QVs
    VestingPositions -->|owns| 7kmN
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

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

*1339200 seconds pass.*

### T2: Claim (day 16)

| observation | before | after |
| --- | --- | --- |
| Vault balance | 10,000,000,000,000 | 9,450,000,000,000 |
| Alice balance | — | 550,000,000,000 |

### Sequence

```mermaid
sequenceDiagram
    participant p0 as 4wQQ…
    participant p1 as VestingPositions
    participant p2 as CoRE…
    participant p3 as system
    participant p4 as token
    p0->>p1: Claim
    activate p1
    p1->>p2: UpdatePlugin
    activate p2
    p2->>p3: transferSol
    activate p3
    p3-->>p2: ✓
    deactivate p3
    p2-->>p1: ✓ 22502cu
    deactivate p2
    p1->>p4: transferChecked
    activate p4
    p4-->>p1: ✓ 105cu
    deactivate p4
    p1-->>p0: ✓ 72635cu
    deactivate p1
```

### Authority

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    4wQQ(["4wQQ…"]):::signer
    Collection[("Collection")]:::state
    campaignAtacampaignCollectionTokeMint[("campaignAta(campaign(Collection), Toke…, Mint)")]:::state
    userAta4wQQTokeMint[("userAta(4wQQ…, Toke…, Mint)")]:::state
    4QVs[("4QVs…")]:::state
    7kmN[("7kmN…")]:::state
    CoRE["CoRE…"]:::program
    updateAuthorityCollection(["updateAuthority(Collection)"]):::signer
    system["system"]:::program
    token["token"]:::program
    campaignCollection(["campaign(Collection)"]):::signer
    4wQQ -->|signs| VestingPositions
    VestingPositions -->|writes| Collection
    VestingPositions -->|writes| campaignAtacampaignCollectionTokeMint
    VestingPositions -->|writes| userAta4wQQTokeMint
    VestingPositions -->|writes| 4QVs
    VestingPositions -->|writes| 7kmN
    CoRE -->|writes| 4QVs
    CoRE -->|writes| Collection
    4wQQ -->|signs| CoRE
    updateAuthorityCollection -->|signs| CoRE
    4wQQ -->|signs| system
    system -->|writes| 4QVs
    token -->|writes| campaignAtacampaignCollectionTokeMint
    token -->|writes| userAta4wQQTokeMint
    campaignCollection -->|signs| token
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

### Ownership

```mermaid
flowchart LR
    system["system"]:::program
    4wQQ[("4wQQ…")]:::state
    CoRE["CoRE…"]:::program
    Collection[("Collection")]:::state
    token["token"]:::program
    campaignAtacampaignCollectionTokeMint[("campaignAta(campaign(Collection), Toke…, Mint)")]:::state
    userAta4wQQTokeMint[("userAta(4wQQ…, Toke…, Mint)")]:::state
    4QVs[("4QVs…")]:::state
    VestingPositions["VestingPositions"]:::program
    7kmN[("7kmN…")]:::state
    system -->|owns| 4wQQ
    CoRE -->|owns| Collection
    token -->|owns| campaignAtacampaignCollectionTokeMint
    token -->|owns| userAta4wQQTokeMint
    CoRE -->|owns| 4QVs
    VestingPositions -->|owns| 7kmN
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

<details>
<summary>tree</summary>

```

4wQQ… (72635cu)
└─ VestingPositions::Claim ✓ 72635cu
   ├─ CoRE…::UpdatePlugin ✓ 22502cu
   │  └─ system::transferSol ✓
   └─ token::transferChecked ✓ 105cu
```

</details>

### T3: ExcludeAsset (day 16)

| observation | before | after |
| --- | --- | --- |
| Vault balance | 9,450,000,000,000 | 9,000,000,000,000 |
| Creator balance | 0 | 450,000,000,000 |

### Sequence

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
    p1-->>p0: ✓ 76272cu
    deactivate p1
```

### Authority

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    Creator(["Creator"]):::signer
    Collection[("Collection")]:::state
    4QVs[("4QVs…")]:::state
    campaignAtacampaignCollectionTokeMint[("campaignAta(campaign(Collection), Toke…, Mint)")]:::state
    creatorAtaCreatorTokeMint[("creatorAta(Creator, Toke…, Mint)")]:::state
    CoRE["CoRE…"]:::program
    updateAuthorityCollection(["updateAuthority(Collection)"]):::signer
    token["token"]:::program
    campaignCollection(["campaign(Collection)"]):::signer
    Creator -->|signs| VestingPositions
    VestingPositions -->|writes| Collection
    VestingPositions -->|writes| 4QVs
    VestingPositions -->|writes| campaignAtacampaignCollectionTokeMint
    VestingPositions -->|writes| creatorAtaCreatorTokeMint
    CoRE -->|writes| 4QVs
    CoRE -->|writes| Collection
    Creator -->|signs| CoRE
    updateAuthorityCollection -->|signs| CoRE
    token -->|writes| campaignAtacampaignCollectionTokeMint
    token -->|writes| creatorAtaCreatorTokeMint
    campaignCollection -->|signs| token
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

### Ownership

```mermaid
flowchart LR
    system["system"]:::program
    Creator[("Creator")]:::state
    CoRE["CoRE…"]:::program
    Collection[("Collection")]:::state
    4QVs[("4QVs…")]:::state
    token["token"]:::program
    campaignAtacampaignCollectionTokeMint[("campaignAta(campaign(Collection), Toke…, Mint)")]:::state
    creatorAtaCreatorTokeMint[("creatorAta(Creator, Toke…, Mint)")]:::state
    system -->|owns| Creator
    CoRE -->|owns| Collection
    CoRE -->|owns| 4QVs
    token -->|owns| campaignAtacampaignCollectionTokeMint
    token -->|owns| creatorAtaCreatorTokeMint
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

<details>
<summary>tree</summary>

```

Creator (76272cu)
└─ VestingPositions::ExcludeAsset ✓ 76272cu
   ├─ CoRE…::Burn ✓ 9904cu
   └─ token::transferChecked ✓ 105cu
```

</details>

*1252801 seconds pass.*

### T4: Claim (day 31)

- [x] refused: InvalidAsset — InstructionError(0, Custom(6015))

### Sequence

```mermaid
sequenceDiagram
    participant p0 as 4wQQ…
    participant p1 as VestingPositions
    p0->>p1: Claim
    activate p1
    note over p1: 🚩 custom program error  0x177f
    p1-->>p0: ✗ 18646cu
    deactivate p1
```

### Authority

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    4wQQ(["4wQQ…"]):::signer
    Collection[("Collection")]:::state
    campaignAtacampaignCollectionTokeMint[("campaignAta(campaign(Collection), Toke…, Mint)")]:::state
    userAta4wQQTokeMint[("userAta(4wQQ…, Toke…, Mint)")]:::state
    4QVs[("4QVs…")]:::state
    7kmN[("7kmN…")]:::state
    4wQQ -->|signs| VestingPositions
    VestingPositions -->|writes| Collection
    VestingPositions -->|writes| campaignAtacampaignCollectionTokeMint
    VestingPositions -->|writes| userAta4wQQTokeMint
    VestingPositions -->|writes| 4QVs
    VestingPositions -->|writes| 7kmN
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

### Ownership

```mermaid
flowchart LR
    system["system"]:::program
    4wQQ[("4wQQ…")]:::state
    CoRE["CoRE…"]:::program
    Collection[("Collection")]:::state
    token["token"]:::program
    campaignAtacampaignCollectionTokeMint[("campaignAta(campaign(Collection), Toke…, Mint)")]:::state
    userAta4wQQTokeMint[("userAta(4wQQ…, Toke…, Mint)")]:::state
    4QVs[("4QVs…")]:::state
    VestingPositions["VestingPositions"]:::program
    7kmN[("7kmN…")]:::state
    system -->|owns| 4wQQ
    CoRE -->|owns| Collection
    token -->|owns| campaignAtacampaignCollectionTokeMint
    token -->|owns| userAta4wQQTokeMint
    CoRE -->|owns| 4QVs
    VestingPositions -->|owns| 7kmN
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

<details>
<summary>tree</summary>

```

4wQQ… (18646cu)
└─ VestingPositions::Claim ✗ 18646cu
```

</details>
