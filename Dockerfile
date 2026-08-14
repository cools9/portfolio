FROM rust:1 AS chef

RUN cargo install cargo-chef

WORKDIR /app


FROM chef AS planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN curl -L --proto '=https' --tlsv1.2 -sSf \
    https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh \
    | bash

RUN cargo binstall dioxus-cli --root /.cargo -y --force

ENV PATH="/.cargo/bin:$PATH"

RUN dx bundle --web --release


FROM nginx:alpine AS runtime

COPY --from=builder \
    /app/target/dx/portfolio/release/web/public/ \
    /usr/share/nginx/html/

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]