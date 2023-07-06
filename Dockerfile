FROM fedora:36

RUN dnf update -y
RUN dnf install @development-tools -y
RUN dnf install llvm-devel clang-devel -y

# Rust dependencies
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain none
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup install 1.69
RUN rustup target add wasm32-unknown-unknown

# Copy project files
COPY . .
