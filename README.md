# my-vehcialpayment# 
🚗 Vehicle Payment Smart Contract (Soroban - Stellar)

## 📌 Project Description

This project is a Soroban smart contract built on the Stellar blockchain that enables secure and transparent vehicle payment tracking between buyers and sellers.

The contract allows users to record payment transactions associated with a unique vehicle ID, ensuring trust and traceability in vehicle deals.

<img width="1920" height="1080" alt="Screenshot 2026-04-13 144643" src="https://github.com/user-attachments/assets/795f0f1a-ce49-4ae5-8d15-bbd6023d1033" />


---

## ⚙️ What it does

* Records a payment transaction between a buyer and a seller
* Links each payment to a unique vehicle ID
* Stores transaction data securely on-chain
* Allows retrieval of payment details at any time

---

## ✨ Features

* 🔐 Secure authentication (buyer authorization required)
* 🚘 Vehicle-specific transaction tracking
* 💾 On-chain storage for transparency and immutability
* 🔍 Easy retrieval of payment records
* ⚡ Lightweight and efficient Soroban smart contract

---

## 🔗 Deployed Smart Contract Link

**Contract Address:**
`CCZXV53VEFVZ457DG3NZQSJ35RDWXW5KMLYSBAVFA5YLIBLLNJGTBNEA`
https://stellar.expert/explorer/testnet/contract/CCZXV53VEFVZ457DG3NZQSJ35RDWXW5KMLYSBAVFA5YLIBLLNJGTBNEA
---

## 🛠️ Tech Stack

* Rust (Soroban SDK)
* Stellar Blockchain

---

## 📦 Contract Functions

### 1. `make_payment`

Stores a vehicle payment record.

**Parameters:**

* `buyer` (Address)
* `seller` (Address)
* `vehicle_id` (Symbol)
* `amount` (i128)

---

### 2. `get_payment`

Retrieves payment details for a given vehicle.

**Parameters:**

* `vehicle_id` (Symbol)

**Returns:**

* `(buyer, seller, amount)` or `None`

---

## 🚀 Future Improvements

* 💰 Integration with real token transfers (XLM / USDC)
* 🔒 Escrow-based payment security
* 📜 Vehicle ownership verification system
* ⏱️ Timestamp and transaction status tracking
* 📊 User transaction history dashboard

---

## 📜 License

MIT License
