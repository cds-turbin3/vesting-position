# clawback_unclaimed_succeeds_despite_buyers_zeroed_receipt

**Source:** [`tests/clawback.rs` L164](https://github.com/cds-turbin3/vesting-position/blob/c89b5ba9f2140371af02754735f2eed3bb87d0ec/babelfish-tests/tests/clawback.rs#L164)

Over 38 days: 5 moments.

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
| userAta(H87x…, Toke…, Mint) | DiZfCmUXMWffwHP4eus9hwgnvTVbiZL7ZGGTBjJ9vc68 |
| 4wQQ… | 4wQQJM9LNuhinieNAqmHuPCm8LXDTVfhx84P32nAVE9P |
| 5yfA… | 5yfASCcX25V4fzBgdfixtXgS2JrvpWdwX27JQXpVyuHB |
| 6hhA… | 6hhAjXPGt41Y6oPH6mATnAqMECdaHrKaAGbE4SFuARXK |
| C49G… | C49GR57waDzt1nFUvxKPDs1HVAKF694inGJeB2KGRPM9 |
| C7Pg… | C7PguAKs34J4WkXRRp762bPFhzhmFBYKdLuHQYBrVmAW |
| CoRE… | CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d |
| Comp… | ComputeBudget111111111111111111111111111111 |
| H87x… | H87xi4CUqrUPXzppV3jotTmre6DyR5pCaMk5bKQQBFTg |

</details>

### Timeline

| time | Vault balance | Δ | Creator balance | Δ | Alice balance | Δ | Bob balance | Δ | comment |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| T0 | 10,000,000,000,000 | +10,000,000,000,000 | 0 |  | — |  | — |  | Initialize (day 0) |
| T1 | 10,000,000,000,000 |  | 0 |  | — |  | — |  | FirstClaim (day 1) |
| T2 | 10,000,000,000,000 |  | 0 |  | 0 |  | — |  | TransferPosition (day 1) |
| T3 | 9,450,000,000,000 | -550,000,000,000 | 0 |  | 0 |  | — |  | Claim (day 16) |
| T4 | 7,450,000,000,000 | -2,000,000,000,000 | 2,000,000,000,000 | +2,000,000,000,000 | 0 |  | 550,000,000,000 | +550,000,000,000 | ClawbackUnclaimed (day 38) |

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

### T2: TransferPosition (day 1)

| observation | before | after |
| --- | --- | --- |
| Alice balance | — | 0 |

<details>
<summary>diagrams</summary>

### Sequence

```mermaid
sequenceDiagram
    participant p0 as 4wQQ…
    participant p1 as CoRE…
    p0->>p1: Transfer
    activate p1
    p1-->>p0: ✓ 9074cu
    deactivate p1
```

### Authority

```mermaid
flowchart LR
    CoRE["CoRE…"]:::program
    n6hhA[("6hhA…")]:::state
    n4wQQ(["4wQQ…"]):::signer
    CoRE -->|writes| n6hhA
    n4wQQ -->|signs| CoRE
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
    linkStyle 0 stroke:#d79b00
    linkStyle 1 stroke:#82b366
```

### Ownership

```mermaid
flowchart LR
    CoRE["CoRE…"]:::program
    n6hhA[("6hhA…")]:::state
    system["system"]:::program
    n4wQQ[("4wQQ…")]:::state
    CoRE -->|owns| n6hhA
    system --> n4wQQ
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
    linkStyle 0,1 stroke:#6c8ebf
```

### Tree

```

4wQQ… (9074cu)
└─ CoRE…::Transfer ✓ 9074cu
```

</details>

*1339200 seconds pass.*

### T3: Claim (day 16)

| observation | before | after |
| --- | --- | --- |
| Vault balance | 10,000,000,000,000 | 9,450,000,000,000 |

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
    participant p5 as CoRE…
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
    p1->>p5: UpdatePlugin
    activate p5
    p5->>p4: transferSol
    activate p4
    p4-->>p5: ✓
    deactivate p4
    p5-->>p1: ✓ 22502cu
    deactivate p5
    p1->>p3: transferChecked
    activate p3
    p3-->>p1: ✓ 105cu
    deactivate p3
    p1-->>p0: ✓ 115304cu
    deactivate p1
```

### Authority

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    H87x(["H87x…"]):::signer
    Collection[("Collection")]:::state
    campaignAtacampaignCollectionTokeMint[("campaignAta(campaign(Collection), Toke…, Mint)")]:::state
    userAtaH87xTokeMint(["userAta(H87x…, Toke…, Mint)"]):::signer
    n6hhA[("6hhA…")]:::state
    C49G(["C49G…"]):::signer
    splAssociatedTokenAccount["splAssociatedTokenAccount"]:::program
    token["token"]:::program
    system["system"]:::program
    CoRE["CoRE…"]:::program
    updateAuthorityCollection(["updateAuthority(Collection)"]):::signer
    campaignCollection(["campaign(Collection)"]):::signer
    H87x -->|signs| VestingPositions
    VestingPositions -->|writes| Collection
    VestingPositions --> campaignAtacampaignCollectionTokeMint
    VestingPositions --> userAtaH87xTokeMint
    VestingPositions --> n6hhA
    VestingPositions --> C49G
    H87x --> splAssociatedTokenAccount
    splAssociatedTokenAccount --> userAtaH87xTokeMint
    H87x --> system
    userAtaH87xTokeMint --> system
    token --> userAtaH87xTokeMint
    C49G --> system
    CoRE --> n6hhA
    CoRE --> Collection
    H87x --> CoRE
    updateAuthorityCollection --> CoRE
    system --> n6hhA
    token --> campaignAtacampaignCollectionTokeMint
    campaignCollection --> token
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
    linkStyle 0,6,8,9,11,14,15,18 stroke:#82b366
    linkStyle 1,2,3,4,5,7,10,12,13,16,17 stroke:#d79b00
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
    userAtaH87xTokeMint[("userAta(H87x…, Toke…, Mint)")]:::state
    n6hhA[("6hhA…")]:::state
    VestingPositions["VestingPositions"]:::program
    C49G[("C49G…")]:::state
    system -->|owns| H87x
    CoRE --> Collection
    token --> campaignAtacampaignCollectionTokeMint
    token --> userAtaH87xTokeMint
    CoRE --> n6hhA
    VestingPositions --> C49G
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
    linkStyle 0,1,2,3,4,5 stroke:#6c8ebf
```

### Tree

```

H87x… (115304cu)
└─ VestingPositions::Claim ✓ 115304cu
   ├─ splAssociatedTokenAccount::create ✓ 13416cu
   │  ├─ token::getAccountDataSize ✓ 183cu
   │  ├─ system::createAccount ✓
   │  ├─ token::initializeImmutableOwner ✓ 38cu
   │  └─ token::initializeAccount3 ✓ 235cu
   ├─ system::createAccount ✓
   ├─ CoRE…::UpdatePlugin ✓ 22502cu
   │  └─ system::transferSol ✓
   └─ token::transferChecked ✓ 105cu
```

</details>

*1857601 seconds pass.*

### T4: ClawbackUnclaimed (day 38)

| observation | before | after |
| --- | --- | --- |
| Vault balance | 9,450,000,000,000 | 7,450,000,000,000 |
| Creator balance | 0 | 2,000,000,000,000 |
| Bob balance | — | 550,000,000,000 |

<details>
<summary>diagrams</summary>

### Sequence

```mermaid
sequenceDiagram
    participant p0 as Creator
    participant p1 as VestingPositions
    participant p2 as token
    p0->>p1: ClawbackUnclaimed
    activate p1
    p1->>p2: transferChecked
    activate p2
    p2-->>p1: ✓ 105cu
    deactivate p2
    p1-->>p0: ✓ 116625cu
    deactivate p1
```

### Authority

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    Creator(["Creator"]):::signer
    C49G[("C49G…")]:::state
    campaignAtacampaignCollectionTokeMint[("campaignAta(campaign(Collection), Toke…, Mint)")]:::state
    creatorAtaCreatorTokeMint[("creatorAta(Creator, Toke…, Mint)")]:::state
    token["token"]:::program
    campaignCollection(["campaign(Collection)"]):::signer
    Creator -->|signs| VestingPositions
    VestingPositions -->|writes| C49G
    VestingPositions --> campaignAtacampaignCollectionTokeMint
    VestingPositions --> creatorAtaCreatorTokeMint
    token --> campaignAtacampaignCollectionTokeMint
    token --> creatorAtaCreatorTokeMint
    campaignCollection --> token
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
    linkStyle 0,6 stroke:#82b366
    linkStyle 1,2,3,4,5 stroke:#d79b00
```

### Ownership

```mermaid
flowchart LR
    system["system"]:::program
    Creator[("Creator")]:::state
    VestingPositions["VestingPositions"]:::program
    C49G[("C49G…")]:::state
    token["token"]:::program
    campaignAtacampaignCollectionTokeMint[("campaignAta(campaign(Collection), Toke…, Mint)")]:::state
    creatorAtaCreatorTokeMint[("creatorAta(Creator, Toke…, Mint)")]:::state
    system -->|owns| Creator
    VestingPositions --> C49G
    token --> campaignAtacampaignCollectionTokeMint
    token --> creatorAtaCreatorTokeMint
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
    linkStyle 0,1,2,3 stroke:#6c8ebf
```

### Tree

```

Creator (116625cu)
└─ VestingPositions::ClawbackUnclaimed ✓ 116625cu
   └─ token::transferChecked ✓ 105cu
```

</details>
