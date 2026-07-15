# ShipOS – Spaceship Cargo Operating System

**日本語 / Japanese**

ShipOS は、Rustで開発する宇宙貨物船向けオペレーティングシステム（OS）の研究・シミュレーションプロジェクトです。

本プロジェクトでは、宇宙貨物船をPC上でエミュレーションし、その上で動作する独自OSを段階的に設計・実装します。

ゲーム開発ではなく、実際の宇宙船コンピュータを参考にしたソフトウェアアーキテクチャ、リアルタイム処理、ハードウェア抽象化、安全性および保守性を重視した設計を目指しています。

現段階ではシミュレータとして開発を進めていますが、将来的には実機へ応用可能なソフトウェアアーキテクチャの研究プラットフォームとなることを目標としています。

---

**English**

ShipOS is a research project to develop a Rust-based operating system for autonomous cargo spacecraft.

The project emulates an entire spacecraft on a PC and develops an operating system that runs on top of the simulated hardware.

Rather than being a game, ShipOS focuses on software architecture inspired by real spacecraft engineering, emphasizing modularity, hardware abstraction, fault tolerance, maintainability, and safety.

The current implementation is a simulator, with the long-term vision of becoming a research platform for spacecraft software architectures.

---

# Project Goals / プロジェクトの目標

* Develop a modular spacecraft operating system

* Emulate spacecraft hardware and onboard computers

* Research microkernel-oriented software architecture

* Design reliable and fault-tolerant spacecraft software

* Implement realistic spacecraft simulation

* Separate hardware, kernel, and application layers

* Build a platform for future autonomous spacecraft research

* 宇宙貨物船向けOSの開発

* 宇宙船ハードウェアおよび搭載コンピュータのエミュレーション

* マイクロカーネル志向のOS設計

* 高信頼・耐故障システムの研究

* 現実的な宇宙船シミュレーション

* ハードウェア・カーネル・アプリケーションの分離

* 将来の自律宇宙船研究プラットフォームの構築

---

# System Architecture / システム構成

```text
Space Simulator
        │
        ▼
Physics Engine
        │
        ▼
Virtual Hardware
        │
        ▼
ShipOS Kernel
        │
        ▼
Userspace Services
        │
        ├── Flight Computer
        ├── Navigation Computer
        ├── Cargo Computer
        ├── Power Computer
        ├── Communication Computer
        └── AI Computer
```

The operating system interacts only with the Virtual Hardware layer, allowing the physics simulator and onboard software to evolve independently.

OSはVirtual Hardware層のみを認識し、物理シミュレータとは分離された構造を採用します。

---

# Planned Features / 主な機能

* 🚀 Flight computer
* 📦 Cargo management
* ⚡ Power management
* 🔥 Propulsion control
* 🛰 Navigation system
* 📡 Sensor simulation
* 🌌 Orbital mechanics
* 🤖 Autonomous flight
* 🛡 Fault detection and recovery
* 📝 Flight logging
* 🔒 System monitoring
* 🔄 Inter-process communication (IPC)
* ⏱ Tick-based scheduler

---

# Technology Stack / 技術構成

* Rust (main language)
* Cargo
* Python (development tools and testing)
* C / Assembly (future bootloader experiments)

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
# schedule / 予定です

```text
spaceship/
├── Cargo.toml
├── README.md
├── LICENSE
├── docs/
│   ├── architecture.md
│   ├── requirements.md
│   └── roadmap.md
│
├── src/
│   ├── main.rs
│   ├── kernel/
│   │   ├── scheduler.rs
│   │   ├── clock.rs
│   │   └── ipc.rs
│   │
│   ├── sim/
│   │   ├── physics.rs
│   │   ├── world.rs
│   │   └── time.rs
│   │
│   ├── hal/
│   │   ├── thruster.rs
│   │   ├── battery.rs
│   │   └── sensor.rs
│   │
│   ├── services/
│   │   ├── flight.rs
│   │   ├── navigation.rs
│   │   ├── cargo.rs
│   │   ├── power.rs
│   │   ├── communication.rs
│   │   └── ai.rs
│   │
│   └── common/
```

---

# Development Roadmap / 開発ロードマップ

## Phase 1 – Foundation

* [x] Git repository
* [x] Rust project
* [x] Initial ship structure
* [ ] Documentation
* [ ] Simulation clock
* [ ] Tick system

## Phase 2 – Kernel

* [ ] Scheduler
* [ ] Computer trait
* [ ] IPC
* [ ] Service framework

## Phase 3 – Virtual Hardware

* [ ] Thruster
* [ ] Battery
* [ ] Sensors
* [ ] Cargo hold

## Phase 4 – Physics

* [ ] Orbital mechanics
* [ ] Fuel consumption
* [ ] Attitude control
* [ ] Collision detection

## Phase 5 – Flight Systems

* [ ] Navigation
* [ ] Flight computer
* [ ] Autonomous flight
* [ ] Fault management

## Phase 6 – Future

* [ ] GUI
* [ ] Networking
* [ ] Multi-spacecraft simulation
* [ ] Mission scripting
* [ ] AI computer

---

# Design Principles / 設計方針

* Microkernel-oriented architecture
* Modular design
* Hardware abstraction
* Tick-driven simulation
* Reliability over complexity
* Safety-first development
* Incremental implementation with small Git commits
* マイクロカーネル指向アーキテクチャ
* モジュール式設計
* ハードウェア抽象化
* ティック駆動型シミュレーション
* 複雑さよりも信頼性を重視
* 安全性を最優先した開発
* 小規模なGitコミットによる段階的な実装

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
