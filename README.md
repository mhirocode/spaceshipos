# ShipOS – Spaceship Cargo Operating System

## 日本語 / Japanese

ShipOS は、Rustで開発する宇宙貨物船向けソフトウェアの研究・シミュレーションプロジェクトです。

本プロジェクトでは、仮想的な宇宙船ハードウェア上で動作するソフトウェアを実装し、実際の宇宙船コンピュータを参考にしたシンプルなシステムアーキテクチャを設計・検証します。

ゲーム開発ではなく、モジュール設計、ハードウェア抽象化、リアルタイム処理を中心としたソフトウェア開発を目的としています。

---

## English

ShipOS is a Rust-based research and simulation project for spacecraft software.

The project develops software running on virtual spacecraft hardware to explore system architectures inspired by real spacecraft computers.

Rather than building a game, ShipOS focuses on modular design, hardware abstraction, and real-time processing.

---

# Project Goals / プロジェクトの目標

### English

- Develop a modular spacecraft software architecture
- Emulate basic spacecraft hardware
- Research hardware abstraction
- Build a simple real-time simulation

### 日本語

- モジュール化された宇宙船ソフトウェアの開発
- 基本的な宇宙船ハードウェアのエミュレーション
- ハードウェア抽象化の研究
- シンプルなリアルタイムシミュレーションの構築

---

# System Architecture / システム構成

```text
Space Simulator
        │
        ▼
Virtual Hardware
        │
        ▼
ShipOS Kernel
        │
        ▼
Services
        ├── Flight Computer
        ├── Navigation
        └── Cargo Management
```

The operating system communicates only with the Virtual Hardware layer, allowing the simulator and software to evolve independently.

OSは Virtual Hardware 層のみを認識し、シミュレータとソフトウェアを独立して開発できる構成を採用します。

---

# Planned Features / 主な機能

- 🚀 Flight Computer
- 🛰 Navigation
- 📦 Cargo Management
- 🔄 Inter-process Communication (IPC)
- ⏱ Tick-based Scheduler

---

# Technology Stack / 技術構成

- Rust
- Cargo
- Python (development tools)

---

# Build / ビルド

```bash
cargo build
```

---

# Run / 実行

```bash
cargo run
```

---

# Project Structure / ディレクトリ構成

```text
spaceship/
├── Cargo.toml
├── README.md
├── LICENSE
│
├── docs/
│   ├── architecture.md
│   ├── requirements.md
│   └── roadmap.md
│
└── src/
    ├── main.rs
    ├── kernel/
    ├── sim/
    ├── hal/
    ├── services/
    └── common/
```

---

# Development Roadmap / 開発ロードマップ

## Phase 1 – Foundation

- [x] Git repository
- [x] Rust project
- [ ] Documentation
- [ ] Simulation clock
- [ ] Tick scheduler

## Phase 2 – Core System

- [ ] Virtual hardware
- [ ] Flight computer
- [ ] Navigation
- [ ] IPC

## Phase 3 – Expansion

- [ ] Cargo management
- [ ] Additional services
- [ ] Testing
- [ ] Documentation

---

# Design Principles / 設計方針

### English

- Keep the project simple
- Modular architecture
- Hardware abstraction
- Tick-driven simulation
- Incremental development
- Reliability over complexity

### 日本語

- できるだけシンプルに完成させる
- モジュール化された設計
- ハードウェア抽象化
- ティック駆動型シミュレーション
- 段階的な開発
- 複雑さより信頼性を重視

---

# License / ライセンス

MIT License

---

# Author / 作者

GitHub: **mhirocode**

---

# Community / コミュニティ

Contributions, ideas, and discussions are welcome in both Japanese and English.

### Discord

💬 Join our Discord community!

https://discord.gg/cszX2vgNCS