# Train Dispatcher Game

**Train Dispatcher** is a 2D top-down train dispatching simulator built in Rust using the [Bevy game engine](https://bevyengine.org/). 
## Table of Contents

- [About the Project](#about-the-project)
- [Features](#features)
- [Gameplay](#gameplay)
- [Installation](#installation)
- [Development](#development)

---

## About the Project

Train Dispatcher is designed as a simulator for train lovers. The primary goal is to control train movements, manage track switches, and optimize traffic flow across different railway networks.

This project is built entirely in Rust and uses the Bevy game engine for rendering, physics, and overall game management.

## Features

- **2D Top-Down Gameplay**: An intuitive view that lets you see the entire rail network and manage trains easily.
- **Track Switch Control**: Operate track switches to change train paths dynamically.
- **Traffic Light Management**: Control traffic lights to manage train flow.
- **Realistic Train Movement**: Trains move along tracks with a realistic motion system.
- **Flexible Track Layout**: The game supports JSON-based track representations, allowing for easily customizable and complex track setups.

## Gameplay

In Train Dispatcher, your objective is to control train movements along tracks and manage railway switches to prevent collisions and delays. You can toggle switches, set traffic lights, and plan the best routes for efficient train dispatching.

## Installation

To play or contribute to the project, follow these steps to set up your environment:

1. **Clone the repository**:
    ```bash
    git clone https://github.com/nickyfoster/train-dispatcher.git
    cd train-dispatcher
    ```

2. **Install Rust**:
   If you haven't installed Rust, [get it here](https://www.rust-lang.org/tools/install).

3. **Build and Run**:
    ```bash
    cargo run --release
    ```

## Development

This game is currently under active development. Planned features and improvements include:

- Additional track layouts and scenarios.
- Advanced train routing logic.
- Enhanced graphics and sound effects.

### Built With

- [Rust](https://www.rust-lang.org/)
- [Bevy Game Engine](https://bevyengine.org/)

