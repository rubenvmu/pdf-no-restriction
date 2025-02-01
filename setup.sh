printf "\e[97mInstalling Rust and Cargo...\n\e[0m"

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

if [ -d "$HOME/.cargo" ] && [ -d "$HOME/.rustup" ]; then
    printf "\e[31mConfiguring environment variables...\n\e[0m"
    
    export CARGO_HOME="$HOME/.cargo"
    export RUSTUP_HOME="$HOME/.rustup"
    
    if ! grep -q "$CARGO_HOME/bin" "$HOME/.bashrc"; then
        echo "export CARGO_HOME=$HOME/.cargo" >> "$HOME/.bashrc"
        echo "export RUSTUP_HOME=$HOME/.rustup" >> "$HOME/.bashrc"
        echo "export PATH=\$CARGO_HOME/bin:\$PATH" >> "$HOME/.bashrc"
  
        source "$HOME/.bashrc"
        
        printf "\e[32mEnvironment variables configured successfully.\n\e[0m"
    else
        printf "\e[33mEnvironment variables already configured in .bashrc.\n\e[0m"
    fi
else
    printf "\e[31mError: Rust installation failed. Please check the installation process.\n\e[0m"
    exit 1
fi

chmod +x pdfnorestriction.sh
printf "\e[32mpdfnorestriction.sh is now executable. Use ./pdfnorestriction\n\e[0m"

printf "\e[97mInstallation complete! You can now use Rust and Cargo.\n\e[0m"
