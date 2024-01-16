#NOTE: this hasnt been tested. this is copied from https://github.com/MinaMatta98/Leptos-Chatting-Client/blob/main/Dockerfile as an example

# Use the official Rust nightly image as the base image
FROM rustlang/rust:nightly

# Set the working directory inside the container
WORKDIR /app

VOLUME ./storage/
# Copy the project files into the container
COPY . .

# Install system dependencies
RUN apt update 

RUN apt-get install -y \
	sudo\
	lsb-release\
	cmake\
	nasm\
	expect\
	wget\
	curl\
	gpg\
	gnupg\
  pkg-config\
  libssl-dev

# Alloc. Env. Vars
ENV LEPTOS_OUTPUT_NAME "kakichat"
# ENV LEPTOS_SITE_ROOT "site"
# ENV LEPTOS_SITE_PKG_DIR "pkg"
ENV LEPTOS_SITE_ADDR "0.0.0.0:7770"
# ENV LEPTOS_RELOAD_PORT "3001"
#not used (yet)
ENV WEBSOCKET_ADDR "141.145.204.255"

# Build the project via Cargo Leptos
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos
# RUN cargo +nightly build --release
RUN cargo leptos build --release

# Expose the necessary ports <=== Documentation Only. Run with -p 8000:8000
EXPOSE 8000

# Run the project
CMD ["cargo", "leptos", "watch", "--release"]
