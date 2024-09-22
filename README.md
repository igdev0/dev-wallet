# Dev Wallet
The goal of this project is to develop a wallet which empowers developers to develop dapps faster. The features of this project are still in development, these are the initial features which I planned to implement:
- **Authentication**: Pretty straight forward, it will enable you to authenticate on your wallet, via some GUI;
- **Wallet management**: Thsi feature will let you to create and delete wallets (and associated accounts) and update certain things like wallet name, via some GUI;
- **Account management** : This feature will let you derrive accounts from the wallet seed, via some GUI;
  
## Supported Platforms

The Dev Wallet will be available on the following platforms:

- macOS
- Windows
- Linux

## Technology Stack

This project is built using the following technologies:

- **Rust:** Core wallet functionality is implemented in Rust for performance and security.
- **Tauri:** Desktop application framework used to build cross-platform native applications.
- **React:** Frontend UI is developed using React.

## Getting Started

### Prerequisites

Before you can run or contribute to this project, ensure that you have the following installed:

- **Rust:** [Install Rust](https://www.rust-lang.org/tools/install)
- **Node.js:** [Install Node.js](https://nodejs.org/)
- **Tauri CLI:** Install via Cargo
    ```bash
    cargo install tauri-cli
    ```

### Installation

1. Clone the repository:
    ```bash
    git clone https://github.com/yourusername/hd-dev-wallet.git
    ```
2. Navigate to the project directory:
    ```bash
    cd hd-dev-wallet
    ```
3. Install the necessary dependencies:
    ```bash
    yarn install
    ```

### Running the Application

To start the application on your platform:

1. Build the Tauri application:
    ```bash
    npm run tauri build
    ```
2. Run the development version:
    ```bash
    npm run tauri dev
    ```

## License

This project is licensed under the MIT - see the [LICENSE](LICENSE) file for details.

## Contact

For any inquiries or feedback, please open an issue on GitHub.
