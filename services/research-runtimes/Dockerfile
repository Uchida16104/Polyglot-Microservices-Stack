FROM python:3.12-slim AS base

RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    wget \
    xz-utils \
    ca-certificates \
    libpq-dev \
    gcc \
    unzip \
    && rm -rf /var/lib/apt/lists/*

ARG ZIG_VERSION=0.12.0
RUN curl -fsSL "https://ziglang.org/download/${ZIG_VERSION}/zig-linux-x86_64-${ZIG_VERSION}.tar.xz" \
    | tar -xJ -C /usr/local \
    && ln -s "/usr/local/zig-linux-x86_64-${ZIG_VERSION}/zig" /usr/local/bin/zig

ARG DAFNY_VERSION=4.4.0
RUN wget -q "https://github.com/dafny-lang/dafny/releases/download/v${DAFNY_VERSION}/dafny-${DAFNY_VERSION}-x64-ubuntu-20.04.zip" -O /tmp/dafny.zip \
    && unzip -q /tmp/dafny.zip -d /usr/local/dafny \
    && ln -s /usr/local/dafny/dafny/dafny /usr/local/bin/dafny \
    && rm /tmp/dafny.zip

ARG FSTAR_VERSION=2024.01.13
RUN wget -q "https://github.com/FStarLang/FStar/releases/download/v${FSTAR_VERSION}/fstar_${FSTAR_VERSION}_Linux_x86_64.tar.gz" -O /tmp/fstar.tar.gz \
    && tar -xzf /tmp/fstar.tar.gz -C /usr/local \
    && find /usr/local -name "fstar.exe" -exec ln -s {} /usr/local/bin/fstar.exe \; \
    && rm /tmp/fstar.tar.gz

WORKDIR /app

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY . .

RUN useradd -m -u 1001 appuser && chown -R appuser:appuser /app
USER appuser

EXPOSE 8004
CMD ["uvicorn", "app.main:app", "--host", "0.0.0.0", "--port", "8004", "--workers", "2"]
