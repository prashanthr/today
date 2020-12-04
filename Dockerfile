FROM rust:latest AS build
# Labels & metadata
LABEL version="latest"
LABEL name="today"
LABEL description="image for the today app"
LABEL maintainer="Prashanth R <https://github.com/prashanthr>"
# Set env
ENV APP_ENV=production
ENV PORT=9000
ENV APP_NAME=today
ENV WORK_DIR=/var/www/deploy/app/${APP_NAME}
# Workdir
RUN mkdir -p ${WORK_DIR}
WORKDIR ${WORK_DIR}
# App setup
COPY Cargo.toml Cargo.lock ./
COPY src ./src
# Build and install the application
RUN cargo build --release
RUN cargo install --path .
USER 1000
# Run application
CMD ["./target/release/today"]
EXPOSE ${PORT}
