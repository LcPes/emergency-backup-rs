#     

# Emergency Backup RS

**Emergency Backup RS** is a lightweight, Rust-based tool for macOS, designed to simplify data backups. It allows you to
configure an external drive and select up to five directories for backup using a user-friendly GUI. Backups are
triggered by drawing a rectangle in a **clockwise sense** on your screen, providing a unique and interactive experience.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
    - [Prerequisites](#prerequisites)
    - [Install Emergency Backup RS](#install-emergency-backup-rs)
- [Usage](#usage)
- [Configuration](#configuration)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)

## Features

- **macOS-Specific**: Designed exclusively for macOS, the application must be placed in `~/Applications`.
- **Configure External Drive**: Easily select the drive where your backups will be stored.
- **Directory Selection**: Choose up to 5 directories to be backed up.
- **GUI Configuration**: Simple graphical interface to select drives and directories.
- **Visual Trigger**: Initiate the backup by drawing a rectangle in a clockwise pattern on your screen.
- **Fast & Reliable**: Leverages Rust’s speed and safety features for efficient backups.

## Installation

### Prerequisites

Before installing, ensure you have [Rust](https://www.rust-lang.org/tools/install) and `cargo` installed on your macOS
system.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install Emergency Backup RS

1. Clone the repository and build the project:
   ```bash
   git clone https://github.com/yourusername/emergency-backup-rs.git
   cd emergency-backup-rs
   cargo bundle
   ```
2. Move the built application to the `~/Applications` folder:
   ```bash
   mv target/release/emergency-backup-rs.app ~/Applications/
   ```

Alternatively, you can package the application and move it to `~/Applications` if required for distribution.

## Usage

1. **Connect your external drive**.
2. **Open the application**:
    - Navigate to `~/Applications` and open **Emergency Backup RS**.
3. **Configure the external drive** through the GUI interface.
4. **Select up to 5 directories** using the graphical interface.
5. **Draw a rectangle in a clockwise sense** on your screen to trigger the backup process.

The tool will copy the files to the selected external drive and notify you when the process is complete.

## Configuration

The configuration process is handled through a simple GUI. Upon running the application, you'll be prompted to:

- Select the external drive where backups will be stored.
- Choose up to 5 directories from your system for backup.

Once configured, the settings are saved for future backups. You can update them anytime by reopening the application.

## Examples

To get started quickly, just launch the **Emergency Backup RS** app from `~/Applications`, configure your drive and
directories, and draw the rectangle on your screen in a clockwise sense to start the backup.

You can also watch a short demo [here](#).

## Contributing

We welcome contributions! To contribute, follow these steps:

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/my-feature`).
3. Commit your changes (`git commit -m 'Add new feature'`).
4. Push to the branch (`git push origin feature/my-feature`).
5. Open a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more details.
