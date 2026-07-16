# close_receipt_returns_rent_after_close_campaign

**Source:** [`tests/clawback.rs` L355](https://github.com/cds-turbin3/vesting-position/blob/c4f43acad0bb9f007622fb43bc19ddc3610c7688/babelfish-tests/tests/clawback.rs#L355)

```mermaid
sequenceDiagram
    participant p0 as 4wQQ…
    participant p1 as VestingPositions
    p0->>p1: CloseReceipt
    activate p1
    p1-->>p0: ✓ 3258cu
    deactivate p1
```

```mermaid
flowchart LR
    VestingPositions["VestingPositions"]:::program
    4wQQ(["4wQQ…"]):::signer
    7kmN[("7kmN…")]:::state
    4wQQ -->|signs| VestingPositions
    VestingPositions -->|writes| 7kmN
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

```mermaid
flowchart LR
    system["system"]:::program
    4wQQ[("4wQQ…")]:::state
    7kmN[("7kmN…")]:::state
    system -->|owns| 4wQQ
    system -->|owns| 7kmN
    classDef program fill:#dae8fc,stroke:#6c8ebf;
    classDef signer fill:#d5e8d4,stroke:#82b366;
    classDef state fill:#ffe6cc,stroke:#d79b00;
```

<details>
<summary>tree</summary>

```

4wQQ… (3258cu)
└─ VestingPositions::CloseReceipt ✓ 3258cu
```

</details>
