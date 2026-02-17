FROM debian:bookworm-slim AS toolchain

RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    xz-utils \
    ca-certificates \
    wget \
    gnupg \
    dotnet-sdk-8 \
    && rm -rf /var/lib/apt/lists/*

RUN curl -fsSL https://ziglang.org/download/0.12.0/zig-linux-x86_64-0.12.0.tar.xz \
    | tar -xJf - -C /opt \
    && ln -s /opt/zig-linux-x86_64-0.12.0/zig /usr/local/bin/zig

RUN dotnet tool install --global dafny --version 4.4.0
ENV PATH="${PATH}:/root/.dotnet/tools"

FROM toolchain AS builder
WORKDIR /build
COPY build.zig .
COPY src/ src/
RUN zig build -Doptimize=ReleaseSafe

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    dotnet-runtime-8 \
    curl \
    && rm -rf /var/lib/apt/lists/*

COPY --from=toolchain /usr/local/bin/zig /usr/local/bin/zig
COPY --from=toolchain /opt/zig-linux-x86_64-0.12.0 /opt/zig-linux-x86_64-0.12.0
COPY --from=toolchain /root/.dotnet/tools/dafny /usr/local/bin/dafny

COPY --from=builder /build/zig-out/bin/research-runtime /usr/local/bin/research-runtime

RUN useradd -m -u 1001 appuser
USER appuser

EXPOSE 8004
ENV PORT=8004

CMD ["/usr/local/bin/research-runtime"]
