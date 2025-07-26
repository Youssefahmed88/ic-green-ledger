# 🏗️ System Architecture

This document provides a professional, modular, and extensible **System Design Specification** for a decentralized crop insurance platform built on the **Internet Computer Protocol (ICP)**. The system targets small-scale farmers in regions like **Malawi**, offering seasonal insurance against climate-related events like droughts or heavy rainfall.

The system architecture separates concerns across several **Rust-based canisters** and **native ICP services** for scalability, auditability, and complete decentralization.

---

## 🧱 Canister Architecture

### 1️⃣ Main Canister – `policy_manager`
Manages the lifecycle of crop insurance policies using Rust and IC-CDK:

#### Core Responsibilities:
- Create new policies (full-season or sub-season)
- Manage farmer subscriptions and registrations
- Enforce subscription constraints and timing
- Coordinate with Treasury & Payout canisters
- Handle Internet Identity authentication

#### Key Features:
- Only one **full-season** policy per farmer per season
- Sub-seasonal policies can be purchased if full-season wasn't
- Subscription has deadlines aligned with Malawi crop calendar
- Integrates with Treasury canister for fund management
- Tracks policy status (Active, Paused, PayoutTriggered) in stable memory
- Cross-canister calls to trigger payouts
- Policy data persists across canister upgrades

#### Data Structures:
- Policy records with farmer identification
- Policy types and seasonal coverage
- Premium and coverage amounts
- Status tracking through policy lifecycle

---

### 2️⃣ Treasury Canister – `treasury_manager`
Handles all funds securely using ICP's native token standards.

#### Core Responsibilities:
- Receive premium payments in ICP tokens or ckUSDC
- Secure fund storage in canister cycles and tokens
- Release funds to `payout_engine` on valid payout requests
- Implement multi-signature controls for large payouts
- Track all financial transactions in stable memory

#### ICP Token Integration:
- **ICP Tokens**: Native protocol tokens for premium payments
- **ckUSDC**: Chain-key USD Coin for stable value payouts
- **ICRC-1/ICRC-2**: Standard token interfaces for transfers
- **Cycles**: For canister computational costs

#### Security Features:
- Principal-based access control
- Time-locked withdrawals for large amounts
- Audit trail of all transactions
- Automatic backup to stable memory

---

### 3️⃣ Payout Engine Canister – `payout_engine`
Coordinates payout distribution based on policy triggers.

#### Core Responsibilities:
- Receives trigger requests from `weather_oracle`
- Validates payout conditions against policy terms
- Executes cross-canister calls to Treasury for fund release
- Notifies `policy_manager` to mark policies as triggered
- Maintains payout history and analytics

#### Payout Processing:
- Receives and validates trigger requests from weather oracle
- Calculates payout amounts based on policy terms and trigger severity
- Executes cross-canister calls to Treasury for secure fund release
- Updates policy status and maintains comprehensive payout history

---

### 4️⃣ Weather Oracle Canister – `weather_oracle`
Fetches and validates weather data using ICP's native HTTPS outcalls.

#### Core Responsibilities:
- Makes direct **HTTPS outcalls** to weather APIs (no external oracles needed)
- Processes rainfall and temperature data for Malawi regions
- Implements **periodic monitoring** using ICP timers
- Validates data integrity across multiple sources
- Triggers payout logic when drought conditions are met

#### Weather Data Sources:
- Malawi Department of Climate Change and Meteorological Services
- OpenWeatherMap API
- NASA's POWER API
- Local weather station networks

#### Weather Monitoring Process:
- **Direct API Integration**: Makes HTTPS outcalls to multiple weather services without intermediaries
- **Data Processing**: Analyzes rainfall patterns, temperature data, and drought indicators specific to Malawi's climate
- **Automated Scheduling**: Uses ICP's native timer system for continuous monitoring during crop seasons
- **Multi-Source Validation**: Cross-references data from multiple APIs to ensure accuracy and prevent manipulation
- **Trigger Logic**: Evaluates drought conditions against policy parameters and initiates payout process when thresholds are met

---

### 5️⃣ NFT Canister – `policy_nft`
Mints unique NFTs to represent farmer policy subscriptions using ICRC-7 standard.

#### Core Responsibilities:
- Mint NFTs on successful policy subscription
- Store rich metadata about policies and farmers
- Enable policy transfer and secondary markets (if permitted)
- Provide visual proof of insurance coverage

#### NFT Metadata Components:
- **Farmer Identity**: Principal-based identification linked to Internet Identity
- **Policy Details**: Coverage type, season, amounts, and terms
- **Farm Information**: Geographic coordinates and crop details
- **Visual Elements**: Custom artwork representing policy status and coverage level
- **Timestamps**: Issue date and policy period for verification

---

### 6️⃣ Asset Storage Canister – `asset_storage`
Manages static assets and rich media content.

#### Core Responsibilities:
- Store policy documents and images
- Serve NFT artwork and metadata
- Handle multilingual content (English/Chichewa)
- Provide CDN-like asset delivery

---

## 🔁 System Workflow

```plaintext
Farmer connects via Internet Identity
      ↓
PolicyManager displays available policies for current season
      ↓
Farmer subscribes to policy (PolicyManager → Treasury)
      ↓
NFT minted with policy details (PolicyNFT)
      ↓
WeatherOracle monitors conditions (Native HTTPS + Timers)
      ↓
Drought Detected → PayoutEngine validates conditions
      ↓
Treasury releases ICP/ckUSDC to farmer's wallet
      ↓
Policy marked as PayoutTriggered
      ↓
Farmer receives instant notification
```

---

## 🔒 Security Measures

### Canister-Level Security:
- **Principal-based access control**: Only authorized canisters can make cross-canister calls
- **Stable memory persistence**: Critical data survives canister upgrades
- **Input validation**: All external inputs validated before processing
- **Rate limiting**: Prevent spam and DoS attacks
- **Timer-based automation**: Removes human intervention points

### Financial Security:
- **Multi-signature payouts**: Large payments require multiple approvals
- **Time-locked withdrawals**: Prevent rapid fund drainage
- **Audit trails**: Complete transaction history in stable memory
- **Token standard compliance**: Use proven ICRC standards

### Data Integrity:
- **Multiple weather sources**: Cross-validate data from different APIs
- **Cryptographic verification**: Hash-based data integrity checks
- **Deterministic execution**: Consistent results across replica nodes

---

## 🔗 ICP Integration Summary

| Service | ICP Feature | Implementation |
|---------|-------------|----------------|
| Weather Data | HTTPS Outcalls | Direct API calls to weather services |
| Automation | IC Timers | Periodic drought monitoring |
| Authentication | Internet Identity | Passwordless farmer login |
| Payments | ICP/ckUSDC Tokens | Native token transfers |
| Storage | Stable Memory | Persistent policy data |
| Frontend | Asset Canister | Full-stack on-chain hosting |
| NFTs | ICRC-7 Standard | Policy representation |

---

## 🌍 Real-World Considerations

### Malawi-Specific Adaptations:
- **Crop calendar alignment**: Policy timing matches local growing seasons
- **Regional weather zones**: Support for different agro-ecological regions
- **Local currency integration**: Future Malawi Kwacha stablecoin support
- **Mobile-first design**: Optimized for smartphone access
- **Offline capabilities**: Progressive Web App with limited offline functionality

### Farmer Experience:
- **Simple onboarding**: Internet Identity eliminates complex wallet setup
- **Visual policy proof**: NFT displays policy status and coverage
- **Real-time updates**: Dashboard shows weather conditions and policy status
- **Instant payouts**: Sub-second transaction finality
- **Zero transaction fees**: Farmers don't pay gas fees

---

## 📦 Canisters & Interfaces Summary

| Canister | Purpose | Language | Key Dependencies |
|----------|---------|----------|------------------|
| `policy_manager` | Core insurance logic | Rust | ic-cdk, ic-stable-structures |
| `treasury_manager` | Secure fund management | Rust | icrc-1, cycles-minting-canister |
| `payout_engine` | Validate & distribute payouts | Rust | ic-cdk-timers |
| `weather_oracle` | Weather data & monitoring | Rust | http_request, serde_json |
| `policy_nft` | ICRC-7 policy NFTs | Rust | icrc-7