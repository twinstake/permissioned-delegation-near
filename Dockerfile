FROM fedora:36

RUN dnf update -y
RUN dnf install @development-tools -y
RUN dnf install llvm-devel clang-devel -y

# Rust dependencies
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain none
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup install 1.70
RUN rustup target add wasm32-unknown-unknown

# WORKAROUND: Install binaryen
# This is required to deal with wasm incompatibility issues
# See: https://stackoverflow.com/questions/71943459/how-can-i-fix-error-happened-while-deserializing-the-module-error
RUN dnf --releasever=38 install binaryen -y

# Copy project files
COPY . .
