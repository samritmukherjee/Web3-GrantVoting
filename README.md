# 🌌 Grant Voting Smart Contract (Soroban)

<img width="1916" height="915" alt="image" src="https://github.com/user-attachments/assets/1caaee1e-d339-4237-8f3a-ecb064ae53d0" />


URL: https://stellar.expert/explorer/testnet/contract/CCM74GXUP37AEEPVGE6L3N5LDGCIXQSA4H36QUZEVHTL3J6LGDLFXPKA

Contact Address: CCM74GXUP37AEEPVGE6L3N5LDGCIXQSA4H36QUZEVHTL3J6LGDLFXPKA


## 📌 Project Description  
Grant Voting is a decentralized smart contract built using **Soroban (Stellar Smart Contracts)** that enables transparent and secure voting for funding proposals.

In many organizations and communities, grant allocation is often centralized and lacks transparency. This project solves that by moving the entire voting process on-chain, ensuring fairness, immutability, and trust.

---

## ⚙️ What it does  

This smart contract allows users to:

- 🆕 Create grant proposals with a title and description  
- 🗳️ Vote for proposals using their Stellar wallet  
- 🚫 Prevent duplicate voting (one user = one vote per proposal)  
- 📊 View proposal details and vote counts  
- 🏆 Automatically determine the winning proposal  

All data is stored on-chain, making the system transparent and tamper-proof.

---

## ✨ Features  

### 🗳️ Decentralized Voting  
- No central authority  
- Fully on-chain voting system  

### 👤 Secure Authentication  
- Uses Soroban’s `require_auth()`  
- Ensures only authorized users can vote  

### 🚫 Anti Double Voting  
- Tracks voter participation per proposal  
- Prevents duplicate votes  

### 📊 Transparency  
- All proposals and votes are publicly verifiable  
- Immutable blockchain records  

### 🏆 Winner Selection  
- Automatically finds the proposal with highest votes  
- Can be extended for grant distribution  

---

## 🛠️ Tech Stack  

- **Blockchain:** Stellar  
- **Smart Contracts:** Soroban  
- **Language:** Rust  
- **SDK:** soroban-sdk  

---

## 📂 Contract Functions  

| Function | Description |
|----------|------------|
| `create_proposal` | Creates a new grant proposal |
| `vote` | Casts a vote for a proposal |
| `get_proposal` | Fetches proposal details |
| `get_count` | Returns total number of proposals |
| `get_winner` | Returns the highest voted proposal |

---

## 🚀 How it works  

1. A user creates a proposal  
2. Each proposal gets a unique ID  
3. Users vote using their wallet (authenticated)  
4. Votes are stored securely on-chain  
5. The contract calculates the winner based on votes  

---

## 🔮 Future Enhancements  

- ⏳ Voting deadlines & time-based restrictions  
- 🪙 Token-weighted voting  
- 👥 Whitelisted voters  
- 💰 Automatic fund distribution to winners  
- 🌐 Frontend dashboard (React + Soroban SDK)  

---

## 🧪 Testing  

You can write and run tests using Soroban’s testing framework inside `test.rs`.

---

## 📜 License  
This project is open-source and available under the **MIT License**.
