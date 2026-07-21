# scenario_9_claim_window_closes_after_grace

**Source:** [`tests/claim.rs` L232](https://github.com/cds-turbin3/vesting-position/blob/c89b5ba9f2140371af02754735f2eed3bb87d0ec/babelfish-tests/tests/claim.rs#L232)

Over 38 days: 4 moments.

<details>
<summary>Cast</summary>

| name | address |
| --- | --- |
| Collection | 9HbgSnRdBeYKzrjr9DYtcT6oKYrcTVB8NWUoU7JVKuWW |
| Creator | 2ZBYuwtWiRzk7CwiCYTv5MQhQHDEaN4B8xhw4L7L3RY5 |
| Mint | 4Kr8ypueV83MddH54fZXLkKFKRd7eWFcejQ8HtynfJRk |
| VestingPositions | 7DkU9TQhcN87f2djZDd2MjjPZoXLfnZZj8HhybeZswX1 |
| campaign(Collection) | G4GWLHr4aHZoxWra82eRc111wRJ9aDiXsSuk3bWoys2G |
| campaignAta(campaign(Collection), Toke…, Mint) | 3kKwPKo9z6XQWrZxuo75c36vm9ChMgGDAMBV7cFEhjSn |
| creatorAta(Creator, Toke…, Mint) | AvzkuSUEzjhXyboXRyfQcsjzKLBqeP9FeoExRBWkXjdJ |
| updateAuthority(Collection) | CYBwE6G2RjrsFYbwy5pUVVDVL5UR5g5VaRcWBPbzby1p |
| userAta(4wQQ…, Toke…, Mint) | C7PguAKs34J4WkXRRp762bPFhzhmFBYKdLuHQYBrVmAW |
| 4wQQ… | 4wQQJM9LNuhinieNAqmHuPCm8LXDTVfhx84P32nAVE9P |
| 5fjL… | 5fjLQR7cXnkBzwud4xAzHaxWKbTiQMJyrNe72RXUtieC |
| 5yfA… | 5yfASCcX25V4fzBgdfixtXgS2JrvpWdwX27JQXpVyuHB |
| 6hhA… | 6hhAjXPGt41Y6oPH6mATnAqMECdaHrKaAGbE4SFuARXK |
| C49G… | C49GR57waDzt1nFUvxKPDs1HVAKF694inGJeB2KGRPM9 |
| CoRE… | CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d |
| Comp… | ComputeBudget111111111111111111111111111111 |
| DiZf… | DiZfCmUXMWffwHP4eus9hwgnvTVbiZL7ZGGTBjJ9vc68 |
| H87x… | H87xi4CUqrUPXzppV3jotTmre6DyR5pCaMk5bKQQBFTg |

</details>

### Timeline

| time | Vault balance | Δ | Creator balance | Δ | Alice balance | Δ | Bob balance | comment |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| T0 | 10,000,000,000,000 | +10,000,000,000,000 | 0 |  | — |  | — | Initialize (day 0) |
| T1 | 10,000,000,000,000 |  | 0 |  | — |  | — | FirstClaim (day 1) |
| T2 | 9,000,000,000,000 | -1,000,000,000,000 | 0 |  | 1,000,000,000,000 | +1,000,000,000,000 | — | Claim (day 37) |
| T3 | 9,000,000,000,000 |  | 0 |  | 1,000,000,000,000 |  | — | FirstClaim (day 38) |

### T0: Initialize (day 0)

| observation | before | after |
| --- | --- | --- |
| Vault balance | — | 10,000,000,000,000 |
| Creator balance | — | 0 |

<details>
<summary>diagrams</summary>

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
    p3-->>p1: ✓ 18017cu
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
    p1-->>p0: ✓ 89713cu
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
    VestingPositions --> Mint
    VestingPositions --> creatorAtaCreatorTokeMint
    VestingPositions --> campaignCollection
    VestingPositions --> campaignAtacampaignCollectionTokeMint
    Creator --> system
    campaignCollection --> system
    Creator --> splAssociatedTokenAccount
    splAssociatedTokenAccount --> campaignAtacampaignCollectionTokeMint
    campaignAtacampaignCollectionTokeMint --> system
    token --> campaignAtacampaignCollectionTokeMint
    Collection --> CoRE
    Creator --> CoRE
    Collection --> system
    system --> Collection
    token --> creatorAtaCreatorTokeMint
    Creator --> token
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
    linkStyle 0,6,7,8,10,12,13,14,17 stroke:#82b366
    linkStyle 1,2,3,4,5,9,11,15,16 stroke:#d79b00
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
    CoRE --> Collection
    token --> Mint
    token --> creatorAtaCreatorTokeMint
    VestingPositions --> campaignCollection
    token --> campaignAtacampaignCollectionTokeMint
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
    linkStyle 0,1,2,3,4,5 stroke:#6c8ebf
```

### Tree

```

Creator (89713cu)
└─ VestingPositions::Initialize ✓ 89713cu
   ├─ system::createAccount ✓
   ├─ splAssociatedTokenAccount::create ✓ 18017cu
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

<details>
<summary>diagrams</summary>

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
    p2-->>p0: ✓ 164909cu
    deactivate p2
```

### Authority

```mermaid
flowchart LR
    Comp["Comp…"]:::program
    VestingPositions["VestingPositions"]:::program
    n4wQQ(["4wQQ…"]):::signer
    Collection[("Collection")]:::state
    campaignAtacampaignCollectionTokeMint[("campaignAta(campaign(Collection), Toke…, Mint)")]:::state
    C7Pg(["C7Pg…"]):::signer
    n6hhA(["6hhA…"]):::signer
    n5yfA(["5yfA…"]):::signer
    splAssociatedTokenAccount["splAssociatedTokenAccount"]:::program
    token["token"]:::program
    system["system"]:::program
    CoRE["CoRE…"]:::program
    updateAuthorityCollection(["updateAuthority(Collection)"]):::signer
    n4wQQ -->|signs| VestingPositions
    VestingPositions -->|writes| Collection
    VestingPositions --> campaignAtacampaignCollectionTokeMint
    VestingPositions --> C7Pg
    VestingPositions --> n6hhA
    VestingPositions --> n5yfA
    n4wQQ --> splAssociatedTokenAccount
    splAssociatedTokenAccount --> C7Pg
    n4wQQ --> system
    C7Pg --> system
    token --> C7Pg
    n5yfA --> system
    n6hhA --> CoRE
    CoRE --> Collection
    updateAuthorityCollection --> CoRE
    n4wQQ --> CoRE
    n6hhA --> system
    system --> n6hhA
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
    linkStyle 0,6,8,9,11,12,14,15,16 stroke:#82b366
    linkStyle 1,2,3,4,5,7,10,13,17 stroke:#d79b00
```

### Ownership

```mermaid
flowchart LR
    system["system"]:::program
    n4wQQ[("4wQQ…")]:::state
    CoRE["CoRE…"]:::program
    Collection[("Collection")]:::state
    token["token"]:::program
    campaignAtacampaignCollectionTokeMint[("campaignAta(campaign(Collection), Toke…, Mint)")]:::state
    C7Pg[("C7Pg…")]:::state
    n6hhA[("6hhA…")]:::state
    VestingPositions["VestingPositions"]:::program
    n5yfA[("5yfA…")]:::state
    system -->|owns| n4wQQ
    CoRE --> Collection
    token --> campaignAtacampaignCollectionTokeMint
    token --> C7Pg
    CoRE --> n6hhA
    VestingPositions --> n5yfA
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
    linkStyle 0,1,2,3,4,5 stroke:#6c8ebf
```

### Tree

```

4wQQ… (165059cu)
├─ Comp…::? ✓
└─ VestingPositions::Claim ✓ 164909cu
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

*3196799 seconds pass.*

### T2: Claim (day 37)

| observation | before | after |
| --- | --- | --- |
| Vault balance | 10,000,000,000,000 | 9,000,000,000,000 |
| Alice balance | — | 1,000,000,000,000 |

<details>
<summary>diagrams</summary>

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
    p2-->>p1: ✓ 22492cu
    deactivate p2
    p1->>p4: transferChecked
    activate p4
    p4-->>p1: ✓ 105cu
    deactivate p4
    p1->>p2: UpdatePlugin
    activate p2
    p2-->>p1: ✓ 13574cu
    deactivate p2
    p1-->>p0: ✓ 95977cu
    deactivate p1
```

### Authority

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    n4wQQ(["4wQQ…"]):::signer
    Collection[("Collection")]:::state
    campaignAtacampaignCollectionTokeMint[("campaignAta(campaign(Collection), Toke…, Mint)")]:::state
    userAta4wQQTokeMint[("userAta(4wQQ…, Toke…, Mint)")]:::state
    n6hhA[("6hhA…")]:::state
    n5yfA[("5yfA…")]:::state
    CoRE["CoRE…"]:::program
    updateAuthorityCollection(["updateAuthority(Collection)"]):::signer
    system["system"]:::program
    token["token"]:::program
    campaignCollection(["campaign(Collection)"]):::signer
    n4wQQ -->|signs| VestingPositions
    VestingPositions -->|writes| Collection
    VestingPositions --> campaignAtacampaignCollectionTokeMint
    VestingPositions --> userAta4wQQTokeMint
    VestingPositions --> n6hhA
    VestingPositions --> n5yfA
    CoRE --> n6hhA
    CoRE --> Collection
    n4wQQ --> CoRE
    updateAuthorityCollection --> CoRE
    n4wQQ --> system
    system --> n6hhA
    token --> campaignAtacampaignCollectionTokeMint
    token --> userAta4wQQTokeMint
    campaignCollection --> token
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
    linkStyle 0,8,9,10,14 stroke:#82b366
    linkStyle 1,2,3,4,5,6,7,11,12,13 stroke:#d79b00
```

### Ownership

```mermaid
flowchart LR
    system["system"]:::program
    n4wQQ[("4wQQ…")]:::state
    CoRE["CoRE…"]:::program
    Collection[("Collection")]:::state
    token["token"]:::program
    campaignAtacampaignCollectionTokeMint[("campaignAta(campaign(Collection), Toke…, Mint)")]:::state
    userAta4wQQTokeMint[("userAta(4wQQ…, Toke…, Mint)")]:::state
    n6hhA[("6hhA…")]:::state
    VestingPositions["VestingPositions"]:::program
    n5yfA[("5yfA…")]:::state
    system -->|owns| n4wQQ
    CoRE --> Collection
    token --> campaignAtacampaignCollectionTokeMint
    token --> userAta4wQQTokeMint
    CoRE --> n6hhA
    VestingPositions --> n5yfA
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
    linkStyle 0,1,2,3,4,5 stroke:#6c8ebf
```

### Tree

```

4wQQ… (95977cu)
└─ VestingPositions::Claim ✓ 95977cu
   ├─ CoRE…::UpdatePlugin ✓ 22492cu
   │  └─ system::transferSol ✓
   ├─ token::transferChecked ✓ 105cu
   └─ CoRE…::UpdatePlugin ✓ 13574cu
```

</details>

*1 second pass.*

### T3: FirstClaim (day 38)

- [x] refused: ClaimWindowClosed — InstructionError(0, Custom(6025))

<details>
<summary>diagrams</summary>

### Sequence

```mermaid
sequenceDiagram
    participant p0 as H87x…
    participant p1 as VestingPositions
    participant p2 as splAssociatedTokenAccount
    participant p3 as token
    participant p4 as system
    p0->>p1: Claim
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
    p1->>p4: createAccount
    activate p4
    p4-->>p1: ✓
    deactivate p4
    note over p1: 🚩 custom program error  0x1789
    p1-->>p0: ✗ 61744cu
    deactivate p1
```

### Authority

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    H87x(["H87x…"]):::signer
    Collection[("Collection")]:::state
    campaignAtacampaignCollectionTokeMint[("campaignAta(campaign(Collection), Toke…, Mint)")]:::state
    DiZf(["DiZf…"]):::signer
    n5fjL[("5fjL…")]:::state
    C49G(["C49G…"]):::signer
    splAssociatedTokenAccount["splAssociatedTokenAccount"]:::program
    token["token"]:::program
    system["system"]:::program
    H87x -->|signs| VestingPositions
    VestingPositions -->|writes| Collection
    VestingPositions --> campaignAtacampaignCollectionTokeMint
    VestingPositions --> DiZf
    VestingPositions --> n5fjL
    VestingPositions --> C49G
    H87x --> splAssociatedTokenAccount
    splAssociatedTokenAccount --> DiZf
    H87x --> system
    DiZf --> system
    token --> DiZf
    C49G --> system
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
    linkStyle 0,6,8,9,11 stroke:#82b366
    linkStyle 1,2,3,4,5,7,10 stroke:#d79b00
```

### Ownership

```mermaid
flowchart LR
    system["system"]:::program
    H87x[("H87x…")]:::state
    CoRE["CoRE…"]:::program
    Collection[("Collection")]:::state
    token["token"]:::program
    campaignAtacampaignCollectionTokeMint[("campaignAta(campaign(Collection), Toke…, Mint)")]:::state
    DiZf[("DiZf…")]:::state
    n5fjL[("5fjL…")]:::state
    VestingPositions["VestingPositions"]:::program
    C49G[("C49G…")]:::state
    system -->|owns| H87x
    CoRE --> Collection
    token --> campaignAtacampaignCollectionTokeMint
    token --> DiZf
    system --> n5fjL
    VestingPositions --> C49G
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
    linkStyle 0,1,2,3,4,5 stroke:#6c8ebf
```

### Tree

```

H87x… (61744cu)
└─ VestingPositions::Claim ✗ 61744cu
   ├─ splAssociatedTokenAccount::create ✓ 13416cu
   │  ├─ token::getAccountDataSize ✓ 183cu
   │  ├─ system::createAccount ✓
   │  ├─ token::initializeImmutableOwner ✓ 38cu
   │  └─ token::initializeAccount3 ✓ 235cu
   └─ system::createAccount ✓
```

</details>
